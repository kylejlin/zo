use crate::syntax_tree::{ast::*, replace_debs::*};

#[cfg(test)]
mod tests;

mod normalized;
pub use normalized::*;

#[derive(Clone, Debug, Default)]
pub struct Evaluator {
    pub eval_expr_cache: NoHashHashMap<Digest, NormalForm>,
    pub eval_exprs_cache: NoHashHashMap<Digest, Normalized<RcHashedVec<Expr>>>,
    pub eval_vcon_defs_cache: NoHashHashMap<Digest, Normalized<RcHashedVec<VconDef>>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Evaluator {
    pub fn eval(&mut self, expr: Expr) -> NormalForm {
        if let Some(result) = self.eval_expr_cache.get(&expr.digest()) {
            result.clone()
        } else {
            self.eval_unseen_expr(expr)
        }
    }

    fn eval_unseen_expr(&mut self, expr: Expr) -> NormalForm {
        match expr {
            Expr::Ind(e) => self.eval_unseen_ind(e),
            Expr::Vcon(e) => self.eval_unseen_vcon(e),
            Expr::Match(e) => self.eval_unseen_match(e),
            Expr::Fun(e) => self.eval_unseen_fun(e),
            Expr::App(e) => self.eval_unseen_app(e),
            Expr::For(e) => self.eval_unseen_for(e),

            Expr::Deb(_) | Expr::Universe(_) => Normalized(expr),
        }
    }

    fn eval_unseen_ind(&mut self, ind: RcHashed<Ind>) -> NormalForm {
        let ind_digest = ind.digest.clone();
        let ind = &ind.hashee;
        let normalized = Ind {
            name: ind.name.clone(),
            universe_level: ind.universe_level,
            index_types: self.eval_expressions(ind.index_types.clone()).into_raw(),
            vcon_defs: self.eval_vcon_defs(ind.vcon_defs.clone()).into_raw(),
        }
        .convert_to_expr_and_wrap_in_normalized();

        self.eval_expr_cache.insert(ind_digest, normalized.clone());
        normalized
    }

    pub fn eval_expressions(&mut self, exprs: RcHashedVec<Expr>) -> Normalized<RcHashedVec<Expr>> {
        if let Some(result) = self.eval_exprs_cache.get(&exprs.digest) {
            result.clone()
        } else {
            self.eval_unseen_expressions(exprs)
        }
    }

    fn eval_unseen_expressions(
        &mut self,
        exprs: RcHashedVec<Expr>,
    ) -> Normalized<RcHashedVec<Expr>> {
        let exprs_digest = exprs.digest.clone();
        let exprs = &exprs.hashee;
        let normalized = exprs
            .iter()
            .map(|expr| self.eval(expr.clone()).into_raw())
            .collect::<Vec<_>>()
            .rc_hash_and_wrap_in_normalized();

        self.eval_exprs_cache
            .insert(exprs_digest, normalized.clone());
        normalized
    }

    fn eval_vcon_defs(&mut self, defs: RcHashedVec<VconDef>) -> Normalized<RcHashedVec<VconDef>> {
        if let Some(result) = self.eval_vcon_defs_cache.get(&defs.digest) {
            result.clone()
        } else {
            self.eval_unseen_vcon_defs(defs)
        }
    }

    fn eval_unseen_vcon_defs(
        &mut self,
        defs: RcHashedVec<VconDef>,
    ) -> Normalized<RcHashedVec<VconDef>> {
        let defs_digest = defs.digest.clone();
        let defs = &defs.hashee;
        let normalized = defs
            .iter()
            .map(|def| self.eval_vcon_def(def.clone()).into_raw())
            .collect::<Vec<_>>()
            .rc_hash_and_wrap_in_normalized();

        self.eval_vcon_defs_cache
            .insert(defs_digest, normalized.clone());
        normalized
    }

    fn eval_vcon_def(&mut self, def: VconDef) -> Normalized<VconDef> {
        // It's not worth caching vcon defs,
        // so we'll just reevaluate them every time.
        // This is not actually that expensive,
        // because the underlying expressions _are_ cached.
        self.eval_unseen_vcon_def(def)
    }

    fn eval_unseen_vcon_def(&mut self, def: VconDef) -> Normalized<VconDef> {
        Normalized(VconDef {
            param_types: self.eval_expressions(def.param_types.clone()).into_raw(),
            index_args: self.eval_expressions(def.index_args.clone()).into_raw(),
        })
    }

    fn eval_unseen_vcon(&mut self, vcon: RcHashed<Vcon>) -> NormalForm {
        let vcon_digest = vcon.digest.clone();
        let vcon = &vcon.hashee;
        let normalized = Vcon {
            ind: self.eval_ind(vcon.ind.clone()).into_raw(),
            vcon_index: vcon.vcon_index,
        }
        .convert_to_expr_and_wrap_in_normalized();

        self.eval_expr_cache.insert(vcon_digest, normalized.clone());
        normalized
    }

    pub fn eval_ind(&mut self, ind: RcHashed<Ind>) -> Normalized<RcHashed<Ind>> {
        if let Some(result) = self.eval_expr_cache.get(&ind.digest) {
            Normalized(
                result
                    .clone()
                    .into_raw()
                    .try_into_ind()
                    .expect("Evaluating an ind should always return an ind"),
            )
        } else {
            Normalized(
                self.eval_unseen_ind(ind)
                    .clone()
                    .into_raw()
                    .try_into_ind()
                    .expect("Evaluating an ind should always return an ind"),
            )
        }
    }

    fn eval_unseen_match(&mut self, m: RcHashed<Match>) -> NormalForm {
        let match_ = &m.hashee;
        let normalized_matchee = self.eval(match_.matchee.clone()).into_raw();

        if let Some((vcon, args)) = self.try_as_vcon_capp(&normalized_matchee) {
            let vcon_index = vcon.hashee.vcon_index;
            if vcon_index >= match_.cases.hashee.len() {
                // The `match` expression does not have enough cases.
                // Therefore, it is a "stuck" term.
                // Since we don't emit errors, we just return the term as-is.
                return m.convert_to_expr_and_wrap_in_normalized();
            }

            let case = match &match_.cases.hashee[vcon_index] {
                MatchCase::Nondismissed(case) => case,
                MatchCase::Dismissed => {
                    // The match case corresponding to the matchee's vcon index
                    // is dismissed.
                    // Therefore, the `match` expression is a "stuck" term.
                    // Since we don't emit errors, we just return the term as-is.
                    return m.convert_to_expr_and_wrap_in_normalized();
                }
            };

            // TODO: Fix downshifting to account for `m.hashee.arity`.
            let unsubstituted = case.return_val.clone();
            let substituted = self.substitute_and_downshift_debs(unsubstituted, args);
            return self.eval(substituted);
        }

        let match_digest = m.digest.clone();
        let normalized = Match {
            matchee: normalized_matchee,
            return_type: self.eval(match_.return_type.clone()).into_raw(),
            cases: self.eval_match_cases(match_.cases.clone()).into_raw(),
        }
        .convert_to_expr_and_wrap_in_normalized();

        self.eval_expr_cache
            .insert(match_digest, normalized.clone());
        normalized
    }

    fn try_as_vcon_capp<'a>(&mut self, expr: &'a Expr) -> Option<(RcHashed<Vcon>, &'a [Expr])> {
        if let Expr::Vcon(vcon) = &expr {
            return Some((vcon.clone(), &[]));
        }

        if let Expr::App(expr) = &expr {
            if let Expr::Vcon(vcon) = &expr.hashee.callee {
                return Some((vcon.clone(), &expr.hashee.args.hashee));
            }
        }

        None
    }

