use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    rc::Rc,
};

pub const SOFT_TAB: &str = "    ";

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PrettyPrinted<'a, T>(pub &'a T);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Indentation {
    pub soft_tab_count: usize,
}

impl Indentation {
    pub fn incremented(self) -> Self {
        Self {
            soft_tab_count: self.soft_tab_count + 1,
        }
    }
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
