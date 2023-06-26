use crate::ast::*;

use hmac_sha256::Hash as Sha256;

use std::{
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
};

#[derive(Clone, Debug)]
pub struct SemanticHashed<T> {
    pub value: T,
    pub digest: Digest,
}

#[derive(Clone, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct Digest([u8; 32]);

impl AsRef<[u8]> for Digest {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Hash for Digest {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_u64(u64::from_ne_bytes([
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6], self.0[7],
        ]));
    }
}

impl Debug for Digest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x")?;
        for byte in self.0.iter() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl nohash_hasher::IsEnabled for Digest {}

pub trait GetDigest {
    fn digest(&self) -> &Digest;
}

pub trait SemanticHash {
    fn semantic_hash(&self) -> Digest;
}

impl<T> SemanticHashed<T>
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
        hasher.update(&self.universe_level.0.to_be_bytes());
        hasher.update([discriminator::END]);

        hasher.update(&self.index_types.digest);

        hasher.update(&self.vcon_defs.digest);

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

impl SemanticHash for DebNode {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256::new();

        hasher.update([discriminator::DEB]);

        hasher.update(self.deb.0.to_be_bytes());

        hasher.update([discriminator::END]);

        Digest(hasher.finalize())
    }
}

impl SemanticHash for UniverseNode {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256::new();

        hasher.update([discriminator::UNIVERSE]);

        hasher.update(self.level.0.to_be_bytes());

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

impl SemanticHash for Box<[VconDef]> {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256::new();

        hasher.update([discriminator::VARIANT_CONSTRUCTOR_DEF_SLICE]);

        for def in self.iter() {
            hasher.update([discriminator::VARIANT_CONSTRUCTOR_DEF]);

            hasher.update(&def.param_types.digest);
            hasher.update(&def.index_args.digest);

            hasher.update([discriminator::END]);
        }

        hasher.update([discriminator::END]);

        Digest(hasher.finalize())
    }
}

impl SemanticHash for Box<[MatchCase]> {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256::new();

        hasher.update([discriminator::MATCH_CASE_SLICE]);

        for case in self.iter() {
            hasher.update([discriminator::MATCH_CASE]);

            hasher.update(case.arity.to_be_bytes());
            hasher.update(case.return_val.digest());

            hasher.update([discriminator::END]);
        }

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

    pub const MATCH_CASE_SLICE: u8 = 16;
    pub const MATCH_CASE: u8 = 17;

    pub const END: u8 = 64;
}
