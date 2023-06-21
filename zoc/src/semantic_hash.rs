use crate::{ast::*, token::*};

#[derive(Clone, Debug)]
pub struct Hashed<T> {
    pub value: T,
    pub digest: Digest,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct Digest([u64; 4]);

pub trait SemanticHash {
    fn semantic_hash(&self) -> Digest;
}

impl<T> Hashed<T>
where
    T: SemanticHash,
{
    pub fn new(value: T) -> Self {
        Self {
            digest: value.semantic_hash(),
            value,
        }
    }
}

impl SemanticHash for Ind {
    fn semantic_hash(&self) -> Digest {
        todo!()
    }
}

impl SemanticHash for Vcon {
    fn semantic_hash(&self) -> Digest {
        todo!()
    }
}

impl SemanticHash for Match {
    fn semantic_hash(&self) -> Digest {
        todo!()
    }
}

impl SemanticHash for Fun {
    fn semantic_hash(&self) -> Digest {
        todo!()
    }
}

impl SemanticHash for App {
    fn semantic_hash(&self) -> Digest {
        todo!()
    }
}

impl SemanticHash for For {
    fn semantic_hash(&self) -> Digest {
        todo!()
    }
}

impl SemanticHash for NumberLiteral {
    fn semantic_hash(&self) -> Digest {
        todo!()
    }
}

impl SemanticHash for StringLiteral {
    fn semantic_hash(&self) -> Digest {
        todo!()
    }
}

impl SemanticHash for UniverseLiteral {
    fn semantic_hash(&self) -> Digest {
        todo!()
    }
}
