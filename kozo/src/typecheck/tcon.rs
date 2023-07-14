use super::*;

use ast::Expr;

/// This type context is "lazy" in the sense
/// that it doesn't store the shifted types.
/// Instead, it lazily performs the shifting.
#[derive(Debug, Clone, Copy)]
pub enum LazyTypeContext<'a> {
    Base(Normalized<&'a [Expr]>),
    Snoc(&'a LazyTypeContext<'a>, Normalized<&'a [Expr]>),
}

impl LazyTypeContext<'_> {
    pub fn len(&self) -> usize {
        match self {
            LazyTypeContext::Base(types) => types.raw().len(),
            LazyTypeContext::Snoc(subcontext, types) => subcontext.len() + types.raw().len(),
        }
    }

    pub fn get(&self, deb: Deb) -> Option<NormalForm> {
        let unshifted = self.get_unshifted(deb)?;
        Some(unshifted.upshift(deb.0 + 1))
    }

    // TODO: Make private again.
    pub fn get_unshifted(&self, deb: Deb) -> Option<NormalForm> {
        match self {
            LazyTypeContext::Base(types) => {
                let index = (types.raw().len()).checked_sub(1 + deb.0)?;
                Some(types.get_ref(index)?.cloned())
            }

            LazyTypeContext::Snoc(subcontext, types) => {
                if let Some(index) = (types.raw().len()).checked_sub(1 + deb.0) {
                    Some(types.get_ref(index)?.cloned())
                } else {
                    subcontext.get(Deb(deb.0 - types.raw().len()))
                }
            }
        }
    }
}
