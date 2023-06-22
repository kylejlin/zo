use crate::cst;

use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Expr {
    Ind(Rc<Hashed<Ind>>),
    Vcon(Rc<Hashed<Vcon>>),
    Match(Rc<Hashed<Match>>),
    Fun(Rc<Hashed<Fun>>),
    App(Rc<Hashed<App>>),
    For(Rc<Hashed<For>>),
    Deb(Rc<Hashed<Deb>>),
    Universe(Rc<Hashed<Universe>>),
}

impl Expr {
    pub fn digest(&self) -> &Digest {
        match self {
            Expr::Ind(e) => &e.digest,
            Expr::Vcon(e) => &e.digest,
            Expr::Match(e) => &e.digest,
            Expr::Fun(e) => &e.digest,
            Expr::App(e) => &e.digest,
            Expr::For(e) => &e.digest,
            Expr::Deb(e) => &e.digest,
            Expr::Universe(e) => &e.digest,
        }
    }
}

pub use crate::semantic_hash::*;

impl Expr {
    pub fn try_into_ind(self) -> Result<Rc<Hashed<Ind>>, Self> {
        match self {
            Expr::Ind(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_vcon(self) -> Result<Rc<Hashed<Vcon>>, Self> {
        match self {
            Expr::Vcon(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_match(self) -> Result<Rc<Hashed<Match>>, Self> {
        match self {
            Expr::Match(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_fun(self) -> Result<Rc<Hashed<Fun>>, Self> {
        match self {
            Expr::Fun(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_app(self) -> Result<Rc<Hashed<App>>, Self> {
        match self {
            Expr::App(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_for(self) -> Result<Rc<Hashed<For>>, Self> {
        match self {
            Expr::For(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_deb(self) -> Result<Rc<Hashed<Deb>>, Self> {
        match self {
            Expr::Deb(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_universe(self) -> Result<Rc<Hashed<Universe>>, Self> {
        match self {
            Expr::Universe(e) => Ok(e),
            _ => Err(self),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Ind {
    pub name: Rc<StringValue>,
    pub universe_level: usize,
    pub index_types: Rc<Hashed<Box<[Expr]>>>,
    pub constructor_defs: Rc<Hashed<Box<[Rc<Hashed<VariantConstructorDef>>]>>>,
    pub original: Option<Rc<cst::Ind>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct StringValue(pub String);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Deb(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Universe {
    pub level: usize,
}
