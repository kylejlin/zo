use std::fmt::{Display, Formatter, Result as FmtResult};

pub const SOFT_TAB: &str = "    ";

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PrettyPrinted<'a, T>(pub &'a T);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Indentation {
    pub soft_tab_count: usize,
}

impl Display for Indentation {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for _ in 0..self.soft_tab_count {
            f.write_str(SOFT_TAB)?;
        }
        Ok(())
    }
}

mod impl_for_ast;
