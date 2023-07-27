use super::*;

// TODO: Reconsider if we really need this module.

#[derive(Clone, Debug)]
pub struct ExpectedTypeEquality {
    pub expr: cst::Expr,
    pub expected_type: NormalForm,
    pub actual_type: NormalForm,
}

/// `exprs`, `expected_types`, and `actual_types` **must** all have the same length.
#[derive(Clone, Debug)]
pub struct ExpectedTypeEqualities<'a> {
    pub exprs: &'a [cst::Expr],
    pub expected_types: Normalized<&'a [ast::Expr]>,
    pub actual_types: Normalized<&'a [ast::Expr]>,
}

impl<'a> ExpectedTypeEqualities<'a> {
    pub fn zip(self) -> impl Iterator<Item = ExpectedTypeEquality> + 'a {
        (0..self.len()).into_iter().map(move |i| {
            let expr = self.exprs[i].clone();
            let expected_type = self.expected_types.index_ref(i).cloned();
            let actual_type = self.actual_types.index_ref(i).cloned();
            ExpectedTypeEquality {
                expr,
                expected_type,
                actual_type,
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
    ) -> Result<(), TypeError> {
        for equality in equalities.zip() {
            self.assert_expected_type_equality_holds_after_applying_scon(equality)?;
        }

        Ok(())
    }

    pub(super) fn assert_expected_type_equality_holds_after_applying_scon(
        &mut self,
        expected_equality: ExpectedTypeEquality,
    ) -> Result<(), TypeError> {
        let ExpectedTypeEquality {
            expr,
            expected_type,
            actual_type,
        } = expected_equality;
        if actual_type.raw().digest() == expected_type.raw().digest() {
            return Ok(());
        }

        return Err(TypeError::TypeMismatch {
            expr,
            expected_type,
            actual_type,
        });
    }
}
