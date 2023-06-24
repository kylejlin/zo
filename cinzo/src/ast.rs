use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Expr {
    Ind(RcHashed<Ind>),
    Vcon(RcHashed<Vcon>),
    Match(RcHashed<Match>),
    Fun(RcHashed<Fun>),
    App(RcHashed<App>),
    For(RcHashed<For>),
    Deb(RcHashed<DebNode>),
    Universe(RcHashed<UniverseNode>),
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
    pub fn try_into_ind(self) -> Result<RcHashed<Ind>, Self> {
        match self {
            Expr::Ind(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_vcon(self) -> Result<RcHashed<Vcon>, Self> {
        match self {
            Expr::Vcon(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_match(self) -> Result<RcHashed<Match>, Self> {
        match self {
            Expr::Match(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_fun(self) -> Result<RcHashed<Fun>, Self> {
        match self {
            Expr::Fun(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_app(self) -> Result<RcHashed<App>, Self> {
        match self {
            Expr::App(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_for(self) -> Result<RcHashed<For>, Self> {
        match self {
            Expr::For(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_deb(self) -> Result<RcHashed<DebNode>, Self> {
        match self {
            Expr::Deb(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_universe(self) -> Result<RcHashed<UniverseNode>, Self> {
        match self {
            Expr::Universe(e) => Ok(e),
            _ => Err(self),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Ind {
    pub name: Rc<StringValue>,
    pub universe_level: UniverseLevel,
    pub index_types: RcHashed<Box<[Expr]>>,
    pub vcon_defs: RcHashed<Box<[VconDef]>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct StringValue(pub String);

#[derive(Debug, Clone)]
pub struct VconDef {
    pub param_types: RcHashed<Box<[Expr]>>,
    pub index_args: RcHashed<Box<[Expr]>>,
}

#[derive(Debug, Clone)]
pub struct Vcon {
    pub ind: RcHashed<Ind>,
    pub vcon_index: usize,
}

#[derive(Debug, Clone)]
pub struct Match {
    pub matchee: Expr,
    pub return_type: Expr,
    pub cases: RcHashed<Box<[MatchCase]>>,
}

#[derive(Debug, Clone)]
pub struct MatchCase {
    pub arity: usize,
    pub return_val: Expr,
}

#[derive(Debug, Clone)]
pub struct Fun {
    pub decreasing_index: Option<usize>,
    pub param_types: RcHashed<Box<[Expr]>>,
    pub return_type: Expr,
    pub return_val: Expr,
}

#[derive(Debug, Clone)]
pub struct App {
    pub callee: Expr,
    pub args: RcHashed<Box<[Expr]>>,
}

#[derive(Debug, Clone)]
pub struct For {
    pub param_types: RcHashed<Box<[Expr]>>,
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

pub type RcHashed<T> = Rc<Hashed<T>>;

pub fn rc_hash<T: SemanticHash>(t: T) -> RcHashed<T> {
    Rc::new(Hashed::new(t))
}

impl App {
    pub fn collapse_if_nullary(self) -> Expr {
        if self.args.value.is_empty() {
            self.callee
        } else {
            Expr::App(Rc::new(Hashed::new(self)))
        }
    }
}

impl For {
    pub fn collapse_if_nullary(self) -> Expr {
        if self.param_types.value.is_empty() {
            self.return_type
        } else {
            Expr::For(Rc::new(Hashed::new(self)))
        }
    }
}
