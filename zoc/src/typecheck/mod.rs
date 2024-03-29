use crate::{
    eval::{Evaluator, NormalForm, Normalized},
    hash::*,
    syntax_tree::{
        ast::prelude::{minimal_ast::UnitAuxDataFamily, spanned_ast::SpanAuxDataFamily, *},
        remove_ast_aux_data::AuxDataRemover,
        replace_debs::*,
    },
};

use std::{marker::PhantomData, rc::Rc};

mod check_fun_recursion;
use check_fun_recursion::*;

mod check_positivity;
use check_positivity::*;

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
    pub aux_remover: AuxDataRemover,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self::default()
    }
}

impl minimal_ast::Expr {
    fn is_universe(&self) -> bool {
        match self {
            minimal_ast::Expr::Universe(_) => true,
            _ => false,
        }
    }
}

/// Non-universe expressions are ignored.
/// If there are no universe expressions, `None` is returned.
fn get_max_universe_level<'a>(
    exprs: impl IntoIterator<Item = &'a minimal_ast::Expr>,
) -> Option<UniverseLevel> {
    exprs
        .into_iter()
        .filter_map(|expr| match expr {
            minimal_ast::Expr::Universe(universe) => Some(universe.hashee.universe.level),
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
