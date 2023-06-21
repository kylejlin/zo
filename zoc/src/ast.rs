use crate::{cst, token::*};

use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Expr {
    Ind(Rc<Hashed<Ind>>),
    Vcon(Rc<Hashed<Vcon>>),
    Match(Rc<Hashed<Match>>),
    Fun(Rc<Hashed<Fun>>),
    App(Rc<Hashed<App>>),
    For(Rc<Hashed<For>>),
    Deb(Rc<Hashed<NumberLiteral>>),
    Universe(Rc<Hashed<UniverseLiteral>>),
}

impl Expr {
    pub fn digest(&self) -> &Digest {
        match self {
            Expr::Ind(h) => &h.digest,
            Expr::Vcon(h) => &h.digest,
            Expr::Match(h) => &h.digest,
            Expr::Fun(h) => &h.digest,
            Expr::App(h) => &h.digest,
            Expr::For(h) => &h.digest,
            Expr::Deb(h) => &h.digest,
            Expr::Universe(h) => &h.digest,
        }
    }
}

pub use crate::semantic_hash::*;

#[derive(Clone, Debug)]
pub struct Ind {
    pub name: Rc<Hashed<StringLiteral>>,
    pub universe_level: usize,
    pub index_types: Rc<Hashed<Box<[Expr]>>>,
    pub constructor_defs: Rc<Hashed<Box<[VariantConstructorDef]>>>,
    pub original: Option<Rc<cst::Ind>>,
}

#[derive(Debug, Clone)]
pub struct VariantConstructorDef {
    pub param_types: Rc<Hashed<Box<[Expr]>>>,
    pub index_args: Rc<Hashed<Box<[Expr]>>>,
    pub original: Option<Rc<cst::VariantConstructorDef>>,
}

#[derive(Debug, Clone)]
pub struct Vcon {
    pub ind: Rc<Hashed<Ind>>,
    pub vcon_index: usize,
    pub original: Option<Rc<cst::Vcon>>,
}

#[derive(Debug, Clone)]
pub struct Match {
    pub matchee: Rc<Expr>,
    pub return_type: Rc<Expr>,
    pub cases: Rc<Hashed<Box<[Expr]>>>,
    pub original: Option<Rc<cst::Match>>,
}

#[derive(Debug, Clone)]
pub struct Fun {
    pub decreasing_index: Option<usize>,
    pub param_types: Rc<Hashed<Box<[Expr]>>>,
    pub return_type: Rc<Expr>,
    pub return_val: Rc<Expr>,
    pub original: Option<Rc<cst::Fun>>,
}

#[derive(Debug, Clone)]
pub struct App {
    pub callee: Box<Expr>,
    pub args: Rc<Hashed<Box<[Expr]>>>,
    pub original: Option<Rc<cst::App>>,
}

#[derive(Debug, Clone)]
pub struct For {
    pub param_types: Rc<Hashed<Box<[Expr]>>>,
    pub return_type: Rc<Expr>,
    pub original: Option<Rc<cst::For>>,
}
