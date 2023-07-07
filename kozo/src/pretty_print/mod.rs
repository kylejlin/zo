use std::fmt::{Display, Formatter, Result as FmtResult};

pub const SOFT_TAB_SIZE: usize = 4;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PrettyPrinted<'a, T>(pub &'a T);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct SoftTabCount(pub usize);

fn get_indent_str(count: SoftTabCount) -> String {
    " ".repeat(count.0 * SOFT_TAB_SIZE)
}

mod impl_for_ast;
