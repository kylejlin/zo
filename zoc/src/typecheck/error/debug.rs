use super::*;

use std::fmt::{Debug, Result as FmtResult};

impl Debug for TypeError<UnitAuxDataFamily> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let pretty = self.pretty_printed();
        write!(f, "{pretty:#}")
    }
}

impl Debug for TypeError<SpanAuxData> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        let pretty = self.pretty_printed();
        write!(f, "{pretty:#}")
    }
}
