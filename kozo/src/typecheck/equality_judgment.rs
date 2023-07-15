use super::*;

#[derive(Clone, Debug)]
pub struct ExpectedTypeEquality {
    pub expr: cst::Expr,
    pub expected_type: NormalForm,
    pub actual_type: NormalForm,
    pub tcon_len: usize,
}

/// `exprs`, `expected_types`, and `actual_types` **must** all have the same length.
#[derive(Clone, Debug)]
pub struct ExpectedTypeEqualities {
    pub exprs: Vec<cst::Expr>,
    pub expected_types: Normalized<Vec<ast::Expr>>,
    pub actual_types: Normalized<Vec<ast::Expr>>,
    pub tcon_len: usize,
}

impl ExpectedTypeEqualities {
    pub fn zip(self) -> impl Iterator<Item = ExpectedTypeEquality> {
        let tcon_len = self.tcon_len;
        (0..self.len()).into_iter().map(move |i| {
            let expr = self.exprs[i].clone();
            let expected_type = self.expected_types.index_ref(i).cloned();
            let actual_type = self.actual_types.index_ref(i).cloned();
            ExpectedTypeEquality {
                expr,
                expected_type,
                actual_type,
                tcon_len,
            }
        })
    }

    pub fn len(&self) -> usize {
        self.exprs.len()
    }
}

impl TypeChecker {
    pub(super) fn assert_expected_type_equalities_holds_after_applying_scon(
        &mut self,
        equalities: ExpectedTypeEqualities,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        for equality in equalities.zip() {
            self.assert_expected_type_equality_holds_after_applying_scon(equality, scon)?;
        }

        Ok(())
    }

    pub(super) fn assert_expected_type_equality_holds_after_applying_scon(
        &mut self,
        expected_equality: ExpectedTypeEquality,
        scon: LazySubstitutionContext,
    ) -> Result<(), TypeError> {
        let ExpectedTypeEquality {
            expr,
            expected_type,
            actual_type,
            tcon_len,
        } = expected_equality;
        if actual_type.raw().digest() == expected_type.raw().digest() {
            return Ok(());
        }

        let (subbed_expected, subbed_actual) =
            self.apply_scon(scon, tcon_len, expected_type.clone(), actual_type.clone());

        if subbed_expected.raw().digest() == subbed_actual.raw().digest() {
            return Ok(());
        }

        return Err(TypeError::TypeMismatch {
            expr,
            expected_type,
            actual_type,
            subbed_expected,
            subbed_actual,
        });
    }

    pub(super) fn apply_scon(
        &mut self,
        scon: LazySubstitutionContext,
        tcon_len: usize,
        expr1: NormalForm,
        expr2: NormalForm,
    ) -> (NormalForm, NormalForm) {
        let subs = scon.into_concrete_noncompounded_substitutions(tcon_len);
        let old_exprs = [expr1, expr2];
        let new_exprs = self.apply_concrete_substitutions(subs, old_exprs);
        (new_exprs[0].clone(), new_exprs[1].clone())
    }
}
