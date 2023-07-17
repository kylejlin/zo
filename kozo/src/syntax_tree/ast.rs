use std::{hash::Hash, rc::Rc};

pub use crate::hash::*;

/// Reference-counted semantically hashed.
pub type RcSemHashed<T> = Rc<Hashed<T>>;

/// Reference-counted semantically hashed vector.
pub type RcSemHashedVec<T> = Rc<Hashed<Vec<T>>>;

pub fn rc_sem_hashed<T: Hash>(t: T) -> RcSemHashed<T> {
    Rc::new(Hashed::new(t))
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expr {
    Ind(RcSemHashed<Ind>),
    Vcon(RcSemHashed<Vcon>),
    Match(RcSemHashed<Match>),
    Fun(RcSemHashed<Fun>),
    App(RcSemHashed<App>),
    For(RcSemHashed<For>),
    Deb(RcSemHashed<DebNode>),
    Universe(RcSemHashed<UniverseNode>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ind {
    pub name: Rc<StringValue>,
    pub universe_level: UniverseLevel,
    pub index_types: RcSemHashedVec<Expr>,
    pub vcon_defs: RcSemHashedVec<VconDef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct StringValue(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VconDef {
    pub param_types: RcSemHashedVec<Expr>,
    pub index_args: RcSemHashedVec<Expr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vcon {
    pub ind: RcSemHashed<Ind>,
    pub vcon_index: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Match {
    pub matchee: Expr,
    pub return_type: Expr,
    pub cases: RcSemHashedVec<MatchCase>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MatchCase {
    Dismissed,
    Nondismissed(NondismissedMatchCase),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NondismissedMatchCase {
    pub arity: usize,
    pub return_val: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fun {
    pub decreasing_index: Option<usize>,
    pub param_types: RcSemHashedVec<Expr>,
    pub return_type: Expr,
    pub return_val: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct App {
    pub callee: Expr,
    pub args: RcSemHashedVec<Expr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct For {
    pub param_types: RcSemHashedVec<Expr>,
    pub return_type: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DebNode {
    pub deb: Deb,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UniverseNode {
    pub level: UniverseLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Deb(pub usize);

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

impl GetDigest for Expr {
    fn digest(&self) -> &Digest {
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
impl GetDigest for RcSemHashed<Ind> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
impl GetDigest for RcSemHashed<Vcon> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
impl GetDigest for RcSemHashed<Match> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
impl GetDigest for RcSemHashed<Fun> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
impl GetDigest for RcSemHashed<App> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
impl GetDigest for RcSemHashed<For> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
impl GetDigest for RcSemHashed<DebNode> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}
impl GetDigest for RcSemHashed<UniverseNode> {
    fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl From<RcSemHashed<Ind>> for Expr {
    fn from(ind: RcSemHashed<Ind>) -> Self {
        Expr::Ind(ind)
    }
}
impl From<RcSemHashed<Vcon>> for Expr {
    fn from(vcon: RcSemHashed<Vcon>) -> Self {
        Expr::Vcon(vcon)
    }
}
impl From<RcSemHashed<Match>> for Expr {
    fn from(match_: RcSemHashed<Match>) -> Self {
        Expr::Match(match_)
    }
}
impl From<RcSemHashed<Fun>> for Expr {
    fn from(fun: RcSemHashed<Fun>) -> Self {
        Expr::Fun(fun)
    }
}
impl From<RcSemHashed<App>> for Expr {
    fn from(app: RcSemHashed<App>) -> Self {
        Expr::App(app)
    }
}
impl From<RcSemHashed<For>> for Expr {
    fn from(for_: RcSemHashed<For>) -> Self {
        Expr::For(for_)
    }
}
impl From<RcSemHashed<DebNode>> for Expr {
    fn from(deb: RcSemHashed<DebNode>) -> Self {
        Expr::Deb(deb)
    }
}
impl From<RcSemHashed<UniverseNode>> for Expr {
    fn from(universe: RcSemHashed<UniverseNode>) -> Self {
        Expr::Universe(universe)
    }
}

impl From<Ind> for Expr {
    fn from(ind: Ind) -> Self {
        rc_sem_hashed(ind).into()
    }
}
impl From<Vcon> for Expr {
    fn from(vcon: Vcon) -> Self {
        rc_sem_hashed(vcon).into()
    }
}
impl From<Match> for Expr {
    fn from(match_: Match) -> Self {
        rc_sem_hashed(match_).into()
    }
}
impl From<Fun> for Expr {
    fn from(fun: Fun) -> Self {
        rc_sem_hashed(fun).into()
    }
}
impl From<App> for Expr {
    fn from(app: App) -> Self {
        rc_sem_hashed(app).into()
    }
}
impl From<For> for Expr {
    fn from(for_: For) -> Self {
        rc_sem_hashed(for_).into()
    }
}
impl From<DebNode> for Expr {
    fn from(deb: DebNode) -> Self {
        rc_sem_hashed(deb).into()
    }
}
impl From<UniverseNode> for Expr {
    fn from(universe: UniverseNode) -> Self {
        rc_sem_hashed(universe).into()
    }
}

impl Expr {
    pub fn try_into_ind(self) -> Result<RcSemHashed<Ind>, Self> {
        match self {
            Expr::Ind(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_vcon(self) -> Result<RcSemHashed<Vcon>, Self> {
        match self {
            Expr::Vcon(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_match(self) -> Result<RcSemHashed<Match>, Self> {
        match self {
            Expr::Match(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_fun(self) -> Result<RcSemHashed<Fun>, Self> {
        match self {
            Expr::Fun(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_app(self) -> Result<RcSemHashed<App>, Self> {
        match self {
            Expr::App(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_for(self) -> Result<RcSemHashed<For>, Self> {
        match self {
            Expr::For(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_deb(self) -> Result<RcSemHashed<DebNode>, Self> {
        match self {
            Expr::Deb(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub fn try_into_universe(self) -> Result<RcSemHashed<UniverseNode>, Self> {
        match self {
            Expr::Universe(e) => Ok(e),
            _ => Err(self),
        }
    }
}
