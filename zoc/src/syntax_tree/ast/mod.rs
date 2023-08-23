use std::{fmt::Debug, hash::Hash, rc::Rc};

pub use crate::hash::*;

mod conversion;
mod get_digest;
mod hash;

pub mod families;

pub mod node_path;
pub use node_path::{NodeEdge, NodePath};

pub mod prelude;

pub trait AstFamily:
    'static + Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash + Default
{
    type IndAux: Clone + Hash;
    type VconAux: Clone + Hash;
    type MatchAux: Clone + Hash;
    type FunAux: Clone + Hash;
    type AppAux: Clone + Hash;
    type ForAux: Clone + Hash;
    type DebAux: Clone + Hash;
    type UniverseAux: Clone + Hash;

    type VconDefAux: Clone + Hash;
    type MatchCaseAux: Clone + Hash;
}

#[derive(Clone, PartialEq, Eq)]
pub enum Expr<A: AstFamily> {
    Ind(RcHashed<Ind<A>>),
    Vcon(RcHashed<Vcon<A>>),
    Match(RcHashed<Match<A>>),
    Fun(RcHashed<Fun<A>>),
    App(RcHashed<App<A>>),
    For(RcHashed<For<A>>),
    Deb(RcHashed<DebNode<A>>),
    Universe(RcHashed<UniverseNode<A>>),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Ind<A: AstFamily> {
    pub name: Rc<StringValue>,
    pub universe: Universe,
    pub index_types: RcHashedVec<Expr<A>>,
    pub vcon_defs: RcHashedVec<VconDef<A>>,
    pub aux_data: A::IndAux,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct StringValue(pub String);

#[derive(Clone, PartialEq, Eq)]
pub struct VconDef<A: AstFamily> {
    pub param_types: RcHashedVec<Expr<A>>,
    pub index_args: RcHashedVec<Expr<A>>,
    pub aux_data: A::VconDefAux,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Vcon<A: AstFamily> {
    pub ind: RcHashed<Ind<A>>,
    pub vcon_index: usize,
    pub aux_data: A::VconAux,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Match<A: AstFamily> {
    pub matchee: Expr<A>,
    pub return_type_arity: usize,
    pub return_type: Expr<A>,
    pub cases: RcHashedVec<MatchCase<A>>,
    pub aux_data: A::MatchAux,
}

#[derive(Clone, PartialEq, Eq)]
pub struct MatchCase<A: AstFamily> {
    pub arity: usize,
    pub return_val: Expr<A>,
    pub aux_data: A::MatchCaseAux,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Fun<A: AstFamily> {
    pub decreasing_index: Option<usize>,
    pub param_types: RcHashedVec<Expr<A>>,
    pub return_type: Expr<A>,
    pub return_val: Expr<A>,
    pub aux_data: A::FunAux,
}

#[derive(Clone, PartialEq, Eq)]
pub struct App<A: AstFamily> {
    pub callee: Expr<A>,
    pub args: RcHashedVec<Expr<A>>,
    pub aux_data: A::AppAux,
}

#[derive(Clone, PartialEq, Eq)]
pub struct For<A: AstFamily> {
    pub param_types: RcHashedVec<Expr<A>>,
    pub return_type: Expr<A>,
    pub aux_data: A::ForAux,
}

#[derive(Clone, PartialEq, Eq)]
pub struct DebNode<A: AstFamily> {
    pub deb: Deb,
    pub aux_data: A::DebAux,
}

#[derive(Clone, PartialEq, Eq)]
pub struct UniverseNode<A: AstFamily> {
    pub universe: Universe,
    pub aux_data: A::UniverseAux,
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

impl<A: AstFamily> App<A> {
    pub fn collapse_if_nullary(self) -> Expr<A> {
        if self.args.hashee.is_empty() {
            self.callee
        } else {
            Expr::App(Rc::new(Hashed::new(self)))
        }
    }
}

impl<A: AstFamily> For<A> {
    pub fn collapse_if_nullary(self) -> Expr<A> {
        if self.param_types.hashee.is_empty() {
            self.return_type
        } else {
            Expr::For(Rc::new(Hashed::new(self)))
        }
    }
}

/// Reference-counted hashed.
pub type RcHashed<T> = Rc<Hashed<T>>;

/// Reference-counted hashed vector.
pub type RcHashedVec<T> = RcHashed<Vec<T>>;

pub fn rc_hashed<T: Hash>(t: T) -> RcHashed<T> {
    Rc::new(Hashed::new(t))
}
