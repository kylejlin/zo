// TODO: Track `original`.

use std::rc::Rc;

use crate::{ast::*, nohash_hashmap::NoHashHashMap};

#[cfg(test)]
mod tests;

mod replace_debs;

use replace_debs::*;

#[derive(Clone, Debug)]
pub enum EvalError {
    TooFewMatchCases(RcHashed<Match>),
}

#[derive(Clone, Debug)]
pub struct Normalized<T>(T);

pub type NormalForm = Normalized<Expr>;

impl From<NormalForm> for Expr {
    fn from(nf: NormalForm) -> Self {
        nf.0
    }
}

impl<T> Normalized<T> {
    pub fn into_raw(self) -> T {
        self.0
    }
}

#[derive(Clone, Debug, Default)]
pub struct Evaluator {
    pub eval_expr_cache: NoHashHashMap<Digest, Result<NormalForm, EvalError>>,
    pub eval_exprs_cache: NoHashHashMap<Digest, Result<Normalized<RcExprs>, EvalError>>,
    pub eval_vcon_defs_cache: NoHashHashMap<Digest, Result<Normalized<RcVconDefs>, EvalError>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self::default()
    }
}

type RcHashed<T> = Rc<Hashed<T>>;
type RcExprs = RcHashed<Box<[Expr]>>;
type RcVconDefs = RcHashed<Box<[VconDef]>>;
type RcMatchCases = RcHashed<Box<[MatchCase]>>;

impl Evaluator {
    pub fn eval(&mut self, expr: Expr) -> Result<NormalForm, EvalError> {
        if let Some(result) = self.eval_expr_cache.get(&expr.digest()) {
            result.clone()
        } else {
            self.eval_unseen_expr(expr)
        }
    }

    fn eval_unseen_expr(&mut self, expr: Expr) -> Result<NormalForm, EvalError> {
        match expr {
            Expr::Ind(e) => self.eval_unseen_ind(e),
            Expr::Vcon(e) => self.eval_unseen_vcon(e),
            Expr::Match(e) => self.eval_unseen_match(e),
            Expr::Fun(e) => self.eval_unseen_fun(e),
            Expr::App(e) => self.eval_unseen_app(e),
            Expr::For(e) => self.eval_unseen_for(e),

            Expr::Deb(_) | Expr::Universe(_) => Ok(Normalized(expr)),
        }
    }

    fn eval_unseen_ind(&mut self, ind: RcHashed<Ind>) -> Result<NormalForm, EvalError> {
        let ind_digest = ind.digest.clone();
        let ind = &ind.value;
        let normalized = Ind {
            name: ind.name.clone(),
            universe_level: ind.universe_level,
            index_types: self
                .eval_expressions(ind.index_types.clone())
                .map(Normalized::into_raw)?,
            vcon_defs: self
                .eval_vcon_defs(ind.vcon_defs.clone())
                .map(Normalized::into_raw)?,
        };

        let result = Ok(Normalized(Expr::Ind(Rc::new(Hashed::new(normalized)))));
        self.eval_expr_cache.insert(ind_digest, result.clone());
        result
    }

    fn eval_expressions(&mut self, exprs: RcExprs) -> Result<Normalized<RcExprs>, EvalError> {
        if let Some(result) = self.eval_exprs_cache.get(&exprs.digest) {
            result.clone()
        } else {
            self.eval_unseen_expressions(exprs)
        }
    }

    fn eval_unseen_expressions(
        &mut self,
        exprs: RcExprs,
    ) -> Result<Normalized<RcExprs>, EvalError> {
        let exprs_digest = exprs.digest.clone();
        let exprs = &exprs.value;
        let normalized: Vec<Expr> = exprs
            .iter()
            .map(|expr| self.eval(expr.clone()).map(Normalized::into_raw))
            .collect::<Result<Vec<_>, _>>()?;

        let result = Ok(Normalized(Rc::new(Hashed::new(
            normalized.into_boxed_slice(),
        ))));
        self.eval_exprs_cache.insert(exprs_digest, result.clone());
        result
    }

    fn eval_vcon_defs(&mut self, defs: RcVconDefs) -> Result<Normalized<RcVconDefs>, EvalError> {
        if let Some(result) = self.eval_vcon_defs_cache.get(&defs.digest) {
            result.clone()
        } else {
            self.eval_unseen_vcon_defs(defs)
        }
    }

    fn eval_unseen_vcon_defs(
        &mut self,
        defs: RcVconDefs,
    ) -> Result<Normalized<RcVconDefs>, EvalError> {
        let defs_digest = defs.digest.clone();
        let defs = &defs.value;
        let normalized: Vec<VconDef> = defs
            .iter()
            .map(|def| self.eval_vcon_def(def.clone()).map(Normalized::into_raw))
            .collect::<Result<Vec<_>, _>>()?;

        let result = Ok(Normalized(Rc::new(Hashed::new(
            normalized.into_boxed_slice(),
        ))));
        self.eval_vcon_defs_cache
            .insert(defs_digest, result.clone());
        result
    }

    fn eval_vcon_def(&mut self, def: VconDef) -> Result<Normalized<VconDef>, EvalError> {
        // It's not worth caching vcon defs,
        // so we'll just reevaluate them every time.
        // This is not actually that expensive,
        // because the underlying expressions _are_ cached.
        self.eval_unseen_vcon_def(def)
    }

