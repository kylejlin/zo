pub use crate::syntax_tree::ast::prelude::*;

use crate::pretty_print::{PrettyPrinted, SimplyPrintableAstFamily};

use std::fmt::{Debug, Result as FmtResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MinimalAst;

impl SimplyPrintableAstFamily for MinimalAst {}

impl AstFamily for MinimalAst {
    type IndAux = ();
    type VconAux = ();
    type MatchAux = ();
    type FunAux = ();
    type AppAux = ();
    type ForAux = ();
    type DebAux = ();
    type UniverseAux = ();

    type VconDefAux = ();
    type MatchCaseAux = ();
}

pub type Expr = ast::Expr<MinimalAst>;
pub type Ind = ast::Ind<MinimalAst>;
pub type VconDef = ast::VconDef<MinimalAst>;
pub type Vcon = ast::Vcon<MinimalAst>;
pub type Match = ast::Match<MinimalAst>;
pub type MatchCase = ast::MatchCase<MinimalAst>;
pub type Fun = ast::Fun<MinimalAst>;
pub type App = ast::App<MinimalAst>;
pub type For = ast::For<MinimalAst>;
pub type DebNode = ast::DebNode<MinimalAst>;
pub type UniverseNode = ast::UniverseNode<MinimalAst>;

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
