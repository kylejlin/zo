pub trait IsStrictSubexpressionOf<Super: ?Sized> {
    fn is_strict_subexpression_of(&self, super_: &Super) -> bool;

    fn is_inclusive_subexpression_of(&self, super_: &Super) -> bool
    where
        Self: PartialEq<Super> + Eq,
    {
        self == super_ || self.is_strict_subexpression_of(super_)
    }
}

mod impl_ast {
    use super::*;

    use crate::syntax_tree::{
        ast::*,
        replace_debs::{DebUpshifter, ReplaceDebs},
    };

    impl IsStrictSubexpressionOf<Expr> for Expr {
        fn is_strict_subexpression_of(&self, super_: &Self) -> bool {
            match super_ {
                Expr::Ind(super_) => self.is_strict_subexpression_of(super_),
                Expr::Vcon(super_) => self.is_strict_subexpression_of(super_),
                Expr::Match(super_) => self.is_strict_subexpression_of(super_),
                Expr::Fun(super_) => self.is_strict_subexpression_of(super_),
                Expr::App(super_) => self.is_strict_subexpression_of(super_),
                Expr::For(super_) => self.is_strict_subexpression_of(super_),
                Expr::Deb(super_) => self.is_strict_subexpression_of(super_),
                Expr::Universe(super_) => self.is_strict_subexpression_of(super_),
            }
        }
    }

    impl IsStrictSubexpressionOf<RcSemHashed<Ind>> for Expr {
        fn is_strict_subexpression_of(&self, super_: &RcSemHashed<Ind>) -> bool {
            self.is_strict_subexpression_of_dependent_expression_slice(
                super_.hashee.index_types.hashee.as_slice(),
            ) || Expr::from(self.clone())
                .replace_debs(&DebUpshifter(1), 0)
                .is_strict_subexpression_of(super_.hashee.vcon_defs.hashee.as_slice())
        }
    }

    impl IsStrictSubexpressionOf<[VconDef]> for Expr {
        fn is_strict_subexpression_of(&self, super_: &[VconDef]) -> bool {
            super_
                .iter()
                .any(|super_def| self.is_strict_subexpression_of(super_def))
        }
    }

    impl IsStrictSubexpressionOf<VconDef> for Expr {
        fn is_strict_subexpression_of(&self, super_: &VconDef) -> bool {
            self.is_strict_subexpression_of_dependent_expression_slice(
                super_.param_types.hashee.as_slice(),
            ) || self
                .clone()
                .replace_debs(&DebUpshifter(super_.param_types.hashee.len()), 0)
                .is_strict_subexpression_of_independent_expression_slice(
                    super_.index_args.hashee.as_slice(),
                )
        }
    }

    impl IsStrictSubexpressionOf<RcSemHashed<Vcon>> for Expr {
        fn is_strict_subexpression_of(&self, super_: &RcSemHashed<Vcon>) -> bool {
            self.is_inclusive_subexpression_of(&Expr::from(super_.hashee.ind.clone()))
        }
    }

    impl IsStrictSubexpressionOf<RcSemHashed<Match>> for Expr {
        fn is_strict_subexpression_of(&self, super_: &RcSemHashed<Match>) -> bool {
            self.is_inclusive_subexpression_of(&super_.hashee.matchee)
                || self.is_inclusive_subexpression_of(&super_.hashee.return_type)
                || self.is_strict_subexpression_of(super_.hashee.cases.hashee.as_slice())
        }
    }

    impl IsStrictSubexpressionOf<[MatchCase]> for Expr {
        fn is_strict_subexpression_of(&self, super_: &[MatchCase]) -> bool {
            super_
                .iter()
                .any(|super_case| self.is_strict_subexpression_of(super_case))
        }
    }

    impl IsStrictSubexpressionOf<MatchCase> for Expr {
        fn is_strict_subexpression_of(&self, super_: &MatchCase) -> bool {
            match super_ {
                MatchCase::Dismissed => false,
                MatchCase::Nondismissed(super_nondismissed) => self
                    .clone()
                    .replace_debs(&DebUpshifter(super_nondismissed.arity), 0)
                    .is_inclusive_subexpression_of(&super_nondismissed.return_val),
            }
        }
    }

