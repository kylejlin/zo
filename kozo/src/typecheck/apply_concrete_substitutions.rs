use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HasExploded(pub bool);

impl BitOrAssign for HasExploded {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl TypeChecker {
    pub(super) fn apply_concrete_substitutions<const N: usize>(
        &mut self,
        mut subs: Vec<ConcreteSubstitution>,
        mut exprs: [NormalForm; N],
    ) -> ([NormalForm; N], HasExploded) {
        loop {
            let HasChanged(has_changed) =
                self.perform_substitution_iteration(&mut subs, &mut exprs);
            if !has_changed {
                let has_exploded = does_any_substitution_contain_exploding_contradiction(&subs);
                // TODO: Delete
                {
                    use crate::pretty_print::*;
                    println!("DONE applying concrete subs ({}).", subs.len());
                    for (i, sub) in subs.iter().enumerate() {
                        println!(
                            "sub_final[{}].from = {}",
                            i,
                            sub.from().raw().pretty_printed()
                        );
                        println!("sub_final[{}].to = {}", i, sub.to().raw().pretty_printed());
                    }
                }
                return (exprs, HasExploded(has_exploded));
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
        let tentative_from = {
            let mut out = target_sub.from().clone();
            self.perform_substitution_on_expr(applied_sub, &mut out);
            out
        };
        let tentative_to = {
            let mut out = target_sub.to().clone();
            self.perform_substitution_on_expr(applied_sub, &mut out);
            out
        };
        let new_sub = ConcreteSubstitution::new(tentative_from, tentative_to);

        if *target_sub == new_sub {
            return HasChanged(false);
        }

        *target_sub = new_sub;
        HasChanged(true)
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

fn does_any_substitution_contain_exploding_contradiction(subs: &[ConcreteSubstitution]) -> bool {
    subs.iter()
        .any(does_substitution_contain_exploding_contradiction)
}

fn does_substitution_contain_exploding_contradiction(sub: &ConcreteSubstitution) -> bool {
    use ast::Expr;

    match (sub.from().raw(), sub.to().raw()) {
        (Expr::Deb(_), _) => false,
        (_, Expr::Deb(_)) => false,

        (Expr::Ind(from), Expr::Ind(to)) => from.hashee.name != to.hashee.name,
        (Expr::Vcon(from), Expr::Vcon(to)) => from.hashee.vcon_index != to.hashee.vcon_index,

        (Expr::Match(_), Expr::Match(_)) => false,
        (Expr::Fun(_), Expr::Fun(_)) => false,
        (Expr::App(_), Expr::App(_)) => false,
        (Expr::For(_), Expr::For(_)) => false,
        (Expr::Universe(_), Expr::Universe(_)) => false,

        _ => true,
    }
}
