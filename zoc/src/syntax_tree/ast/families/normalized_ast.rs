pub use crate::{
    eval::NormalFormMarker, pretty_print::PrettyPrinted, syntax_tree::ast::prelude::*,
};

use std::fmt::{Debug, Result as FmtResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NormalizedUnitAuxDataFamily;

impl ZeroSizedAuxDataFamily for NormalizedUnitAuxDataFamily {}

impl AuxDataFamily for NormalizedUnitAuxDataFamily {
    type Ind = NormalFormMarker;
    type Vcon = NormalFormMarker;
    type Match = NormalFormMarker;
    type Fun = NormalFormMarker;
    type App = NormalFormMarker;
    type For = NormalFormMarker;
    type Deb = NormalFormMarker;
    type Universe = NormalFormMarker;

    type VconDef = NormalFormMarker;
    type MatchCase = NormalFormMarker;
}

pub type Expr = ast::Expr<NormalizedUnitAuxDataFamily>;
pub type Ind = ast::Ind<NormalizedUnitAuxDataFamily>;
pub type VconDef = ast::VconDef<NormalizedUnitAuxDataFamily>;
pub type Vcon = ast::Vcon<NormalizedUnitAuxDataFamily>;
pub type Match = ast::Match<NormalizedUnitAuxDataFamily>;
pub type MatchCase = ast::MatchCase<NormalizedUnitAuxDataFamily>;
pub type Fun = ast::Fun<NormalizedUnitAuxDataFamily>;
pub type App = ast::App<NormalizedUnitAuxDataFamily>;
pub type For = ast::For<NormalizedUnitAuxDataFamily>;
pub type DebNode = ast::DebNode<NormalizedUnitAuxDataFamily>;
pub type UniverseNode = ast::UniverseNode<NormalizedUnitAuxDataFamily>;

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
