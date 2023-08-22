pub use crate::syntax_tree::ast::prelude::*;

mod debug;
mod span;

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
