pub use crate::syntax_tree::ast::prelude::*;

mod debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct UnitAuxData;

impl AuxDataFamily for UnitAuxData {
    type Ind = ();
    type Vcon = ();
    type Match = ();
    type Fun = ();
    type App = ();
    type For = ();
    type Deb = ();
    type Universe = ();

    type VconDef = ();
    type MatchCase = ();
}

pub type Expr = ast::Expr<UnitAuxData>;
pub type Ind = ast::Ind<UnitAuxData>;
pub type VconDef = ast::VconDef<UnitAuxData>;
pub type Vcon = ast::Vcon<UnitAuxData>;
pub type Match = ast::Match<UnitAuxData>;
pub type MatchCase = ast::MatchCase<UnitAuxData>;
pub type Fun = ast::Fun<UnitAuxData>;
pub type App = ast::App<UnitAuxData>;
pub type For = ast::For<UnitAuxData>;
pub type DebNode = ast::DebNode<UnitAuxData>;
pub type UniverseNode = ast::UniverseNode<UnitAuxData>;
