use crate::syntax_tree::{ast::*, replace_debs::*};

#[cfg(test)]
mod tests;

mod normalized;
pub use normalized::*;

#[derive(Clone, Debug, Default)]
pub struct Evaluator {
    pub eval_expr_cache: NoHashHashMap<Digest, NormalForm>,
    pub eval_exprs_cache: NoHashHashMap<Digest, Normalized<RcExprs>>,
    pub eval_vcon_defs_cache: NoHashHashMap<Digest, Normalized<RcVconDefs>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self::default()
    }
}

type RcExprs = RcSemHashed<Vec<Expr>>;
type RcVconDefs = RcSemHashed<Vec<VconDef>>;
type RcMatchCases = RcSemHashed<Vec<MatchCase>>;

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

    fn eval_unseen_ind(&mut self, ind: RcSemHashed<Ind>) -> NormalForm {
        let ind_digest = ind.digest.clone();
        let ind = &ind.value;
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

    pub fn eval_expressions(&mut self, exprs: RcExprs) -> Normalized<RcExprs> {
        if let Some(result) = self.eval_exprs_cache.get(&exprs.digest) {
            result.clone()
        } else {
            self.eval_unseen_expressions(exprs)
        }
    }

    fn eval_unseen_expressions(&mut self, exprs: RcExprs) -> Normalized<RcExprs> {
        let exprs_digest = exprs.digest.clone();
        let exprs = &exprs.value;
        let normalized = exprs
            .iter()
            .map(|expr| self.eval(expr.clone()).into_raw())
            .collect::<Vec<_>>()
            .rc_hash_and_wrap_in_normalized();

        self.eval_exprs_cache
            .insert(exprs_digest, normalized.clone());
        normalized
    }

    fn eval_vcon_defs(&mut self, defs: RcVconDefs) -> Normalized<RcVconDefs> {
        if let Some(result) = self.eval_vcon_defs_cache.get(&defs.digest) {
            result.clone()
        } else {
            self.eval_unseen_vcon_defs(defs)
        }
    }

    fn eval_unseen_vcon_defs(&mut self, defs: RcVconDefs) -> Normalized<RcVconDefs> {
        let defs_digest = defs.digest.clone();
        let defs = &defs.value;
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

    fn eval_unseen_vcon(&mut self, vcon: RcSemHashed<Vcon>) -> NormalForm {
        let vcon_digest = vcon.digest.clone();
        let vcon = &vcon.value;
        let normalized = Vcon {
            ind: self.eval_ind(vcon.ind.clone()).into_raw(),
            vcon_index: vcon.vcon_index,
        }
        .convert_to_expr_and_wrap_in_normalized();

        self.eval_expr_cache.insert(vcon_digest, normalized.clone());
        normalized
    }

    pub fn eval_ind(&mut self, ind: RcSemHashed<Ind>) -> Normalized<RcSemHashed<Ind>> {
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

    fn eval_unseen_match(&mut self, m: RcSemHashed<Match>) -> NormalForm {
        let match_ = &m.value;
        let normalized_matchee = self.eval(match_.matchee.clone()).into_raw();

        if let Expr::Vcon(vcon) = &normalized_matchee {
            let vcon_index = vcon.value.vcon_index;
            if vcon_index >= match_.cases.value.len() {
                // This is a "stuck" term.
                // Since we don't emit errors, we just return it as-is.
                return m.convert_to_expr_and_wrap_in_normalized();
            }

            let match_return_value = match_.cases.value[vcon_index].return_val.clone();
            return self.eval(match_return_value);
        }

        if let Expr::App(normalized_matchee) = &normalized_matchee {
            if let Expr::Vcon(vcon) = &normalized_matchee.value.callee {
                let vcon_index = vcon.value.vcon_index;
                if vcon_index >= match_.cases.value.len() {
                    // This is a "stuck" term.
                    // Since we don't emit errors, we just return it as-is.
                    return m.convert_to_expr_and_wrap_in_normalized();
                }

                let unsubstituted = match_.cases.value[vcon_index].return_val.clone();
                let substituted = self.substitute_and_downshift_debs(
                    unsubstituted,
                    &normalized_matchee.value.args.value,
                );
                return self.eval(substituted);
            }
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

    fn eval_match_cases(&mut self, cases: RcMatchCases) -> Normalized<RcMatchCases> {
        // It's not worth caching match cases,
        // so we'll just re-evaluate them every time.
        // This isn't actually that expensive,
        // since the underlying expressions will be cached.
        self.eval_unseen_match_cases(cases)
    }

    fn eval_unseen_match_cases(&mut self, cases: RcMatchCases) -> Normalized<RcMatchCases> {
        let cases = &cases.value;
        cases
            .iter()
            .map(|original| MatchCase {
                arity: original.arity,
                return_val: self.eval(original.return_val.clone()).into_raw(),
            })
            .collect::<Vec<_>>()
            .rc_hash_and_wrap_in_normalized()
    }

    fn eval_unseen_fun(&mut self, fun: RcSemHashed<Fun>) -> NormalForm {
        let fun_digest = fun.digest.clone();
        let fun = &fun.value;
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

    fn eval_unseen_app(&mut self, app: RcSemHashed<App>) -> NormalForm {
        let normalized_callee = self.eval(app.value.callee.clone()).into_raw();
        let normalized_args = self.eval_expressions(app.value.args.clone()).into_raw();

        if let Expr::Fun(callee) = &normalized_callee {
            if can_unfold_app(callee.clone(), normalized_args.clone()) {
                let unsubstituted = callee.value.return_val.clone();
                let new_exprs: Vec<Expr> = normalized_args
                    .value
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

    fn eval_unseen_for(&mut self, for_: RcSemHashed<For>) -> NormalForm {
        let for_digest = for_.digest.clone();
        let for_ = &for_.value;
        let normalized = For {
            param_types: self.eval_expressions(for_.param_types.clone()).into_raw(),
            return_type: self.eval(for_.return_type.clone()).into_raw(),
        }
        .convert_to_expr_and_wrap_in_normalized();

        self.eval_expr_cache.insert(for_digest, normalized.clone());
        normalized
    }

    fn substitute_and_downshift_debs(&mut self, expr: Expr, new_exprs: &[Expr]) -> Expr {
        DebDownshiftSubstituter { new_exprs }.replace_debs(expr, 0)
    }
}

fn can_unfold_app(callee: RcSemHashed<Fun>, args: RcExprs) -> bool {
    let Some(decreasing_index) = callee.value.decreasing_index else {
        // If there is no decreasing param index,
        // the function is non-recursive.
        // We can always unfold non-recursive functions.
        return true;
    };

    let Some(decreasing_arg) = args.value.get(decreasing_index) else {
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
        Expr::App(app) => match &app.value.callee {
            Expr::Vcon(_) => true,
            _other_callee => false,
        },
        _ => false,
    }
}
