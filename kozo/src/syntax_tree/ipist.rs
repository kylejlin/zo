pub use crate::{
    hash::*,
    syntax_tree::token::{ByteIndex, NumberLiteral, StringLiteral, UniverseLiteral},
};

use std::{hash::Hash, rc::Rc};

/// Reference-counted hashed.
pub type RcHashed<T> = Rc<Hashed<T>>;

/// Reference-counted hashed vector.
pub type RcHashedVec<T> = RcHashed<Vec<T>>;

pub fn rc_hashed<T: Hash>(t: T) -> RcHashed<T> {
    Rc::new(Hashed::new(t))
}

#[derive(Debug, Clone, Hash)]
pub enum Expr {
    Ind(RcHashed<Ind>),
    Vcon(RcHashed<Vcon>),
    Match(RcHashed<Match>),
    Fun(RcHashed<Fun>),
    App(RcHashed<App>),
    For(RcHashed<For>),
    Deb(RcHashed<NumberLiteral>),
    Universe(RcHashed<UniverseLiteral>),
}

#[derive(Debug, Clone, Hash)]
pub struct Ind {
    pub lparen: ByteIndex,
    pub type_: UniverseLiteral,
    pub name: StringLiteral,
    pub index_types_lparen: ByteIndex,
    pub index_types: Vec<Expr>,
    pub index_types_rparen: ByteIndex,
    pub vcon_defs_lparen: ByteIndex,
    pub vcon_defs: Vec<VconDef>,
    pub vcon_defs_rparen: ByteIndex,
    pub rparen: ByteIndex,
}

#[derive(Debug, Clone, Hash)]
pub struct VconDef {
    pub lparen: ByteIndex,
    pub param_types_lparen: ByteIndex,
    pub param_types: Vec<Expr>,
    pub param_types_rparen: ByteIndex,
    pub index_args_lparen: ByteIndex,
    pub index_args: Vec<Expr>,
    pub index_args_rparen: ByteIndex,
    pub rparen: ByteIndex,
}

#[derive(Debug, Clone, Hash)]
pub struct Vcon {
    pub lparen: ByteIndex,
    pub ind: RcHashed<Ind>,
    pub vcon_index: NumberLiteral,
    pub rparen: ByteIndex,
}

#[derive(Debug, Clone, Hash)]
pub struct Match {
    pub lparen: ByteIndex,
    pub matchee: Expr,
    pub return_type: Expr,
    pub cases_lparen: ByteIndex,
    pub cases: Vec<MatchCase>,
    pub cases_rparen: ByteIndex,
    pub rparen: ByteIndex,
}

#[derive(Debug, Clone, Hash)]
pub struct MatchCase {
    pub lparen: ByteIndex,
    pub arity: NumberLiteral,
    pub return_val: Expr,
    pub rparen: ByteIndex,
}

#[derive(Debug, Clone, Hash)]
pub struct Fun {
    pub lparen: ByteIndex,
    pub decreasing_index: NumberOrNonrecKw,
    pub param_types_lparen: ByteIndex,
    pub param_types: Vec<Expr>,
    pub param_types_rparen: ByteIndex,
    pub return_type: Expr,
    pub return_val: Expr,
    pub rparen: ByteIndex,
}

pub use crate::syntax_tree::ost::NumberOrNonrecKw;

#[derive(Debug, Clone, Hash)]
pub struct App {
    pub lparen: ByteIndex,
    pub callee: Expr,
    pub args: Vec<Expr>,
    pub rparen: ByteIndex,
}

#[derive(Debug, Clone, Hash)]
pub struct For {
    pub lparen: ByteIndex,
    pub param_types_lparen: ByteIndex,
    pub param_types: Vec<Expr>,
    pub param_types_rparen: ByteIndex,
    pub return_type: Expr,
    pub rparen: ByteIndex,
}

impl From<Ind> for Expr {
    fn from(ind: Ind) -> Self {
        Expr::Ind(rc_hashed(ind))
    }
}
impl From<Vcon> for Expr {
    fn from(vcon: Vcon) -> Self {
        Expr::Vcon(rc_hashed(vcon))
    }
}
impl From<Match> for Expr {
    fn from(match_: Match) -> Self {
        Expr::Match(rc_hashed(match_))
    }
}
impl From<Fun> for Expr {
    fn from(fun: Fun) -> Self {
        Expr::Fun(rc_hashed(fun))
    }
}
impl From<App> for Expr {
    fn from(app: App) -> Self {
        Expr::App(rc_hashed(app))
    }
}
impl From<For> for Expr {
    fn from(for_: For) -> Self {
        Expr::For(rc_hashed(for_))
    }
}
impl From<NumberLiteral> for Expr {
    fn from(deb: NumberLiteral) -> Self {
        Expr::Deb(rc_hashed(deb))
    }
}
impl From<UniverseLiteral> for Expr {
    fn from(universe: UniverseLiteral) -> Self {
        Expr::Universe(rc_hashed(universe))
    }
}
