use super::*;

#[derive(Clone, Debug)]
pub struct ExpectedTypeEquality {
    pub expr: Expr,
    pub expected_type: NormalForm,
    pub actual_type: NormalForm,
    pub tcon_len: usize,
}

/// `exprs`, `expected_types`, and `actual_types` **must** all have the same length.
#[derive(Clone, Debug)]
pub struct ExpectedTypeEqualities {
    pub exprs: RcHashed<Box<[Expr]>>,
    pub expected_types: Normalized<Vec<Expr>>,
    pub actual_types: Normalized<Vec<Expr>>,
    pub tcon_len: usize,
}

impl ExpectedTypeEqualities {
    pub fn zip(self) -> impl Iterator<Item = ExpectedTypeEquality> {
        let tcon_len = self.tcon_len;
        (0..self.len()).into_iter().map(move |i| {
            let expr = self.exprs.value[i].clone();
            let expected_type = self.expected_types.index(i).cloned();
            let actual_type = self.actual_types.index(i).cloned();
            ExpectedTypeEquality {
                expr,
                expected_type,
                actual_type,
                tcon_len,
            }
        })
    }

    pub fn len(&self) -> usize {
        self.exprs.value.len()
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
        mut expr1: NormalForm,
        mut expr2: NormalForm,
    ) -> (NormalForm, NormalForm) {
        let mut subs = scon.into_concrete_noncompounded_substitutions(tcon_len);

        loop {
            let HasChanged(has_changed) =
                self.perform_substitution_iteration(&mut subs, &mut expr1, &mut expr2);
            if !has_changed {
                return (expr1, expr2);
            }
        }
    }

    fn perform_substitution_iteration(
        &mut self,
        subs: &mut [ConcreteSubstitution],
        expr1: &mut NormalForm,
        expr2: &mut NormalForm,
    ) -> HasChanged {
        let mut has_changed = HasChanged(false);
        for applied_sub_index in 0..subs.len() {
            let applied_sub = subs[applied_sub_index].clone();
            for target_sub_index in 0..subs.len() {
                if target_sub_index == applied_sub_index {
                    continue;
                }

                has_changed |= self.perform_substitution_on_substitution(
                    &applied_sub,
                    &mut subs[target_sub_index],
                );

                has_changed |= self.perform_substitution_on_expr(&applied_sub, expr1);
                has_changed |= self.perform_substitution_on_expr(&applied_sub, expr2);
            }
        }

        has_changed
    }

    fn perform_substitution_on_substitution(
        &mut self,
        applied_sub: &ConcreteSubstitution,
        target_sub: &mut ConcreteSubstitution,
    ) -> HasChanged {
        let mut has_changed = HasChanged(false);
        has_changed |= self.perform_substitution_on_expr(applied_sub, &mut target_sub.from);
        has_changed |= self.perform_substitution_on_expr(applied_sub, &mut target_sub.to);
        has_changed
    }

    fn perform_substitution_on_expr(
        &mut self,
        applied_sub: &ConcreteSubstitution,
        expr: &mut NormalForm,
    ) -> HasChanged {
        let subbed = expr.raw().clone().substitute(applied_sub);
        let normalized = self.evaluator.eval(subbed);

        if expr.raw().digest() == normalized.raw().digest() {
            return HasChanged(false);
        }

        *expr = normalized;
        HasChanged(true)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct HasChanged(pub bool);

impl BitOrAssign for HasChanged {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
