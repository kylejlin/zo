use super::*;

use crate::syntax_tree::ost::Span;

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
