use crate::ast::*;

use std::rc::Rc;

type RcExprs = RcSemHashed<Box<[Expr]>>;
type RcVconDefs = RcSemHashed<Box<[VconDef]>>;
type RcMatchCases = RcSemHashed<Box<[MatchCase]>>;

/// Replaces `0` with the last element of in `new_exprs`,
/// `1` with the second to last element,
/// and so on.
/// Free variables that are not replaced by an element of
/// `new_exprs` will be downshifted by the length of `new_exprs`.
pub struct DebDownshiftSubstituter<'a> {
    pub new_exprs: &'a [Expr],
}

impl ReplaceDebs for DebDownshiftSubstituter<'_> {
    fn replace_deb(&self, original: RcSemHashed<DebNode>, cutoff: usize) -> Expr {
        if original.value.deb.0 < cutoff {
            return Expr::Deb(original);
        }

        let adjusted = original.value.deb.0 - cutoff;
        let new_exprs_len = self.new_exprs.len();
        if adjusted < new_exprs_len {
            let unshifted_new_expr = self.new_exprs[new_exprs_len - 1 - adjusted].clone();
            return DebUpshifter(cutoff).replace_debs(unshifted_new_expr, 0);
        }

        let shifted = Deb(original.value.deb.0 - new_exprs_len);
        Expr::Deb(Rc::new(Hashed::new(DebNode { deb: shifted })))
    }
}

pub struct DebUpshifter(pub usize);

impl ReplaceDebs for DebUpshifter {
    fn replace_deb(&self, original: RcSemHashed<DebNode>, cutoff: usize) -> Expr {
        if original.value.deb.0 < cutoff {
            return Expr::Deb(original);
        }

        Expr::Deb(Rc::new(Hashed::new(DebNode {
            deb: Deb(original.value.deb.0 + self.0),
        })))
    }
}

pub trait ReplaceDebs {
    fn replace_deb(&self, original: RcSemHashed<DebNode>, cutoff: usize) -> Expr;

    fn replace_debs(&self, original: Expr, cutoff: usize) -> Expr {
        match original {
            Expr::Ind(o) => Expr::Ind(self.replace_debs_in_ind(o, cutoff)),
            Expr::Vcon(o) => Expr::Vcon(self.replace_debs_in_vcon(o, cutoff)),
            Expr::Match(o) => Expr::Match(self.replace_debs_in_match(o, cutoff)),
            Expr::Fun(o) => Expr::Fun(self.replace_debs_in_fun(o, cutoff)),
            Expr::App(o) => Expr::App(self.replace_debs_in_app(o, cutoff)),
            Expr::For(o) => Expr::For(self.replace_debs_in_for(o, cutoff)),
            Expr::Deb(o) => self.replace_deb(o, cutoff),
            Expr::Universe(_) => original,
        }
    }

    fn replace_debs_in_ind(&self, original: RcSemHashed<Ind>, cutoff: usize) -> RcSemHashed<Ind> {
        let original = &original.value;
        Rc::new(Hashed::new(Ind {
            name: original.name.clone(),
            universe_level: original.universe_level,
            index_types: self.replace_debs_in_expressions_with_increasing_cutoff(
                original.index_types.clone(),
                cutoff,
            ),
            vcon_defs: self.replace_debs_in_vcon_defs(original.vcon_defs.clone(), cutoff + 1),
        }))
    }

    fn replace_debs_in_expressions_with_increasing_cutoff(
        &self,
        original: RcExprs,
        starting_cutoff: usize,
    ) -> RcExprs {
        let shifted: Vec<Expr> = original
            .value
            .iter()
            .enumerate()
            .map(|(expr_index, expr)| self.replace_debs(expr.clone(), starting_cutoff + expr_index))
            .collect();
        Rc::new(Hashed::new(shifted.into_boxed_slice()))
    }

    fn replace_debs_in_vcon_defs(&self, original: RcVconDefs, cutoff: usize) -> RcVconDefs {
        let shifted: Vec<VconDef> = original
            .value
            .iter()
            .map(|def| self.replace_debs_in_vcon_def(def.clone(), cutoff))
            .collect();
        Rc::new(Hashed::new(shifted.into_boxed_slice()))
    }

