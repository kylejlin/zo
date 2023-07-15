use crate::{
    eval::{Evaluator, NormalForm, Normalized},
    hash::sha256::*,
    syntax_tree::{
        ast::{self, Deb, RcSemHashed, RcSemHashedVec, UniverseLevel},
        rch_cst::{self as cst, RcHashed},
        rch_cst_to_ast::RchCstToAstConverter,
        replace_debs::*,
        token::*,
    },
};

use std::{ops::BitOrAssign, rc::Rc};

mod concrete_substitution;
use concrete_substitution::*;

mod cst_impls;

mod equality_judgment;
use equality_judgment::*;

mod error;
use error::*;

mod typechecker_impls;

mod scon;
pub use scon::*;

mod tcon;
pub use tcon::*;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Default)]
pub struct TypeChecker {
    pub evaluator: Evaluator,
    pub cst_converter: RchCstToAstConverter,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self::default()
    }
}

fn assert_every_expr_is_universe(exprs: &[ast::Expr]) -> Result<(), usize> {
    for (i, expr) in exprs.iter().enumerate() {
        if !expr.is_universe() {
            return Err(i);
        }
    }

    Ok(())
}

impl ast::Expr {
    fn is_universe(&self) -> bool {
        match self {
            ast::Expr::Universe(_) => true,
            _ => false,
        }
    }
}

fn get_max_universe_level<'a>(
    exprs: impl IntoIterator<Item = &'a ast::Expr>,
) -> Option<UniverseLevel> {
    exprs
        .into_iter()
        .filter_map(|expr| match expr {
            ast::Expr::Universe(universe) => Some(universe.hashee.level),
            _ => None,
        })
        .max()
}

trait MaxOrSelf: Sized {
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
