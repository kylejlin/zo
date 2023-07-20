use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct ByteIndex(pub usize);

impl Add for ByteIndex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ByteIndex(self.0 + rhs.0)
    }
}

impl Sub for ByteIndex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        ByteIndex(self.0 - rhs.0)
    }
}

pub type Span = (ByteIndex, ByteIndex);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct NumberLiteral {
    pub value: usize,
    pub span: (ByteIndex, ByteIndex),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct StringLiteral {
    pub value: String,
    pub span: (ByteIndex, ByteIndex),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RewriteLiteral {
    pub value: Rewrite,
    pub start: ByteIndex,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Rewrite {
    pub direction: RewriteDirection,
    pub econ_index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RewriteDirection {
    Ltr,
    Rtl,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct UniverseLiteral {
    pub level: usize,
    pub start: ByteIndex,
}

pub use crate::syntax_tree::parser::Token;