    fn replace_debs_in_vcon_def(&self, original: VconDef, cutoff: usize) -> VconDef {
        VconDef {
            param_types: self.replace_debs_in_expressions_with_increasing_cutoff(
                original.param_types.clone(),
                cutoff,
            ),
            index_args: self.replace_debs_in_expressions_with_constant_cutoff(
                original.index_args.clone(),
                cutoff + original.param_types.value.len(),
            ),
        }
    }

    fn replace_debs_in_vcon(
        &self,
        original: RcSemHashed<Vcon>,
        cutoff: usize,
    ) -> RcSemHashed<Vcon> {
        let original = &original.value;
        Rc::new(Hashed::new(Vcon {
            ind: self.replace_debs_in_ind(original.ind.clone(), cutoff),
            vcon_index: original.vcon_index,
        }))
    }

    fn replace_debs_in_match(
        &self,
        original: RcSemHashed<Match>,
        cutoff: usize,
    ) -> RcSemHashed<Match> {
        let original = &original.value;
        Rc::new(Hashed::new(Match {
            matchee: self.replace_debs(original.matchee.clone(), cutoff),
            return_type: self.replace_debs(original.return_type.clone(), cutoff),
            cases: self.replace_debs_in_match_cases(original.cases.clone(), cutoff),
        }))
    }

    fn replace_debs_in_match_cases(&self, original: RcMatchCases, cutoff: usize) -> RcMatchCases {
        let shifted: Vec<MatchCase> = original
            .value
            .iter()
            .map(|case| self.replace_debs_in_match_case(case.clone(), cutoff))
            .collect();
        Rc::new(Hashed::new(shifted.into_boxed_slice()))
    }

    fn replace_debs_in_match_case(&self, original: MatchCase, cutoff: usize) -> MatchCase {
        MatchCase {
            arity: original.arity,
            return_val: self.replace_debs(original.return_val, cutoff + original.arity),
        }
    }

    fn replace_debs_in_fun(&self, original: RcSemHashed<Fun>, cutoff: usize) -> RcSemHashed<Fun> {
        let original = &original.value;
        Rc::new(Hashed::new(Fun {
            decreasing_index: original.decreasing_index,
            param_types: self.replace_debs_in_expressions_with_increasing_cutoff(
                original.param_types.clone(),
                cutoff,
            ),
            return_type: self.replace_debs(
                original.return_type.clone(),
                cutoff + original.param_types.value.len(),
            ),
            return_val: self.replace_debs(
                original.return_val.clone(),
                cutoff + original.param_types.value.len() + 1,
            ),
        }))
    }

    fn replace_debs_in_app(&self, original: RcSemHashed<App>, cutoff: usize) -> RcSemHashed<App> {
        let original = &original.value;
        Rc::new(Hashed::new(App {
            callee: self.replace_debs(original.callee.clone(), cutoff),
            args: self
                .replace_debs_in_expressions_with_constant_cutoff(original.args.clone(), cutoff),
        }))
    }

    fn replace_debs_in_expressions_with_constant_cutoff(
        &self,
        original: RcExprs,
        cutoff: usize,
    ) -> RcExprs {
        let shifted: Vec<Expr> = original
            .value
            .iter()
            .map(|expr| self.replace_debs(expr.clone(), cutoff))
            .collect();
        Rc::new(Hashed::new(shifted.into_boxed_slice()))
    }

    fn replace_debs_in_for(&self, original: RcSemHashed<For>, cutoff: usize) -> RcSemHashed<For> {
        let original = &original.value;
        Rc::new(Hashed::new(For {
            param_types: self.replace_debs_in_expressions_with_increasing_cutoff(
                original.param_types.clone(),
                cutoff,
            ),
            return_type: self.replace_debs(
                original.return_type.clone(),
                cutoff + original.param_types.value.len(),
            ),
        }))
    }
}
