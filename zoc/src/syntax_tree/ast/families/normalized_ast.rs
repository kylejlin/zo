pub use crate::{
    eval::NormalFormMarker,
    pretty_print::{PrettyPrinted, SimplyPrintableAstFamily},
    syntax_tree::ast::prelude::*,
};

use std::fmt::{Debug, Result as FmtResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NormalizedAst;

impl SimplyPrintableAstFamily for NormalizedAst {}

impl AstFamily for NormalizedAst {
    type IndAux = NormalFormMarker;
    type VconAux = NormalFormMarker;
    type MatchAux = NormalFormMarker;
    type FunAux = NormalFormMarker;
    type AppAux = NormalFormMarker;
    type ForAux = NormalFormMarker;
    type DebAux = NormalFormMarker;
    type UniverseAux = NormalFormMarker;

    type VconDefAux = NormalFormMarker;
    type MatchCaseAux = NormalFormMarker;
}

pub type Expr = ast::Expr<NormalizedAst>;
pub type Ind = ast::Ind<NormalizedAst>;
pub type VconDef = ast::VconDef<NormalizedAst>;
pub type Vcon = ast::Vcon<NormalizedAst>;
pub type Match = ast::Match<NormalizedAst>;
pub type MatchCase = ast::MatchCase<NormalizedAst>;
pub type Fun = ast::Fun<NormalizedAst>;
pub type App = ast::App<NormalizedAst>;
pub type For = ast::For<NormalizedAst>;
pub type DebNode = ast::DebNode<NormalizedAst>;
pub type UniverseNode = ast::UniverseNode<NormalizedAst>;

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
