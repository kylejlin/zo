use std::{hash::Hash, rc::Rc};

pub use crate::hash::*;

pub use crate::syntax_tree::ipist::{rc_hashed, RcHashed, RcHashedVec};

mod conversion;
mod debug;
mod get_digest;
mod hash;

#[derive(Clone, PartialEq, Eq)]
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

#[derive(Clone, PartialEq, Eq)]
pub struct Ind {
    pub name: Rc<StringValue>,
    pub universe: Universe,
    pub index_types: RcHashedVec<Expr>,
    pub vcon_defs: RcHashedVec<VconDef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct StringValue(pub String);

#[derive(Clone, PartialEq, Eq)]
pub struct VconDef {
    pub param_types: RcHashedVec<Expr>,
    pub index_args: RcHashedVec<Expr>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Vcon {
    pub ind: RcHashed<Ind>,
    pub vcon_index: usize,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Match {
    pub matchee: Expr,
    pub return_type_arity: usize,
    pub return_type: Expr,
    pub cases: RcHashedVec<MatchCase>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct MatchCase {
    pub arity: usize,
    pub return_val: Expr,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Fun {
    pub decreasing_index: Option<usize>,
    pub param_types: RcHashedVec<Expr>,
    pub return_type: Expr,
    pub return_val: Expr,
}

#[derive(Clone, PartialEq, Eq)]
pub struct App {
    pub callee: Expr,
    pub args: RcHashedVec<Expr>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct For {
    pub param_types: RcHashedVec<Expr>,
    pub return_type: Expr,
}

#[derive(Clone, PartialEq, Eq)]
pub struct DebNode {
    pub deb: Deb,
}

#[derive(Clone, PartialEq, Eq)]
pub struct UniverseNode {
    pub universe: Universe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Deb(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Universe {
    pub level: UniverseLevel,
    pub erasable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct UniverseLevel(pub usize);

impl App {
    pub fn collapse_if_nullary(self) -> Expr {
        if self.args.hashee.is_empty() {
            self.callee
        } else {
            Expr::App(Rc::new(Hashed::new(self)))
        }
    }
}

impl For {
    pub fn collapse_if_nullary(self) -> Expr {
        if self.param_types.hashee.is_empty() {
            self.return_type
        } else {
            Expr::For(Rc::new(Hashed::new(self)))
        }
    }
}
