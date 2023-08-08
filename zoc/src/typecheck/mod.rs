use crate::{
    eval::{Evaluator, NormalForm, Normalized},
    hash::*,
    syntax_tree::{
        ast::{self, Deb, RcHashed, RcHashedVec, Universe, UniverseLevel},
        ipist::{self as cst},
        ipist_to_ast::IpistToAstConverter,
        replace_debs::*,
        token::*,
    },
};

use std::rc::Rc;

mod equality_assertion;
use equality_assertion::*;

mod error;
pub use error::TypeError;

mod tcon;
pub use tcon::*;

mod typechecker_impls;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Default)]
pub struct TypeChecker {
    pub evaluator: Evaluator,
    pub cst_converter: IpistToAstConverter,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ast::Expr {
    fn is_universe(&self) -> bool {
        match self {
            ast::Expr::Universe(_) => true,
            _ => false,
        }
    }
}

/// Non-universe expressions are ignored.
/// If there are no universe expressions, `None` is returned.
fn get_max_universe_level<'a>(
    exprs: impl IntoIterator<Item = &'a ast::Expr>,
) -> Option<UniverseLevel> {
    exprs
        .into_iter()
        .filter_map(|expr| match expr {
            ast::Expr::Universe(universe) => Some(universe.hashee.universe.level),
            _ => None,
        })
        .max()
}

trait MaxOrSelf: Sized {
    /// If `other` is `None`, `self` is returned.
    /// Otherwise, `self.max(o)` is returned, where `other` equals `Some(o)`.
    fn max_or_self(self, other: Option<Self>) -> Self;
}

impl<T> MaxOrSelf for T
where
    T: Ord,
{
    fn max_or_self(self, other: Option<Self>) -> Self {
        match other {
            Some(other) => self.max(other),
            None => self,
        }
    }
}
