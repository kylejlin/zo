use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Expr {
    Ind(Rc<Hashed<Ind>>),
    Vcon(Rc<Hashed<Vcon>>),
    Match(Rc<Hashed<Match>>),
    Fun(Rc<Hashed<Fun>>),
    App(Rc<Hashed<App>>),
    For(Rc<Hashed<For>>),
    Deb(Rc<Hashed<DebNode>>),
    Universe(Rc<Hashed<UniverseNode>>),
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

    pub fn try_into_deb(self) -> Result<Rc<Hashed<DebNode>>, Self> {
        match self {
            Expr::Deb(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_universe(self) -> Result<Rc<Hashed<UniverseNode>>, Self> {
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
    pub vcon_defs: Rc<Hashed<Box<[VconDef]>>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct StringValue(pub String);

#[derive(Debug, Clone)]
pub struct VconDef {
    pub param_types: Rc<Hashed<Box<[Expr]>>>,
    pub index_args: Rc<Hashed<Box<[Expr]>>>,
}

#[derive(Debug, Clone)]
pub struct Vcon {
    pub ind: Rc<Hashed<Ind>>,
    pub vcon_index: usize,
}

#[derive(Debug, Clone)]
pub struct Match {
    pub matchee: Expr,
    pub return_type: Expr,
    pub cases: Rc<Hashed<Box<[MatchCase]>>>,
}

#[derive(Debug, Clone)]
pub struct MatchCase {
    pub arity: usize,
    pub return_val: Expr,
}

#[derive(Debug, Clone)]
pub struct Fun {
    pub decreasing_index: Option<usize>,
    pub param_types: Rc<Hashed<Box<[Expr]>>>,
    pub return_type: Expr,
    pub return_val: Expr,
}

#[derive(Debug, Clone)]
pub struct App {
    pub callee: Expr,
    pub args: Rc<Hashed<Box<[Expr]>>>,
}

#[derive(Debug, Clone)]
pub struct For {
    pub param_types: Rc<Hashed<Box<[Expr]>>>,
    pub return_type: Expr,
}

#[derive(Debug, Clone)]
pub struct DebNode {
    pub deb: Deb,
}

#[derive(Debug, Clone)]
pub struct UniverseNode {
    pub level: UniverseLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Deb(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct UniverseLevel(pub usize);
