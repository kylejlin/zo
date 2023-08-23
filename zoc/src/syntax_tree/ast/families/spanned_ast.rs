pub use crate::syntax_tree::ast::prelude::*;

use crate::{
    pretty_print::{PrettyPrinted, WithLocationAppended},
    syntax_tree::remove_ast_aux_data::AuxDataRemover,
};

use std::fmt::{Debug, Result as FmtResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SpanAuxData;

pub type Expr = ast::Expr<SpanAuxData>;
pub type Ind = ast::Ind<SpanAuxData>;
pub type VconDef = ast::VconDef<SpanAuxData>;
pub type Vcon = ast::Vcon<SpanAuxData>;
pub type Match = ast::Match<SpanAuxData>;
pub type MatchCase = ast::MatchCase<SpanAuxData>;
pub type Fun = ast::Fun<SpanAuxData>;
pub type App = ast::App<SpanAuxData>;
pub type For = ast::For<SpanAuxData>;
pub type DebNode = ast::DebNode<SpanAuxData>;
pub type UniverseNode = ast::UniverseNode<SpanAuxData>;

impl AuxDataFamily for SpanAuxData {
    type Ind = IndSpans;
    type Vcon = VconSpans;
    type Match = MatchSpans;
    type Fun = FunSpans;
    type App = Span;
    type For = ForSpans;
    type Deb = Span;
    type Universe = Span;

    type VconDef = VconDefSpans;
    type MatchCase = MatchCaseSpans;
}

#[derive(Debug, Clone, Hash)]
pub struct IndSpans {
    pub span: Span,
    pub universe_span: Span,
    pub name_span: Span,
    pub index_types_span: Span,
    pub vcon_defs_span: Span,
}

#[derive(Debug, Clone, Hash)]
pub struct VconSpans {
    pub span: Span,
    pub vcon_index_span: Span,
}

#[derive(Debug, Clone, Hash)]
pub struct VconDefSpans {
    pub span: Span,
    pub param_types_span: Span,
    pub index_args_span: Span,
}

#[derive(Debug, Clone, Hash)]
pub struct MatchSpans {
    pub span: Span,
    pub return_type_arity_span: Span,
    pub cases_span: Span,
}

#[derive(Debug, Clone, Hash)]
pub struct MatchCaseSpans {
    pub span: Span,
    pub arity_span: Span,
}

#[derive(Debug, Clone, Hash)]
pub struct FunSpans {
    pub span: Span,
    pub decreasing_index_span: Span,
    pub param_types_span: Span,
}

#[derive(Debug, Clone, Hash)]
pub struct ForSpans {
    pub span: Span,
    pub param_types_span: Span,
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Self::Ind(e) => e.hashee.aux_data.span,
            Self::Vcon(e) => e.hashee.aux_data.span,
            Self::Match(e) => e.hashee.aux_data.span,
            Self::Fun(e) => e.hashee.aux_data.span,
            Self::App(e) => e.hashee.aux_data,
            Self::For(e) => e.hashee.aux_data.span,
            Self::Deb(e) => e.hashee.aux_data,
            Self::Universe(e) => e.hashee.aux_data,
        }
    }
}
impl Ind {
    pub fn span(&self) -> Span {
        self.aux_data.span
    }
}
impl VconDef {
    pub fn span(&self) -> Span {
        self.aux_data.span
    }
}
impl Vcon {
    pub fn span(&self) -> Span {
        self.aux_data.span
    }
}
impl Match {
    pub fn span(&self) -> Span {
        self.aux_data.span
    }
}
impl MatchCase {
    pub fn span(&self) -> Span {
        self.aux_data.span
    }
}
impl Fun {
    pub fn span(&self) -> Span {
        self.aux_data.span
    }
}
impl App {
    pub fn span(&self) -> Span {
        self.aux_data
    }
}
impl For {
    pub fn span(&self) -> Span {
        self.aux_data.span
    }
}
impl DebNode {
    pub fn span(&self) -> Span {
        self.aux_data
    }
}
impl UniverseNode {
    pub fn span(&self) -> Span {
        self.aux_data
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = AuxDataRemover::default().convert(self.clone());
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}
impl Debug for Ind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = AuxDataRemover::default().convert(Expr::from(self.clone()));
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}
impl Debug for VconDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = AuxDataRemover::default().convert_vcon_def(self.clone());
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}
impl Debug for Vcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = AuxDataRemover::default().convert(Expr::from(self.clone()));
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}
impl Debug for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = AuxDataRemover::default().convert(Expr::from(self.clone()));
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}
impl Debug for MatchCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = AuxDataRemover::default().convert_match_case(self.clone());
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}
impl Debug for Fun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = AuxDataRemover::default().convert(Expr::from(self.clone()));
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}
impl Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = AuxDataRemover::default().convert(Expr::from(self.clone()));
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}
impl Debug for For {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = AuxDataRemover::default().convert(Expr::from(self.clone()));
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}
impl Debug for DebNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = AuxDataRemover::default().convert(Expr::from(self.clone()));
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}
impl Debug for UniverseNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = AuxDataRemover::default().convert(Expr::from(self.clone()));
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}
