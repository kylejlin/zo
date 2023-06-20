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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct UniverseLiteral {
    pub level: usize,
    pub start: ByteIndex,
}

pub use crate::parser::Token;