    fn eval_match_cases(
        &mut self,
        cases: RcHashedVec<MatchCase>,
    ) -> Normalized<RcHashedVec<MatchCase>> {
        // It's not worth caching match case vecs,
        // so we'll just re-evaluate them every time.
        // This isn't actually that expensive,
        // since the underlying case return val
        // expressions will be cached.
        self.eval_unseen_match_cases(cases)
    }

    fn eval_unseen_match_cases(
        &mut self,
        cases: RcHashedVec<MatchCase>,
    ) -> Normalized<RcHashedVec<MatchCase>> {
        let cases = &cases.hashee;
        cases
            .iter()
            .map(|original| self.eval_match_case(original).0)
            .collect::<Vec<_>>()
            .rc_hash_and_wrap_in_normalized()
    }

    fn eval_match_case(&mut self, case: &MatchCase) -> Normalized<MatchCase> {
        // It's not worth caching match cases,
        // so we'll just re-evaluate them every time.
        // This isn't actually that expensive,
        // since the underlying case return val
        // expressions will be cached.
        self.eval_unseen_match_case(case)
    }

    fn eval_unseen_match_case(&mut self, case: &MatchCase) -> Normalized<MatchCase> {
        match case {
            MatchCase::Dismissed => Normalized(MatchCase::Dismissed),

            MatchCase::Nondismissed(original) => {
                Normalized(MatchCase::Nondismissed(NondismissedMatchCase {
                    arity: original.arity,
                    return_val: self.eval(original.return_val.clone()).into_raw(),
                }))
            }
        }
    }

