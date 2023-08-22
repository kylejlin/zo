use super::*;

use crate::{
    pretty_print::{PrettyPrinted, WithLocationAppended},
    syntax_tree::spanned_ast_to_minimal::SpanRemover,
};

use std::fmt::{Debug, Result as FmtResult};

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = SpanRemover::default().convert(self.clone());
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}

impl Debug for Ind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = SpanRemover::default().convert(Expr::from(self.clone()));
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}

impl Debug for VconDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = SpanRemover::default().convert_vcon_def(self.clone());
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}

impl Debug for Vcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = SpanRemover::default().convert(Expr::from(self.clone()));
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}

impl Debug for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = SpanRemover::default().convert(Expr::from(self.clone()));
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}

impl Debug for MatchCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = SpanRemover::default().convert_match_case(self.clone());
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}

impl Debug for Fun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = SpanRemover::default().convert(Expr::from(self.clone()));
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}

impl Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = SpanRemover::default().convert(Expr::from(self.clone()));
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}

impl Debug for For {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = SpanRemover::default().convert(Expr::from(self.clone()));
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}

impl Debug for DebNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = SpanRemover::default().convert(Expr::from(self.clone()));
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}

impl Debug for UniverseNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let minimal = SpanRemover::default().convert(Expr::from(self.clone()));
        let pretty = minimal.pretty_printed().with_location_appended(self.span());
        write!(f, "{pretty:#}")
    }
}
