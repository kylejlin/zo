use crate::ast::*;

// #[derive(Clone, Debug)]
// pub enum Expr {
//     Ind(Rc<Hashed<Ind>>),
//     Vcon(Rc<Hashed<Vcon>>),
//     Match(Rc<Hashed<Match>>),
//     Fun(Rc<Hashed<Fun>>),
//     App(Rc<Hashed<App>>),
//     For(Rc<Hashed<For>>),
//     Deb(Rc<Hashed<Deb>>),
//     Universe(Rc<Hashed<Universe>>),
// }

// impl Expr {
//     pub fn digest(&self) -> &Digest {
//         match self {
//             Expr::Ind(e) => &e.digest,
//             Expr::Vcon(e) => &e.digest,
//             Expr::Match(e) => &e.digest,
//             Expr::Fun(e) => &e.digest,
//             Expr::App(e) => &e.digest,
//             Expr::For(e) => &e.digest,
//             Expr::Deb(e) => &e.digest,
//             Expr::Universe(e) => &e.digest,
//         }
//     }
// }

// pub use crate::semantic_hash::*;

// impl Expr {
//     pub fn try_into_ind(self) -> Result<Rc<Hashed<Ind>>, Self> {
//         match self {
//             Expr::Ind(e) => Ok(e),
//             _ => Err(self),
//         }
//     }

//     pub fn try_into_vcon(self) -> Result<Rc<Hashed<Vcon>>, Self> {
//         match self {
//             Expr::Vcon(e) => Ok(e),
//             _ => Err(self),
//         }
//     }

//     pub fn try_into_match(self) -> Result<Rc<Hashed<Match>>, Self> {
//         match self {
//             Expr::Match(e) => Ok(e),
//             _ => Err(self),
//         }
//     }

//     pub fn try_into_fun(self) -> Result<Rc<Hashed<Fun>>, Self> {
//         match self {
//             Expr::Fun(e) => Ok(e),
//             _ => Err(self),
//         }
//     }

//     pub fn try_into_app(self) -> Result<Rc<Hashed<App>>, Self> {
//         match self {
//             Expr::App(e) => Ok(e),
//             _ => Err(self),
//         }
//     }

//     pub fn try_into_for(self) -> Result<Rc<Hashed<For>>, Self> {
//         match self {
//             Expr::For(e) => Ok(e),
//             _ => Err(self),
//         }
//     }

//     pub fn try_into_deb(self) -> Result<Rc<Hashed<Deb>>, Self> {
//         match self {
//             Expr::Deb(e) => Ok(e),
//             _ => Err(self),
//         }
//     }

//     pub fn try_into_universe(self) -> Result<Rc<Hashed<Universe>>, Self> {
//         match self {
//             Expr::Universe(e) => Ok(e),
//             _ => Err(self),
//         }
//     }
// }

// #[derive(Clone, Debug)]
// pub struct Ind {
//     pub name: Rc<StringValue>,
//     pub universe_level: usize,
//     pub index_types: Rc<Hashed<Box<[Expr]>>>,
//     pub vcon_defs: Rc<Hashed<Box<[VconDef]>>>,
// }

// #[derive(Clone, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
// pub struct StringValue(pub String);

// #[derive(Debug, Clone)]
// pub struct VconDef {
//     pub param_types: Rc<Hashed<Box<[Expr]>>>,
//     pub index_args: Rc<Hashed<Box<[Expr]>>>,
// }

// #[derive(Debug, Clone)]
// pub struct Vcon {
//     pub ind: Rc<Hashed<Ind>>,
//     pub vcon_index: usize,
// }

// #[derive(Debug, Clone)]
// pub struct Match {
//     pub matchee: Expr,
//     pub return_type: Expr,
//     pub cases: Rc<Hashed<Box<[MatchCase]>>>,
// }

// #[derive(Debug, Clone)]
// pub struct MatchCase {
//     pub arity: usize,
//     pub return_val: Expr,
// }

// #[derive(Debug, Clone)]
// pub struct Fun {
//     pub decreasing_index: Option<usize>,
//     pub param_types: Rc<Hashed<Box<[Expr]>>>,
//     pub return_type: Expr,
//     pub return_val: Expr,
// }

// #[derive(Debug, Clone)]
// pub struct App {
//     pub callee: Expr,
//     pub args: Rc<Hashed<Box<[Expr]>>>,
// }

// #[derive(Debug, Clone)]
// pub struct For {
//     pub param_types: Rc<Hashed<Box<[Expr]>>>,
//     pub return_type: Expr,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
// pub struct Deb(pub usize);

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
// pub struct Universe {
//     pub level: usize,
// }

use std::fmt::{Debug, Formatter};

pub struct Abbrev<T>(pub T);

impl<T> Debug for Abbrev<T>
where
    T: AbbrevDebug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.afmt(f)
    }
}

trait AbbrevDebug {
    fn afmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

impl AbbrevDebug for Expr {
    fn afmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Ind(e) => e.value.afmt(f),
            Expr::Vcon(e) => e.value.afmt(f),
            Expr::Match(e) => e.value.afmt(f),
            Expr::Fun(e) => e.value.afmt(f),
            Expr::App(e) => e.value.afmt(f),
            Expr::For(e) => e.value.afmt(f),
            Expr::Deb(e) => write!(f, "{}", e.value.0),
            Expr::Universe(e) => write!(f, "Type{}", e.value.level),
        }
    }
}

impl AbbrevDebug for Ind {
    fn afmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.0)
    }
}

impl AbbrevDebug for Vcon {
    fn afmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", self.ind.value.name.0, self.vcon_index)
    }
}

impl AbbrevDebug for Match {
    fn afmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("match")
            .field("matchee", &Abbrev(self.matchee.clone()))
            .field("return_type", &Abbrev(self.return_type.clone()))
            .field("cases", &Abbrev(&self.cases.value[..]))
            .finish()
    }
}

impl AbbrevDebug for &'_ [MatchCase] {
    fn afmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.iter().cloned().map(Abbrev))
            .finish()
    }
}

impl AbbrevDebug for MatchCase {
    fn afmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("case")
            .field("arity", &self.arity)
            .field("return_val", &Abbrev(self.return_val.clone()))
            .finish()
    }
}

impl AbbrevDebug for Fun {
    fn afmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("fun")
            .field("decreasing_index", &self.decreasing_index)
            .field("param_types", &Abbrev(&self.param_types.value[..]))
            .field("return_type", &Abbrev(self.return_type.clone()))
            .field("return_val", &Abbrev(self.return_val.clone()))
            .finish()
    }
}

impl AbbrevDebug for App {
    fn afmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("app")
            .field("callee", &Abbrev(self.callee.clone()))
            .field("args", &Abbrev(&self.args.value[..]))
            .finish()
    }
}

impl AbbrevDebug for For {
    fn afmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("for")
            .field("param_types", &Abbrev(&self.param_types.value[..]))
            .field("return_type", &Abbrev(self.return_type.clone()))
            .finish()
    }
}

impl AbbrevDebug for &'_ [Expr] {
    fn afmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.iter().cloned().map(Abbrev))
            .finish()
    }
}