    impl IsStrictSubexpressionOf<RcSemHashed<Fun>> for Expr {
        fn is_strict_subexpression_of(&self, super_: &RcSemHashed<Fun>) -> bool {
            self.is_strict_subexpression_of_dependent_expression_slice(
                super_.hashee.param_types.hashee.as_slice(),
            ) || self
                .clone()
                .replace_debs(&DebUpshifter(super_.hashee.param_types.hashee.len()), 0)
                .is_inclusive_subexpression_of(&super_.hashee.return_type)
                || self
                    .clone()
                    .replace_debs(&DebUpshifter(super_.hashee.param_types.hashee.len() + 1), 0)
                    .is_inclusive_subexpression_of(&super_.hashee.return_val)
        }
    }

    impl IsStrictSubexpressionOf<RcSemHashed<App>> for Expr {
        fn is_strict_subexpression_of(&self, super_: &RcSemHashed<App>) -> bool {
            self.is_inclusive_subexpression_of(&super_.hashee.callee)
                || self.is_strict_subexpression_of_independent_expression_slice(
                    super_.hashee.args.hashee.as_slice(),
                )
        }
    }

    impl IsStrictSubexpressionOf<RcSemHashed<For>> for Expr {
        fn is_strict_subexpression_of(&self, super_: &RcSemHashed<For>) -> bool {
            self.is_strict_subexpression_of_dependent_expression_slice(
                super_.hashee.param_types.hashee.as_slice(),
            ) || self
                .clone()
                .replace_debs(&DebUpshifter(super_.hashee.param_types.hashee.len()), 0)
                .is_inclusive_subexpression_of(&super_.hashee.return_type)
        }
    }

    impl IsStrictSubexpressionOf<RcSemHashed<DebNode>> for Expr {
        fn is_strict_subexpression_of(&self, _: &RcSemHashed<DebNode>) -> bool {
            false
        }
    }

    impl IsStrictSubexpressionOf<RcSemHashed<UniverseNode>> for Expr {
        fn is_strict_subexpression_of(&self, _: &RcSemHashed<UniverseNode>) -> bool {
            false
        }
    }

    trait IsStrictSubexpressionOfExprSlice {
        fn is_strict_subexpression_of_independent_expression_slice(&self, super_: &[Expr]) -> bool;

        fn is_strict_subexpression_of_dependent_expression_slice(&self, super_: &[Expr]) -> bool;
    }

    impl IsStrictSubexpressionOfExprSlice for Expr {
        fn is_strict_subexpression_of_independent_expression_slice(&self, super_: &[Expr]) -> bool {
            super_
                .iter()
                .any(|super_expr| self.is_inclusive_subexpression_of(super_expr))
        }

        fn is_strict_subexpression_of_dependent_expression_slice(&self, super_: &[Expr]) -> bool {
            super_.iter().enumerate().any(|(i, super_expr)| {
                self.clone()
                    .replace_debs(&DebUpshifter(i), 0)
                    .is_inclusive_subexpression_of(super_expr)
            })
        }
    }
}

#[cfg(test)]
mod impl_ast_test {
    use super::*;

    use crate::test_utils::*;

    #[test]
    fn ind_index_arg_0() {
        let left_src = r#"0"#;
        let right_src = r#"(ind Type0 "" (0) ())"#;
        let left = parse_ast_or_panic(left_src);
        let right = parse_ast_or_panic(right_src);
        assert!(left.is_strict_subexpression_of(&right));
    }

    #[test]
    fn ind_index_arg_1() {
        let left_src = r#"0"#;
        let right_src = r#"(ind Type0 "" (999 1) ())"#;
        let left = parse_ast_or_panic(left_src);
        let right = parse_ast_or_panic(right_src);
        assert!(left.is_strict_subexpression_of(&right));
    }
}
