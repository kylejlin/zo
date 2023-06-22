// TODO: Track `original`, or delete it.
//
// TODO: Make `MatchCase` and `VariantConstructorDef`
// not hashed. It's not worth the trouble.
// At least for `VariantConstructorDef`.
//
// TODO: Rename `VariantConstructorDef` to `VconDef`.

use std::rc::Rc;

use crate::{ast::*, nohash_hashmap::NoHashHashMap};

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
    pub eval_vcon_def_cache: NoHashHashMap<Digest, Result<Normalized<RcVconDef>, EvalError>>,
    pub eval_vcon_defs_cache: NoHashHashMap<Digest, Result<Normalized<RcVconDefs>, EvalError>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self::default()
    }
}

type RcHashed<T> = Rc<Hashed<T>>;
type RcExprs = RcHashed<Box<[Expr]>>;
type RcVconDef = RcHashed<VariantConstructorDef>;
type RcVconDefs = RcHashed<Box<[RcVconDef]>>;
type RcMatchCase = RcHashed<MatchCase>;
type RcMatchCases = RcHashed<Box<[RcMatchCase]>>;

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
            constructor_defs: self
                .eval_vcon_defs(ind.constructor_defs.clone())
                .map(Normalized::into_raw)?,
            original: None,
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
        let normalized: Vec<RcVconDef> = defs
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

    fn eval_vcon_def(&mut self, def: RcVconDef) -> Result<Normalized<RcVconDef>, EvalError> {
        if let Some(result) = self.eval_vcon_def_cache.get(&def.digest) {
            result.clone()
        } else {
            self.eval_unseen_vcon_def(def)
        }
    }

    fn eval_unseen_vcon_def(&mut self, def: RcVconDef) -> Result<Normalized<RcVconDef>, EvalError> {
        let def_digest = def.digest.clone();
        let def = &def.value;
        let normalized = VariantConstructorDef {
            param_types: self.eval_expressions(def.param_types.clone())?.into_raw(),
            index_args: self.eval_expressions(def.index_args.clone())?.into_raw(),
            original: None,
        };

        let result = Ok(Normalized(Rc::new(Hashed::new(normalized))));
        self.eval_vcon_def_cache.insert(def_digest, result.clone());
        result
    }

    fn eval_unseen_vcon(&mut self, vcon: RcHashed<Vcon>) -> Result<NormalForm, EvalError> {
        let vcon_digest = vcon.digest.clone();
        let vcon = &vcon.value;
        let normalized = Vcon {
            ind: self.eval_ind(vcon.ind.clone())?.into_raw(),
            vcon_index: vcon.vcon_index,
            original: None,
        };

        let result = Ok(Normalized(Expr::Vcon(Rc::new(Hashed::new(normalized)))));
        self.eval_expr_cache.insert(vcon_digest, result.clone());
        result
    }

    fn eval_ind(&mut self, ind: RcHashed<Ind>) -> Result<Normalized<RcHashed<Ind>>, EvalError> {
        if let Some(result) = self.eval_expr_cache.get(&ind.digest) {
            result.clone().map(|expr| {
                Normalized(expr.into_raw().try_into_ind().expect(
                    "Impossible: An `ind` expression cannot evaluate into a non-`ind` expression.",
                ))
            })
        } else {
            self.eval_unseen_ind(ind).map(|expr| {
                Normalized(expr.into_raw().try_into_ind().expect(
                    "Impossible: An `ind` expression cannot evaluate into a non-`ind` expression.",
                ))
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

            let match_return_value = match_.cases.value[vcon_index].value.return_val.clone();
            return self.eval(match_return_value);
        }

        if let Expr::App(normalized_matchee) = &normalized_matchee {
            if let Expr::Vcon(vcon) = &normalized_matchee.value.callee {
                let vcon_index = vcon.value.vcon_index;
                if vcon_index >= match_.cases.value.len() {
                    return Err(EvalError::TooFewMatchCases(m));
                }

                let unsubstituted = match_.cases.value[vcon_index].value.return_val.clone();
                let substituted = self
                    .substitute_and_downshift(unsubstituted, &normalized_matchee.value.args.value);
                return self.eval(substituted);
            }
        }

        let match_digest = m.digest.clone();
        let normalized = Match {
            matchee: normalized_matchee,
            return_type: self.eval(match_.return_type.clone())?.into_raw(),
            cases: self.eval_match_cases(match_.cases.clone())?.into_raw(),
            original: None,
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
        let normalized: Vec<RcMatchCase> = cases
            .iter()
            .map(|original| {
                Ok(Rc::new(Hashed::new(MatchCase {
                    arity: original.value.arity,
                    return_val: self.eval(original.value.return_val.clone())?.into_raw(),
                    original: None,
                })))
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
            original: None,
        };

        let result = Ok(Normalized(Expr::Fun(Rc::new(Hashed::new(normalized)))));
        self.eval_expr_cache.insert(fun_digest, result.clone());
        result
    }

    fn eval_unseen_app(&mut self, app: RcHashed<App>) -> Result<NormalForm, EvalError> {
        let normalized_callee = self.eval(app.value.callee.clone())?.into_raw();
        let normalized_args = self.eval_expressions(app.value.args.clone())?.into_raw();

        if let Expr::Fun(callee) = &normalized_callee {
            let unsubstituted = callee.value.return_val.clone();
            let new_exprs: Vec<Expr> = std::iter::once(normalized_callee)
                .chain(normalized_args.value.iter().cloned())
                .collect();
            let substituted = self.substitute_and_downshift(unsubstituted, &new_exprs);
            return self.eval(substituted);
        }

        let app_digest = app.digest.clone();
        let normalized = App {
            callee: normalized_callee,
            args: normalized_args,
            original: None,
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
            original: None,
        };

        let result = Ok(Normalized(Expr::For(Rc::new(Hashed::new(normalized)))));
        self.eval_expr_cache.insert(for_digest, result.clone());
        result
    }

    fn substitute_and_downshift(&mut self, expr: Expr, new_exprs: &[Expr]) -> Expr {
        DebSubstituter::new(new_exprs).substitute_and_downshift(expr)
    }
}

struct DebSubstituter<'a> {
    new_exprs: &'a [Expr],
}

impl<'a> DebSubstituter<'a> {
    pub fn new(new_exprs: &'a [Expr]) -> Self {
        Self { new_exprs }
    }
}

impl DebSubstituter<'_> {
    pub fn substitute_and_downshift(&self, expr: Expr) -> Expr {
        self.substitute_and_downshift_with_cutoff(expr, 0)
    }

    fn substitute_and_downshift_with_cutoff(&self, original: Expr, cutoff: usize) -> Expr {
        match original {
            Expr::Ind(o) => Expr::Ind(self.substitute_and_downshift_ind_with_cutoff(o, cutoff)),
            Expr::Vcon(o) => Expr::Vcon(self.substitute_and_downshift_vcon_with_cutoff(o, cutoff)),
            Expr::Match(o) => {
                Expr::Match(self.substitute_and_downshift_match_with_cutoff(o, cutoff))
            }
            Expr::Fun(o) => Expr::Fun(self.substitute_and_downshift_fun_with_cutoff(o, cutoff)),
            Expr::App(o) => Expr::App(self.substitute_and_downshift_app_with_cutoff(o, cutoff)),
            Expr::For(o) => Expr::For(self.substitute_and_downshift_for_with_cutoff(o, cutoff)),
            Expr::Deb(o) => self.substitute_and_downshift_deb_with_cutoff(o, cutoff),
            Expr::Universe(_) => original,
        }
    }

    fn substitute_and_downshift_ind_with_cutoff(
        &self,
        original: RcHashed<Ind>,
        cutoff: usize,
    ) -> RcHashed<Ind> {
        let original = &original.value;
        Rc::new(Hashed::new(Ind {
            name: original.name.clone(),
            universe_level: original.universe_level,
            index_types: self.substitute_and_downshift_expressions_with_increasing_cutoff(
                original.index_types.clone(),
                cutoff,
            ),
            constructor_defs: self.substitute_and_downshift_vcon_defs_with_cutoff(
                original.constructor_defs.clone(),
                cutoff + 1,
            ),
            original: None,
        }))
    }

    fn substitute_and_downshift_expressions_with_increasing_cutoff(
        &self,
        original: RcExprs,
        starting_cutoff: usize,
    ) -> RcExprs {
        let shifted: Vec<Expr> = original
            .value
            .iter()
            .enumerate()
            .map(|(expr_index, expr)| {
                self.substitute_and_downshift_with_cutoff(
                    expr.clone(),
                    starting_cutoff + expr_index,
                )
            })
            .collect();
        Rc::new(Hashed::new(shifted.into_boxed_slice()))
    }

    fn substitute_and_downshift_vcon_defs_with_cutoff(
        &self,
        original: RcVconDefs,
        cutoff: usize,
    ) -> RcVconDefs {
        let shifted: Vec<RcHashed<VariantConstructorDef>> = original
            .value
            .iter()
            .map(|def| self.substitute_and_downshift_vcon_def_with_cutoff(def.clone(), cutoff))
            .collect();
        Rc::new(Hashed::new(shifted.into_boxed_slice()))
    }

    fn substitute_and_downshift_vcon_def_with_cutoff(
        &self,
        original: RcHashed<VariantConstructorDef>,
        cutoff: usize,
    ) -> RcHashed<VariantConstructorDef> {
        let original = &original.value;
        Rc::new(Hashed::new(VariantConstructorDef {
            param_types: self.substitute_and_downshift_expressions_with_increasing_cutoff(
                original.param_types.clone(),
                cutoff,
            ),
            index_args: self.substitute_and_downshift_expressions_with_constant_cutoff(
                original.index_args.clone(),
                cutoff + original.param_types.value.len(),
            ),
            original: None,
        }))
    }

    fn substitute_and_downshift_vcon_with_cutoff(
        &self,
        original: RcHashed<Vcon>,
        cutoff: usize,
    ) -> RcHashed<Vcon> {
        let original = &original.value;
        Rc::new(Hashed::new(Vcon {
            ind: self.substitute_and_downshift_ind_with_cutoff(original.ind.clone(), cutoff),
            vcon_index: original.vcon_index,
            original: None,
        }))
    }

    fn substitute_and_downshift_match_with_cutoff(
        &self,
        original: RcHashed<Match>,
        cutoff: usize,
    ) -> RcHashed<Match> {
        let original = &original.value;
        Rc::new(Hashed::new(Match {
            matchee: self.substitute_and_downshift_with_cutoff(original.matchee.clone(), cutoff),
            return_type: self
                .substitute_and_downshift_with_cutoff(original.return_type.clone(), cutoff),
            cases: todo!(),
            original: None,
        }))
    }

    fn substitute_and_downshift_fun_with_cutoff(
        &self,
        original: RcHashed<Fun>,
        cutoff: usize,
    ) -> RcHashed<Fun> {
        let original = &original.value;
        Rc::new(Hashed::new(Fun {
            decreasing_index: original.decreasing_index,
            param_types: self.substitute_and_downshift_expressions_with_increasing_cutoff(
                original.param_types.clone(),
                cutoff,
            ),
            return_type: self.substitute_and_downshift_with_cutoff(
                original.return_type.clone(),
                cutoff + original.param_types.value.len(),
            ),
            return_val: self.substitute_and_downshift_with_cutoff(
                original.return_val.clone(),
                cutoff + original.param_types.value.len() + 1,
            ),
            original: None,
        }))
    }

    fn substitute_and_downshift_app_with_cutoff(
        &self,
        original: RcHashed<App>,
        cutoff: usize,
    ) -> RcHashed<App> {
        let original = &original.value;
        Rc::new(Hashed::new(App {
            callee: self.substitute_and_downshift_with_cutoff(original.callee.clone(), cutoff),
            args: self.substitute_and_downshift_expressions_with_constant_cutoff(
                original.args.clone(),
                cutoff,
            ),
            original: None,
        }))
    }

    fn substitute_and_downshift_expressions_with_constant_cutoff(
        &self,
        original: RcExprs,
        cutoff: usize,
    ) -> RcExprs {
        let shifted: Vec<Expr> = original
            .value
            .iter()
            .map(|expr| self.substitute_and_downshift_with_cutoff(expr.clone(), cutoff))
            .collect();
        Rc::new(Hashed::new(shifted.into_boxed_slice()))
    }

    fn substitute_and_downshift_for_with_cutoff(
        &self,
        original: RcHashed<For>,
        cutoff: usize,
    ) -> RcHashed<For> {
        let original = &original.value;
        Rc::new(Hashed::new(For {
            param_types: self.substitute_and_downshift_expressions_with_increasing_cutoff(
                original.param_types.clone(),
                cutoff,
            ),
            return_type: self.substitute_and_downshift_with_cutoff(
                original.return_type.clone(),
                cutoff + original.param_types.value.len(),
            ),
            original: None,
        }))
    }

    fn substitute_and_downshift_deb_with_cutoff(
        &self,
        original: RcHashed<Deb>,
        cutoff: usize,
    ) -> Expr {
        if original.value.0 < cutoff {
            return Expr::Deb(original);
        }

        let adjusted = original.value.0 - cutoff;
        if adjusted < self.new_exprs.len() {
            return self.new_exprs[adjusted].clone();
        }

        let shifted = original.value.0 - self.new_exprs.len();
        Expr::Deb(Rc::new(Hashed::new(Deb(shifted))))
    }
}
