use std::{fmt::Debug, hash::Hash, rc::Rc};

pub use crate::hash::*;

mod conversion;
mod get_digest;
mod hash;

pub mod families;

pub mod node_path;
pub use node_path::{NodeEdge, NodePath};

pub mod prelude;

pub trait AuxDataFamily:
    'static + Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash + Default
{
    type Ind: Clone + Hash;
    type Vcon: Clone + Hash;
    type Match: Clone + Hash;
    type Fun: Clone + Hash;
    type App: Clone + Hash;
    type For: Clone + Hash;
    type Deb: Clone + Hash;
    type Universe: Clone + Hash;

    type VconDef: Clone + Hash;
    type MatchCase: Clone + Hash;
}

/// This marker trait can safely be implemented for `A`
/// if and only if
/// `<A as AuxDataFamily>::Ind` is zero-sized and
/// `<A as AuxDataFamily>::Vcon` is zero-sized and
/// `<A as AuxDataFamily>::Match` is zero-sized and
/// `<A as AuxDataFamily>::Fun` is zero-sized and ... etc.
pub trait ZeroSizedAuxDataFamily: AuxDataFamily {}

#[derive(Clone, PartialEq, Eq)]
pub enum Expr<A: AuxDataFamily> {
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
pub struct Ind<A: AuxDataFamily> {
    pub name: Rc<StringValue>,
    pub universe: Universe,
    pub index_types: RcHashedVec<Expr<A>>,
    pub vcon_defs: RcHashedVec<VconDef<A>>,
    pub aux_data: A::Ind,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct StringValue(pub String);

#[derive(Clone, PartialEq, Eq)]
pub struct VconDef<A: AuxDataFamily> {
    pub param_types: RcHashedVec<Expr<A>>,
    pub index_args: RcHashedVec<Expr<A>>,
    pub aux_data: A::VconDef,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Vcon<A: AuxDataFamily> {
    pub ind: RcHashed<Ind<A>>,
    pub vcon_index: usize,
    pub aux_data: A::Vcon,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Match<A: AuxDataFamily> {
    pub matchee: Expr<A>,
    pub return_type_arity: usize,
    pub return_type: Expr<A>,
    pub cases: RcHashedVec<MatchCase<A>>,
    pub aux_data: A::Match,
}

#[derive(Clone, PartialEq, Eq)]
pub struct MatchCase<A: AuxDataFamily> {
    pub arity: usize,
    pub return_val: Expr<A>,
    pub aux_data: A::MatchCase,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Fun<A: AuxDataFamily> {
    pub decreasing_index: Option<usize>,
    pub param_types: RcHashedVec<Expr<A>>,
    pub return_type: Expr<A>,
    pub return_val: Expr<A>,
    pub aux_data: A::Fun,
}

#[derive(Clone, PartialEq, Eq)]
pub struct App<A: AuxDataFamily> {
    pub callee: Expr<A>,
    pub args: RcHashedVec<Expr<A>>,
    pub aux_data: A::App,
}

#[derive(Clone, PartialEq, Eq)]
pub struct For<A: AuxDataFamily> {
    pub param_types: RcHashedVec<Expr<A>>,
    pub return_type: Expr<A>,
    pub aux_data: A::For,
}

#[derive(Clone, PartialEq, Eq)]
pub struct DebNode<A: AuxDataFamily> {
    pub deb: Deb,
    pub aux_data: A::Deb,
}

#[derive(Clone, PartialEq, Eq)]
pub struct UniverseNode<A: AuxDataFamily> {
    pub universe: Universe,
    pub aux_data: A::Universe,
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

impl<A: AuxDataFamily> App<A> {
    pub fn collapse_if_nullary(self) -> Expr<A> {
        if self.args.hashee.is_empty() {
            self.callee
        } else {
            Expr::App(Rc::new(Hashed::new(self)))
        }
    }
}

impl<A: AuxDataFamily> For<A> {
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
