pub use crate::{
    hashed::*,
    semantic_hash::*,
    token::{ByteIndex, NumberLiteral, StringLiteral, UniverseLiteral},
};

use std::rc::Rc;

/// Reference-counted semantically hashed.
pub type RcSemHashed<T> = Rc<SemanticallyHashed<T>>;

pub fn rc_sem_hashed<T: SemanticHash>(t: T) -> RcSemHashed<T> {
    Rc::new(Sha256Hashed::new(t))
}

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum Expr {
//     Ind(
//         Box<Ind>,
//     ),
//     Vcon(
//         Box<Vcon>,
//     ),
//     Match(
//         Box<Match>,
//     ),
//     Fun(
//         Box<Fun>,
//     ),
//     App(
//         Box<App>,
//     ),
//     For(
//         Box<For>,
//     ),
//     Deb(
//         crate::token::NumberLiteral,
//     ),
//     Universe(
//         crate::token::UniverseLiteral,
//     ),
// }
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    // TODO
    // Ind(RcHashed<Ind>),
    // Vcon(RcHashed<Vcon>),
    // Match(RcHashed<Match>),
    // Fun(RcHashed<Fun>),
    // App(RcHashed<App>),
    // For(RcHashed<For>),
    // Deb(RcHashed<NumberLiteral>),
    // Universe(RcHashed<UniverseLiteral>),
}

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct Ind {
//     pub lparen: crate::token::ByteIndex,
//     pub type_: crate::token::UniverseLiteral,
//     pub name: crate::token::StringLiteral,
//     pub index_types_lparen: crate::token::ByteIndex,
//     pub index_types: Box<ZeroOrMoreExprs>,
//     pub index_types_rparen: crate::token::ByteIndex,
//     pub vcon_defs_lparen: crate::token::ByteIndex,
//     pub vcon_defs: Box<ZeroOrMoreVconDefs>,
//     pub vcon_defs_rparen: crate::token::ByteIndex,
//     pub rparen: crate::token::ByteIndex,
// }
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum ZeroOrMoreExprs {
//     Nil,
//     Cons(
//         Box<ZeroOrMoreExprs>,
//         Box<Expr>,
//     ),
// }
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ZeroOrMoreExprs {
    Nil,
    Cons(Box<ZeroOrMoreExprs>, Expr),
}

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum ZeroOrMoreVconDefs {
//     Nil,
//     Cons(
//         Box<ZeroOrMoreVconDefs>,
//         Box<VconDef>,
//     ),
// }
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ZeroOrMoreVconDefs {
    Nil,
    // TODO
    // Cons(Box<ZeroOrMoreVconDefs>, VconDef),
}

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct VconDef {
//     pub lparen: crate::token::ByteIndex,
//     pub param_types_lparen: crate::token::ByteIndex,
//     pub param_types: Box<ZeroOrMoreExprs>,
//     pub param_types_rparen: crate::token::ByteIndex,
//     pub index_args_lparen: crate::token::ByteIndex,
//     pub index_args: Box<ZeroOrMoreExprs>,
//     pub index_args_rparen: crate::token::ByteIndex,
//     pub rparen: crate::token::ByteIndex,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct Vcon {
//     pub lparen: crate::token::ByteIndex,
//     pub ind: Box<Ind>,
//     pub vcon_index: crate::token::NumberLiteral,
//     pub rparen: crate::token::ByteIndex,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct Match {
//     pub lparen: crate::token::ByteIndex,
//     pub matchee: Box<Expr>,
//     pub return_type: Box<Expr>,
//     pub cases_lparen: crate::token::ByteIndex,
//     pub cases: Box<ZeroOrMoreMatchCases>,
//     pub cases_rparen: crate::token::ByteIndex,
//     pub rparen: crate::token::ByteIndex,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum ZeroOrMoreMatchCases {
//     Nil,
//     Cons(
//         Box<ZeroOrMoreMatchCases>,
//         Box<MatchCase>,
//     ),
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct MatchCase {
//     pub lparen: crate::token::ByteIndex,
//     pub arity: crate::token::NumberLiteral,
//     pub return_val: Box<Expr>,
//     pub rparen: crate::token::ByteIndex,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct Fun {
//     pub lparen: crate::token::ByteIndex,
//     pub decreasing_index: Box<NumberOrNonrecKw>,
//     pub param_types_lparen: crate::token::ByteIndex,
//     pub param_types: Box<ZeroOrMoreExprs>,
//     pub param_types_rparen: crate::token::ByteIndex,
//     pub return_type: Box<Expr>,
//     pub return_val: Box<Expr>,
//     pub rparen: crate::token::ByteIndex,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum NumberOrNonrecKw {
//     Number(
//         crate::token::NumberLiteral,
//     ),
//     NonrecKw(
//         crate::token::ByteIndex,
//     ),
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct App {
//     pub lparen: crate::token::ByteIndex,
//     pub callee: Box<Expr>,
//     pub args: Box<ZeroOrMoreExprs>,
//     pub rparen: crate::token::ByteIndex,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct For {
//     pub lparen: crate::token::ByteIndex,
//     pub param_types_lparen: crate::token::ByteIndex,
//     pub param_types: Box<ZeroOrMoreExprs>,
//     pub param_types_rparen: crate::token::ByteIndex,
//     pub return_type: Box<Expr>,
//     pub rparen: crate::token::ByteIndex,
// }