    fn eval_unseen_fun(&mut self, fun: RcHashed<Fun>) -> NormalForm {
        let fun_digest = fun.digest.clone();
        let fun = &fun.hashee;
        let normalized = Fun {
            decreasing_index: fun.decreasing_index,
            param_types: self.eval_expressions(fun.param_types.clone()).into_raw(),
            return_type: self.eval(fun.return_type.clone()).into_raw(),
            return_val: self.eval(fun.return_val.clone()).into_raw(),
        }
        .convert_to_expr_and_wrap_in_normalized();

        self.eval_expr_cache.insert(fun_digest, normalized.clone());
        normalized
    }

    fn eval_unseen_app(&mut self, app: RcHashed<App>) -> NormalForm {
        let normalized_callee = self.eval(app.hashee.callee.clone()).into_raw();
        let normalized_args = self.eval_expressions(app.hashee.args.clone()).into_raw();

        if let Expr::Fun(callee) = &normalized_callee {
            if can_unfold_app(callee.clone(), normalized_args.clone()) {
                let unsubstituted = callee.hashee.return_val.clone();
                let new_exprs: Vec<Expr> = normalized_args
                    .hashee
                    .iter()
                    .cloned()
                    .chain(std::iter::once(normalized_callee))
                    .collect();
                let substituted = self.substitute_and_downshift_debs(unsubstituted, &new_exprs);
                return self.eval(substituted);
            }
        }

        let app_digest = app.digest.clone();
        let normalized = App {
            callee: normalized_callee,
            args: normalized_args,
        }
        .convert_to_expr_and_wrap_in_normalized();

        self.eval_expr_cache.insert(app_digest, normalized.clone());
        normalized
    }

    fn eval_unseen_for(&mut self, for_: RcHashed<For>) -> NormalForm {
        let for_digest = for_.digest.clone();
        let for_ = &for_.hashee;
        let normalized = For {
            param_types: self.eval_expressions(for_.param_types.clone()).into_raw(),
            return_type: self.eval(for_.return_type.clone()).into_raw(),
        }
        .convert_to_expr_and_wrap_in_normalized();

        self.eval_expr_cache.insert(for_digest, normalized.clone());
        normalized
    }

    fn substitute_and_downshift_debs(&mut self, expr: Expr, new_exprs: &[Expr]) -> Expr {
        expr.replace_debs(&DebDownshiftSubstituter { new_exprs }, 0)
    }
}

fn can_unfold_app(callee: RcHashed<Fun>, args: RcHashedVec<Expr>) -> bool {
    let Some(decreasing_index) = callee.hashee.decreasing_index else {
        // If there is no decreasing param index,
        // the function is non-recursive.
        // We can always unfold non-recursive functions.
        return true;
    };

    let Some(decreasing_arg) = args.hashee.get(decreasing_index) else {
        // If there is no argument at the decreasing index,
        // the application is ill-typed.
        // So, we do not unfold, in order to minimize
        // the chance of infinite loops.
        return false;
    };

    is_vconlike(decreasing_arg.clone())
}

fn is_vconlike(expr: Expr) -> bool {
    match expr {
        Expr::Vcon(_) => true,
        Expr::App(app) => match &app.hashee.callee {
            Expr::Vcon(_) => true,
            _other_callee => false,
        },
        _ => false,
    }
}
