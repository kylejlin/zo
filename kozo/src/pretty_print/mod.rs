use std::fmt::{Display, Formatter, Result as FmtResult};

pub const SOFT_TAB_SIZE: usize = 4;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PrettyPrinted<'a, T>(pub &'a T);

mod impl_for_ast;
