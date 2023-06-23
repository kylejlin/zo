// TODO: Make `MatchCase` and `VariantConstructorDef`
// not hashed. It's not worth the trouble.
// At least for `MatchCase`.
//
// TODO: Rename `VariantConstructorDef` to `VconDef`.
//
// TODO: Reinstate the requirement that `Vcon.ind` is an `Ind`.
//
// TODO: Track `original`.

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
        };

        let result = Ok(Normalized(Rc::new(Hashed::new(normalized))));
        self.eval_vcon_def_cache.insert(def_digest, result.clone());
        result
    }

    fn eval_unseen_vcon(&mut self, vcon: RcHashed<Vcon>) -> Result<NormalForm, EvalError> {
        let vcon_digest = vcon.digest.clone();
        let vcon = &vcon.value;
        let normalized = Vcon {
            ind: self.eval(vcon.ind.clone())?.into_raw(),
            vcon_index: vcon.vcon_index,
        };

        let result = Ok(Normalized(Expr::Vcon(Rc::new(Hashed::new(normalized)))));
        self.eval_expr_cache.insert(vcon_digest, result.clone());
        result
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
                let substituted = self.substitute_and_downshift(
                    unsubstituted,
                    ReverseExprSlice {
                        unprocessed: &normalized_matchee.value.args.value,
                    },
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
        let normalized: Vec<RcMatchCase> = cases
            .iter()
            .map(|original| {
                Ok(Rc::new(Hashed::new(MatchCase {
                    arity: original.value.arity,
                    return_val: self.eval(original.value.return_val.clone())?.into_raw(),
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
                let substituted = self.substitute_and_downshift(
                    unsubstituted,
                    ReverseExprSlice {
                        unprocessed: &new_exprs,
                    },
                );
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

    fn substitute_and_downshift(&mut self, expr: Expr, new_exprs: ReverseExprSlice) -> Expr {
        DebSubstituter::new(new_exprs).substitute_and_downshift(expr)
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

struct DebSubstituter<'a> {
    new_exprs: ReverseExprSlice<'a>,
}

struct ReverseExprSlice<'a> {
    pub unprocessed: &'a [Expr],
}

impl<'a> DebSubstituter<'a> {
    pub fn new(new_exprs: ReverseExprSlice<'a>) -> Self {
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
        }))
    }

    fn substitute_and_downshift_vcon_with_cutoff(
        &self,
        original: RcHashed<Vcon>,
        cutoff: usize,
    ) -> RcHashed<Vcon> {
        let original = &original.value;
        Rc::new(Hashed::new(Vcon {
            ind: self.substitute_and_downshift_with_cutoff(original.ind.clone(), cutoff),
            vcon_index: original.vcon_index,
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
            cases: self
                .substitute_and_downshift_match_cases_with_cutoff(original.cases.clone(), cutoff),
        }))
    }

    fn substitute_and_downshift_match_cases_with_cutoff(
        &self,
        original: RcMatchCases,
        cutoff: usize,
    ) -> RcMatchCases {
        let shifted: Vec<RcHashed<MatchCase>> = original
            .value
            .iter()
            .map(|case| self.substitute_and_downshift_match_case_with_cutoff(case.clone(), cutoff))
            .collect();
        Rc::new(Hashed::new(shifted.into_boxed_slice()))
    }

    fn substitute_and_downshift_match_case_with_cutoff(
        &self,
        original: RcMatchCase,
        cutoff: usize,
    ) -> RcMatchCase {
        let original = &original.value;
        Rc::new(Hashed::new(MatchCase {
            arity: original.arity,
            return_val: self.substitute_and_downshift_with_cutoff(
                original.return_val.clone(),
                cutoff + original.arity,
            ),
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
        let new_exprs_len = self.new_exprs.unprocessed.len();
        if adjusted < new_exprs_len {
            return self.new_exprs.unprocessed[new_exprs_len - 1 - adjusted].clone();
        }

        let shifted = original.value.0 - new_exprs_len;
        Expr::Deb(Rc::new(Hashed::new(Deb(shifted))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn add_2_3() {
        let nat_def = (
            "<NAT>",
            r#"(ind Type0 "Nat" () (
(() ())
((0) ())
))"#,
        );
        let zero_def = ("<ZERO>", "(vcon <NAT> 0)");
        let succ_def = ("<SUCC>", "(vcon <NAT> 1)");
        let add_two_three_src = substitute_with_compounding(
            [
                nat_def,
                zero_def,
                succ_def,
                (
                    "<ADD>",
                    "(fun 0 (<NAT> <NAT>) Type0
(
    match 2 <NAT>

    (
        (0 1)

        (1 (1 0 (<SUCC> 2)))
    )
))",
                ),
                ("<2>", "(<SUCC> (<SUCC> <ZERO>))"),
                ("<3>", "(<SUCC> <2>)"),
            ],
            r#"(<ADD> <2> <3>)"#,
        );
        let five_src = substitute_with_compounding(
            [nat_def, zero_def, succ_def],
            "(<SUCC> (<SUCC> (<SUCC> (<SUCC> (<SUCC> <ZERO>)))))",
        );

        let actual = {
            let tokens = crate::lexer::lex(&add_two_three_src).unwrap();
            let cst = crate::parser::parse(tokens).unwrap();
            let ast: Expr = cst.into();
            Evaluator::default().eval(ast).unwrap().into_raw()
        };

        let expected = {
            let tokens = crate::lexer::lex(&five_src).unwrap();
            let cst = crate::parser::parse(tokens).unwrap();
            Expr::from(cst)
        };

        assert_eq!(expected.digest(), actual.digest());
    }

    #[test]
    fn nullary_match_case() {
        let dummy_ind_def = (
            "<DUMMY_IND>",
            r#"(ind Type0 "Dummy" () (
(() ())
((0) ())
((0 1) ())
))"#,
        );
        let match_src = substitute_with_compounding(
            [dummy_ind_def],
            r#"
(
    match (vcon <DUMMY_IND> 0) <DUMMY_IND> (
        (0 12)
        (1 14)
        (2 (16 1 0))
    )
)"#,
        );
        let expected_src = r#"12"#;

        let actual = {
            let tokens = crate::lexer::lex(&match_src).unwrap();
            let cst = crate::parser::parse(tokens).unwrap();
            let ast: Expr = cst.into();
            Evaluator::default().eval(ast).unwrap().into_raw()
        };

        let expected = {
            let tokens = crate::lexer::lex(&expected_src).unwrap();
            let cst = crate::parser::parse(tokens).unwrap();
            Expr::from(cst)
        };

        assert_eq!(expected.digest(), actual.digest());
    }

    #[test]
    fn match_case_param_substitution() {
        let dummy_ind_def = (
            "<DUMMY_IND>",
            r#"(ind Type0 "Dummy" () (
(() ())
((0) ())
((0 1) ())
))"#,
        );
        let match_src = substitute_with_compounding(
            [dummy_ind_def],
            r#"
(
    match ((vcon <DUMMY_IND> 2) 10 11) <DUMMY_IND> (
        (0 12)
        (1 14)
        (2 (16 1 0))
    )
)"#,
        );
        let expected_src = r#"(14 10 11)"#;

        let actual = {
            let tokens = crate::lexer::lex(&match_src).unwrap();
            let cst = crate::parser::parse(tokens).unwrap();
            let ast: Expr = cst.into();
            Evaluator::default().eval(ast).unwrap().into_raw()
        };

        let expected = {
            let tokens = crate::lexer::lex(&expected_src).unwrap();
            let cst = crate::parser::parse(tokens).unwrap();
            Expr::from(cst)
        };

        assert_eq!(expected.digest(), actual.digest());
    }

    #[test]
    fn rev_1_2_3() {
        let nat_def = (
            "<NAT>",
            r#"(ind Type0 "Nat" () (
    (() ())
    ((0) ())
))"#,
        );
        let zero_def = ("<ZERO>", "(vcon <NAT> 0)");
        let succ_def = ("<SUCC>", "(vcon <NAT> 1)");
        let one_def = ("<1>", "(<SUCC> <ZERO>)");
        let two_def = ("<2>", "(<SUCC> <1>)");
        let three_def = ("<3>", "(<SUCC> <2>)");
        let list_0_def = (
            "<LIST_0>",
            r#"(
            ind
    
            Type0
    
            "List"
    
            ()
    
            (
                // DB index stack is
                // 0 =>  List(T)
                // 1 => List 
                // 2 => T
    
                // nil
                (() ())
    
                // cons
                ((
                    2
    
                    // DB index stack is
                    // 0 => car
                    // 1 => List(T)
                    // 2 => List
                    // 3 => T
                    1
                ) ())
            )
        )"#,
        );
        let polymorphic_list_def = (
            "<POLYMORPHIC_LIST>",
            r#"(
    fun

    nonrec

    (Type0)

    Type0

    <LIST_0>
)"#,
        );
        let polymorphic_nil_def = (
            "<POLYMORPHIC_NIL>",
            r#"(
    fun

    nonrec

    (Type0)

    Type0

    (vcon <LIST_0> 0)
)"#,
        );
        let polymorphic_cons_def = (
            "<POLYMORPHIC_CONS>",
            r#"(
    fun

    nonrec

    (Type0)

    Type0

    (vcon <LIST_0> 1)
)"#,
        );
        let nat_nil_def = ("<NAT_NIL>", "(<POLYMORPHIC_NIL> <NAT>)");
        let cons_def = ("<NAT_CONS>", "(<POLYMORPHIC_CONS> <NAT>)");
        let normalized_nat_list_def = (
            "<NORMALIZED_NAT_LIST>",
            r#"(
            ind
    
            Type0
    
            "List"
    
            ()
    
            (
                // DB index stack is
                // 0 =>  List(Nat)
                // 1 => List 
    
                // nil
                (() ())
    
                // cons
                ((
                    <NAT>
    
                    // DB index stack is
                    // 0 => car
                    // 1 => List(Nat)
                    1
                ) ())
            )
        )"#,
        );
        let normalized_nat_nil_def = ("<NORMALIZED_NAT_NIL>", "(vcon <NORMALIZED_NAT_LIST> 0)");
        let normalized_nat_cons_def = ("<NORMALIZED_NAT_CONS>", "(vcon <NORMALIZED_NAT_LIST> 1)");
        let one_two_three_src = (
            "<123>",
            "(<NAT_CONS> <1> (<NAT_CONS> <2> (<NAT_CONS> <3> <NAT_NIL>)))",
        );
        let rev_src = (
            "<REV>",
            r#"(
    fun
    
    0
    
    (
        (<POLYMORPHIC_LIST> <NAT>) // reversee
        (<POLYMORPHIC_LIST> <NAT>) // out
    )
    
    (<POLYMORPHIC_LIST> <NAT>)
    
    (
        match 2 (<POLYMORPHIC_LIST> <NAT>)

        (
            (0 1)

            (2 
                // DB index stack
                // 0 => reversee.cdr
                // 1 => reversee.car
                // 2 => rev
                // 3 => out
                // 4 => reversee

                (2 0 (<NAT_CONS> 1 3))
            )
        )
    )
)"#,
        );
        let src_defs = [
            nat_def,
            zero_def,
            succ_def,
            one_def,
            two_def,
            three_def,
            list_0_def,
            polymorphic_list_def,
            polymorphic_nil_def,
            polymorphic_cons_def,
            nat_nil_def,
            cons_def,
            normalized_nat_list_def,
            normalized_nat_nil_def,
            normalized_nat_cons_def,
            one_two_three_src,
            rev_src,
        ];
        let rev_one_two_three_src =
            substitute_with_compounding(src_defs, r#"(<REV> <123> <NAT_NIL>)"#);
        let three_two_one_src =
            substitute_with_compounding(src_defs, "(<NORMALIZED_NAT_CONS> <3> (<NORMALIZED_NAT_CONS> <2> (<NORMALIZED_NAT_CONS> <1> <NORMALIZED_NAT_NIL>)))");

        let actual = {
            let tokens = crate::lexer::lex(&rev_one_two_three_src).unwrap();
            let cst = crate::parser::parse(tokens).unwrap();
            let ast: Expr = cst.into();
            Evaluator::default().eval(ast).unwrap().into_raw()
        };

        let expected = {
            let tokens = crate::lexer::lex(&three_two_one_src).unwrap();
            let cst = crate::parser::parse(tokens).unwrap();
            Expr::from(cst)
        };

        assert_eq!(expected.digest(), actual.digest());
    }

    #[test]
    fn recursive_fun_app_stops_unfolding_when_decreasing_arg_not_vconlike() {
        let nat_def = (
            "<NAT>",
            r#"(ind Type0 "Nat" () (
(() ())
((0) ())
))"#,
        );
        let zero_def = ("<ZERO>", "(vcon <NAT> 0)");
        let succ_def = ("<SUCC>", "(vcon <NAT> 1)");
        let recursive_identity_def = (
            "<RECURSIVE_IDENTITY>",
            r#"
(
    fun

    0

    (<NAT>)

    <NAT>

    (
        match 1 <NAT> (
            (0 <ZERO>)
            (1 (<SUCC> (1 0)))
        )
    )
)"#,
        );
        let defs = [nat_def, zero_def, succ_def, recursive_identity_def];
        let ident_succ_deb_123_src =
            substitute_with_compounding(defs, r#"(<RECURSIVE_IDENTITY> (<SUCC> 123))"#);
        let succ_ident_deb_123_src =
            substitute_with_compounding(defs, "(<SUCC> (<RECURSIVE_IDENTITY> 123))");

        let actual = {
            let tokens = crate::lexer::lex(&ident_succ_deb_123_src).unwrap();
            let cst = crate::parser::parse(tokens).unwrap();
            let ast: Expr = cst.into();
            Evaluator::default().eval(ast).unwrap().into_raw()
        };

        let expected = {
            let tokens = crate::lexer::lex(&succ_ident_deb_123_src).unwrap();
            let cst = crate::parser::parse(tokens).unwrap();
            Expr::from(cst)
        };

        assert_eq!(expected.digest(), actual.digest());
    }

    fn substitute_with_compounding<'a>(
        iter: impl IntoIterator<Item = (&'a str, &'a str)>,
        last: &'a str,
    ) -> String {
        let mut replacements = vec![];
        for (from, unreplaced_to) in iter {
            let to = substitute_without_compounding(&replacements, unreplaced_to);
            replacements.push((from, to));
        }
        substitute_without_compounding(&replacements, last)
    }

    fn substitute_without_compounding(replacements: &[(&str, String)], original: &str) -> String {
        let mut result = original.to_string();
        for (from, to) in replacements {
            result = result.replace(from, to);
        }
        result
    }
}
