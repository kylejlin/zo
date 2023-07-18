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
/// that it doesn't store the shifted
/// `tentative_from` and `tentative_to`.
/// Instead, it lazily performs the shifting.
#[derive(Debug, Clone)]
pub struct LazySubstitution {
    pub tcon_len: usize,
    /// This is "tentative" in the sense
    /// that it will be swapped with `tentative_to`
    /// iff `tentative_from` is a strict subexpression
    /// of `tentative_to`.
    pub tentative_from: NormalForm,
    /// This is "tentative" in the sense
    /// that it will be swapped with `tentative_from`
    /// iff `tentative_from` is a strict subexpression
    /// of `tentative_to`.
    pub tentative_to: NormalForm,
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
        let tentative_from = sub.tentative_from.clone().upshift(upshift_amount, 0);
        let tentative_to = sub.tentative_to.clone().upshift(upshift_amount, 0);
        ConcreteSubstitution::new(tentative_from, tentative_to)
    })
}
