use super::*;

pub struct DebSubstituter<'a> {
    new_exprs: ReverseExprSlice<'a>,
}

pub struct ReverseExprSlice<'a> {
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
            vcon_defs: self.substitute_and_downshift_vcon_defs_with_cutoff(
                original.vcon_defs.clone(),
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
        let shifted: Vec<VconDef> = original
            .value
            .iter()
            .map(|def| self.substitute_and_downshift_vcon_def_with_cutoff(def.clone(), cutoff))
            .collect();
        Rc::new(Hashed::new(shifted.into_boxed_slice()))
    }

    fn substitute_and_downshift_vcon_def_with_cutoff(
        &self,
        original: VconDef,
        cutoff: usize,
    ) -> VconDef {
        VconDef {
            param_types: self.substitute_and_downshift_expressions_with_increasing_cutoff(
                original.param_types.clone(),
                cutoff,
            ),
            index_args: self.substitute_and_downshift_expressions_with_constant_cutoff(
                original.index_args.clone(),
                cutoff + original.param_types.value.len(),
            ),
        }
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
        let shifted: Vec<MatchCase> = original
            .value
            .iter()
            .map(|case| self.substitute_and_downshift_match_case_with_cutoff(case.clone(), cutoff))
            .collect();
        Rc::new(Hashed::new(shifted.into_boxed_slice()))
    }

    fn substitute_and_downshift_match_case_with_cutoff(
        &self,
        original: MatchCase,
        cutoff: usize,
    ) -> MatchCase {
        MatchCase {
            arity: original.arity,
            return_val: self
                .substitute_and_downshift_with_cutoff(original.return_val, cutoff + original.arity),
        }
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
