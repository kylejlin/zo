#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct ByteIndex(pub usize);

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
