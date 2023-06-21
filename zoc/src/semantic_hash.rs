use crate::{ast::*, token::*};

use hmac_sha256::Hash as Sha256;

#[derive(Clone, Debug)]
pub struct Hashed<T> {
    pub value: T,
    pub digest: Digest,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct Digest([u8; 32]);

impl AsRef<[u8]> for Digest {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

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
        // let mut hasher = Sha256::new();

        // hasher.update([discriminator::IND]);

        // hasher.update(&self.name.digest);

        // hasher.update([discriminator::UNIVERSE]);
        // hasher.update(&self.universe_level.to_be_bytes());
        // hasher.update([discriminator::END]);

        // for index_type in self.index_types.iter() {
        //     hasher.update(&index_type.digest());
        // }

        // for constructor_def in self.constructor_defs.iter() {
        //     hasher.update(&constructor_def.param_types.digest());
        //     hasher.update(&constructor_def.index_args.digest());
        // }

        // hasher.update([discriminator::END]);

        // Digest(hasher.finalize())

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

impl SemanticHash for Box<[Expr]> {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256::new();

        hasher.update([discriminator::EXPR_SLICE]);

        for expr in self.iter() {
            hasher.update(expr.digest());
        }

        hasher.update([discriminator::END]);

        Digest(hasher.finalize())
    }
}

impl SemanticHash for Box<[VariantConstructorDef]> {
    fn semantic_hash(&self) -> Digest {
        todo!()
    }
}

mod discriminator {
    pub const IND: u8 = 1;
    pub const VCON: u8 = 2;
    pub const MATCH: u8 = 3;
    pub const FUN: u8 = 4;
    pub const APP: u8 = 5;
    pub const FOR: u8 = 6;
    pub const DEB: u8 = 7;
    pub const UNIVERSE: u8 = 8;

    pub const EXPR_SLICE: u8 = 9;
    pub const VARIANT_CONSTRUCTOR_DEF_SLICE: u8 = 10;

    pub const END: u8 = 64;
}
