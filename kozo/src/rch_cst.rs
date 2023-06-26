pub use crate::{
    hashed::*,
    semantic_hash::*,
    token::{ByteIndex, NumberLiteral, StringLiteral, UniverseLiteral},
};

use std::{hash::Hash, rc::Rc};

/// Reference-counted semantically hashed.
pub type RcHashed<T> = Rc<Sha256Hashed<T, DefaultHashAlgorithm>>;

pub fn rc_hashed<T: Hash>(t: T) -> RcHashed<T> {
    Rc::new(Sha256Hashed::new(t))
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Ind {
    pub lparen: ByteIndex,
    pub type_: UniverseLiteral,
    pub name: StringLiteral,
    pub index_types_lparen: ByteIndex,
    pub index_types: ZeroOrMoreExprs,
    pub index_types_rparen: ByteIndex,
    pub vcon_defs_lparen: ByteIndex,
    pub vcon_defs: ZeroOrMoreVconDefs,
    pub vcon_defs_rparen: ByteIndex,
    pub rparen: ByteIndex,
}

#[derive(Debug, Clone)]
pub enum ZeroOrMoreExprs {
    Nil,
    Cons(Box<ZeroOrMoreExprs>, Expr),
}

#[derive(Debug, Clone)]
pub enum ZeroOrMoreVconDefs {
    Nil,
    Cons(Box<ZeroOrMoreVconDefs>, VconDef),
}

#[derive(Debug, Clone)]
pub struct VconDef {
    pub lparen: ByteIndex,
    pub param_types_lparen: ByteIndex,
    pub param_types: ZeroOrMoreExprs,
    pub param_types_rparen: ByteIndex,
    pub index_args_lparen: ByteIndex,
    pub index_args: ZeroOrMoreExprs,
    pub index_args_rparen: ByteIndex,
    pub rparen: ByteIndex,
}

#[derive(Debug, Clone)]
pub struct Vcon {
    pub lparen: ByteIndex,
    pub ind: RcHashed<Ind>,
    pub vcon_index: NumberLiteral,
    pub rparen: ByteIndex,
}

#[derive(Debug, Clone)]
pub struct Match {
    pub lparen: ByteIndex,
    pub matchee: Expr,
    pub return_type: Expr,
    pub cases_lparen: ByteIndex,
    pub cases: ZeroOrMoreMatchCases,
    pub cases_rparen: ByteIndex,
    pub rparen: ByteIndex,
}

#[derive(Debug, Clone)]
pub enum ZeroOrMoreMatchCases {
    Nil,
    Cons(Box<ZeroOrMoreMatchCases>, MatchCase),
}

#[derive(Debug, Clone)]
pub struct MatchCase {
    pub lparen: ByteIndex,
    pub arity: NumberLiteral,
    pub return_val: Expr,
    pub rparen: ByteIndex,
}

#[derive(Debug, Clone)]
pub struct Fun {
    pub lparen: ByteIndex,
    pub decreasing_index: NumberOrNonrecKw,
    pub param_types_lparen: ByteIndex,
    pub param_types: ZeroOrMoreExprs,
    pub param_types_rparen: ByteIndex,
    pub return_type: Expr,
    pub return_val: Expr,
    pub rparen: ByteIndex,
}

pub use crate::cst::NumberOrNonrecKw;

#[derive(Debug, Clone)]
pub struct App {
    pub lparen: ByteIndex,
    pub callee: Expr,
    pub args: ZeroOrMoreExprs,
    pub rparen: ByteIndex,
}

#[derive(Debug, Clone)]
pub struct For {
    pub lparen: ByteIndex,
    pub param_types_lparen: ByteIndex,
    pub param_types: ZeroOrMoreExprs,
    pub param_types_rparen: ByteIndex,
    pub return_type: Expr,
    pub rparen: ByteIndex,
}
