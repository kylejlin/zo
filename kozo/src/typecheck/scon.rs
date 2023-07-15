use super::*;

/// This substitution context is "lazy" in the sense
/// that it doesn't store the shifted substitutions.
/// Instead, it lazily performs the shifting.
#[derive(Debug, Clone, Copy)]
pub enum LazySubstitutionContext<'a> {
    Base(&'a [LazySubstitution]),
    Snoc(&'a LazySubstitutionContext<'a>, &'a [LazySubstitution]),
}

/// This substitution is "lazy" in the sense
/// that it doesn't store the shifted `from` and `to`.
/// Instead, it lazily performs the shifting.
#[derive(Debug, Clone)]
pub struct LazySubstitution {
    pub tcon_len: usize,
    pub from: NormalForm,
    pub to: NormalForm,
}

impl LazySubstitutionContext<'_> {
    pub fn len(&self) -> usize {
        match self {
            LazySubstitutionContext::Base(subs) => subs.len(),
            LazySubstitutionContext::Snoc(first, subs) => first.len() + subs.len(),
        }
    }

    pub fn into_concrete_noncompounded_substitutions(
        self,
        current_tcon_len: usize,
    ) -> Vec<ConcreteSubstitution> {
        match self {
            LazySubstitutionContext::Base(subs) => {
                lazy_substitution_slice_to_concrete_noncompounded_substitutions(
                    subs,
                    current_tcon_len,
                )
                .collect()
            }

            LazySubstitutionContext::Snoc(first, subs) => {
                let mut first = first.into_concrete_noncompounded_substitutions(current_tcon_len);
                let rest = lazy_substitution_slice_to_concrete_noncompounded_substitutions(
                    subs,
                    current_tcon_len,
                );
                first.extend(rest);
                first
            }
        }
    }
}

fn lazy_substitution_slice_to_concrete_noncompounded_substitutions(
    subs: &[LazySubstitution],
    current_tcon_len: usize,
) -> impl Iterator<Item = ConcreteSubstitution> + '_ {
    subs.iter().map(move |sub| {
        let upshift_amount = current_tcon_len - sub.tcon_len;
        let from = sub.from.clone().upshift(upshift_amount, 0);
        let to = sub.to.clone().upshift(upshift_amount, 0);
        ConcreteSubstitution { from, to }
    })
}
