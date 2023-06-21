use crate::ast::*;

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
        let mut hasher = Sha256::new();

        hasher.update([discriminator::IND]);

        hasher.update(&self.name.0);

        hasher.update([discriminator::IND_UNIVERSE]);
        hasher.update(&self.universe_level.to_be_bytes());
        hasher.update([discriminator::END]);

        hasher.update(&self.index_types.digest);

        hasher.update(&self.constructor_defs.digest);

        hasher.update([discriminator::END]);

        Digest(hasher.finalize())
    }
}

impl SemanticHash for Vcon {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256::new();

        hasher.update([discriminator::VCON]);

        hasher.update(&self.ind.digest);

        hasher.update([discriminator::VCON_INDEX]);
        hasher.update(&self.vcon_index.to_be_bytes());
        hasher.update([discriminator::END]);

        hasher.update([discriminator::END]);

        Digest(hasher.finalize())
    }
}

impl SemanticHash for Match {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256::new();

        hasher.update([discriminator::MATCH]);

        hasher.update(self.matchee.digest());
        hasher.update(self.return_type.digest());
        hasher.update(&self.cases.digest);

        hasher.update([discriminator::END]);

        Digest(hasher.finalize())
    }
}

impl SemanticHash for Fun {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256::new();

        hasher.update([discriminator::FUN]);

        if let Some(i) = self.decreasing_index {
            hasher.update([discriminator::SOME]);
            hasher.update(i.to_be_bytes());
        } else {
            hasher.update([discriminator::NONE]);
            hasher.update(0usize.to_be_bytes());
        }

        hasher.update(&self.param_types.digest);
        hasher.update(self.return_type.digest());
        hasher.update(self.return_val.digest());

        hasher.update([discriminator::END]);

        Digest(hasher.finalize())
    }
}

impl SemanticHash for App {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256::new();

        hasher.update([discriminator::APP]);

        hasher.update(self.callee.digest());
        hasher.update(&self.args.digest);

        hasher.update([discriminator::END]);

        Digest(hasher.finalize())
    }
}

impl SemanticHash for For {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256::new();

        hasher.update([discriminator::FOR]);

        hasher.update(&self.param_types.digest);
        hasher.update(self.return_type.digest());

        hasher.update([discriminator::END]);

        Digest(hasher.finalize())
    }
}

impl SemanticHash for Deb {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256::new();

        hasher.update([discriminator::DEB]);

        hasher.update(self.0.to_be_bytes());

        hasher.update([discriminator::END]);

        Digest(hasher.finalize())
    }
}

impl SemanticHash for Universe {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256::new();

        hasher.update([discriminator::UNIVERSE]);

        hasher.update(self.level.to_be_bytes());

        hasher.update([discriminator::END]);

        Digest(hasher.finalize())
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

impl SemanticHash for Box<[Hashed<VariantConstructorDef>]> {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256::new();

        hasher.update([discriminator::VARIANT_CONSTRUCTOR_DEF_SLICE]);

        for def in self.iter() {
            hasher.update(&def.digest);
        }

        hasher.update([discriminator::END]);

        Digest(hasher.finalize())
    }
}

impl SemanticHash for VariantConstructorDef {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256::new();

        hasher.update([discriminator::VARIANT_CONSTRUCTOR_DEF]);

        hasher.update(&self.param_types.digest);
        hasher.update(&self.index_args.digest);

        hasher.update([discriminator::END]);

        Digest(hasher.finalize())
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
    pub const VARIANT_CONSTRUCTOR_DEF: u8 = 11;

    pub const VCON_INDEX: u8 = 12;

    pub const SOME: u8 = 13;
    pub const NONE: u8 = 14;

    pub const IND_UNIVERSE: u8 = 15;

    pub const END: u8 = 64;
}