    fn eval_unseen_vcon_def(&mut self, def: VconDef) -> Result<Normalized<VconDef>, EvalError> {
        let normalized = VconDef {
            param_types: self.eval_expressions(def.param_types.clone())?.into_raw(),
            index_args: self.eval_expressions(def.index_args.clone())?.into_raw(),
        };

        Ok(Normalized(normalized))
    }

    fn eval_unseen_vcon(&mut self, vcon: RcHashed<Vcon>) -> Result<NormalForm, EvalError> {
        let vcon_digest = vcon.digest.clone();
        let vcon = &vcon.value;
        let normalized = Vcon {
            ind: self.eval_ind(vcon.ind.clone())?.into_raw(),
            vcon_index: vcon.vcon_index,
        };

        let result = Ok(Normalized(Expr::Vcon(Rc::new(Hashed::new(normalized)))));
        self.eval_expr_cache.insert(vcon_digest, result.clone());
        result
    }

    fn eval_ind(&mut self, ind: RcHashed<Ind>) -> Result<Normalized<RcHashed<Ind>>, EvalError> {
        if let Some(result) = self.eval_expr_cache.get(&ind.digest) {
            result.clone().map(|nf| match nf.into_raw() {
                Expr::Ind(ind) => Normalized(ind),
                _ => unreachable!(),
            })
        } else {
            self.eval_unseen_ind(ind).map(|nf| match nf.into_raw() {
                Expr::Ind(ind) => Normalized(ind),
                _ => unreachable!(),
            })
        }
    }

    fn eval_unseen_match(&mut self, m: RcHashed<Match>) -> Result<NormalForm, EvalError> {
        let match_ = &m.value;
        let normalized_matchee = self.eval(match_.matchee.clone())?.into_raw();

        if let Expr::Vcon(vcon) = &normalized_matchee {
            let vcon_index = vcon.value.vcon_index;
            if vcon_index >= match_.cases.value.len() {
                return Err(EvalError::TooFewMatchCases(m));
            }

            let match_return_value = match_.cases.value[vcon_index].return_val.clone();
            return self.eval(match_return_value);
        }

        if let Expr::App(normalized_matchee) = &normalized_matchee {
            if let Expr::Vcon(vcon) = &normalized_matchee.value.callee {
                let vcon_index = vcon.value.vcon_index;
                if vcon_index >= match_.cases.value.len() {
                    return Err(EvalError::TooFewMatchCases(m));
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
            return_type: self.eval(match_.return_type.clone())?.into_raw(),
            cases: self.eval_match_cases(match_.cases.clone())?.into_raw(),
        };

        let result = Ok(Normalized(Expr::Match(Rc::new(Hashed::new(normalized)))));
        self.eval_expr_cache.insert(match_digest, result.clone());
        result
    }

    fn eval_match_cases(
        &mut self,
        cases: RcMatchCases,
    ) -> Result<Normalized<RcMatchCases>, EvalError> {
        // It's not worth caching match cases,
        // so we'll just re-evaluate them every time.
        // This isn't actually that expensive,
        // since the underlying expressions will be cached.
        self.eval_unseen_match_cases(cases)
    }

    fn eval_unseen_match_cases(
        &mut self,
        cases: RcMatchCases,
    ) -> Result<Normalized<RcMatchCases>, EvalError> {
        let cases = &cases.value;
        let normalized: Vec<MatchCase> = cases
            .iter()
            .map(|original| {
                Ok(MatchCase {
                    arity: original.arity,
                    return_val: self.eval(original.return_val.clone())?.into_raw(),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Normalized(Rc::new(Hashed::new(
            normalized.into_boxed_slice(),
        ))))
    }

    fn eval_unseen_fun(&mut self, fun: RcHashed<Fun>) -> Result<NormalForm, EvalError> {
        let fun_digest = fun.digest.clone();
        let fun = &fun.value;
        let normalized = Fun {
            decreasing_index: fun.decreasing_index,
            param_types: self.eval_expressions(fun.param_types.clone())?.into_raw(),
            return_type: self.eval(fun.return_type.clone())?.into_raw(),
            return_val: self.eval(fun.return_val.clone())?.into_raw(),
        };

        let result = Ok(Normalized(Expr::Fun(Rc::new(Hashed::new(normalized)))));
        self.eval_expr_cache.insert(fun_digest, result.clone());
        result
    }

    fn eval_unseen_app(&mut self, app: RcHashed<App>) -> Result<NormalForm, EvalError> {
        let normalized_callee = self.eval(app.value.callee.clone())?.into_raw();
        let normalized_args = self.eval_expressions(app.value.args.clone())?.into_raw();

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
        };

        let result = Ok(Normalized(Expr::App(Rc::new(Hashed::new(normalized)))));
        self.eval_expr_cache.insert(app_digest, result.clone());
        result
    }

    fn eval_unseen_for(&mut self, for_: RcHashed<For>) -> Result<NormalForm, EvalError> {
        let for_digest = for_.digest.clone();
        let for_ = &for_.value;
        let normalized = For {
            param_types: self.eval_expressions(for_.param_types.clone())?.into_raw(),
            return_type: self.eval(for_.return_type.clone())?.into_raw(),
        };

        let result = Ok(Normalized(Expr::For(Rc::new(Hashed::new(normalized)))));
        self.eval_expr_cache.insert(for_digest, result.clone());
        result
    }

    fn substitute_and_downshift_debs(&mut self, expr: Expr, new_exprs: &[Expr]) -> Expr {
        DebDownshiftSubstituter { new_exprs }.replace_debs(expr, 0)
    }
}

fn can_unfold_app(callee: RcHashed<Fun>, args: RcExprs) -> bool {
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
