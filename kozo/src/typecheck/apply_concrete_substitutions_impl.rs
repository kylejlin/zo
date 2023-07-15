use super::*;

impl TypeChecker {
    pub(super) fn apply_concrete_substitutions<const N: usize>(
        &mut self,
        mut subs: Vec<ConcreteSubstitution>,
        mut exprs: [NormalForm; N],
    ) -> [NormalForm; N] {
        loop {
            let HasChanged(has_changed) =
                self.perform_substitution_iteration(&mut subs, &mut exprs);
            if !has_changed {
                return exprs;
            }
        }
    }

    fn perform_substitution_iteration<const N: usize>(
        &mut self,
        subs: &mut [ConcreteSubstitution],
        exprs: &mut [NormalForm; N],
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

                for expr in exprs.iter_mut() {
                    has_changed |= self.perform_substitution_on_expr(&applied_sub, expr);
                }
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
