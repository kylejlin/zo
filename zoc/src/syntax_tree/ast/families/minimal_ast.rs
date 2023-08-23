pub use crate::syntax_tree::ast::prelude::*;

use crate::pretty_print::PrettyPrinted;

use std::fmt::{Debug, Result as FmtResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct UnitAuxDataFamily;

impl ZeroSizedAuxDataFamily for UnitAuxDataFamily {}

impl AuxDataFamily for UnitAuxDataFamily {
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

pub type Expr = ast::Expr<UnitAuxDataFamily>;
pub type Ind = ast::Ind<UnitAuxDataFamily>;
pub type VconDef = ast::VconDef<UnitAuxDataFamily>;
pub type Vcon = ast::Vcon<UnitAuxDataFamily>;
pub type Match = ast::Match<UnitAuxDataFamily>;
pub type MatchCase = ast::MatchCase<UnitAuxDataFamily>;
pub type Fun = ast::Fun<UnitAuxDataFamily>;
pub type App = ast::App<UnitAuxDataFamily>;
pub type For = ast::For<UnitAuxDataFamily>;
pub type DebNode = ast::DebNode<UnitAuxDataFamily>;
pub type UniverseNode = ast::UniverseNode<UnitAuxDataFamily>;

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let pretty = self.pretty_printed();
        write!(f, "{pretty:#}")
    }
}
impl Debug for Ind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let pretty = self.pretty_printed();
        write!(f, "{pretty:#}")
    }
}
impl Debug for VconDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let pretty = self.pretty_printed();
        write!(f, "{pretty:#}")
    }
}
impl Debug for Vcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let pretty = self.pretty_printed();
        write!(f, "{pretty:#}")
    }
}
impl Debug for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let pretty = self.pretty_printed();
        write!(f, "{pretty:#}")
    }
}
impl Debug for MatchCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let pretty = self.pretty_printed();
        write!(f, "{pretty:#}")
    }
}
impl Debug for Fun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let pretty = self.pretty_printed();
        write!(f, "{pretty:#}")
    }
}
impl Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let pretty = self.pretty_printed();
        write!(f, "{pretty:#}")
    }
}
impl Debug for For {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let pretty = self.pretty_printed();
        write!(f, "{pretty:#}")
    }
}
impl Debug for DebNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let pretty = self.pretty_printed();
        write!(f, "{pretty:#}")
    }
}
impl Debug for UniverseNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let pretty = self.pretty_printed();
        write!(f, "{pretty:#}")
    }
}
