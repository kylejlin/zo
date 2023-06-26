use crate::{ast::*, sha256_hasher::*};

use std::hash::Hash;

pub use crate::hashed::*;

pub trait GetDigest {
    fn digest(&self) -> &Digest;
}

pub trait SemanticHash {
    fn semantic_hash(&self) -> Digest;
}

pub struct SemanticHashAlgorithm<T: SemanticHash>(std::marker::PhantomData<T>);

impl<T: SemanticHash> Clone for SemanticHashAlgorithm<T> {
    fn clone(&self) -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<T: SemanticHash> Copy for SemanticHashAlgorithm<T> {}
impl<T: SemanticHash> Default for SemanticHashAlgorithm<T> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<T: SemanticHash> std::fmt::Debug for SemanticHashAlgorithm<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = std::any::type_name::<Self>();
        f.write_str(type_name)
    }
}
impl<T: SemanticHash> PartialEq for SemanticHashAlgorithm<T> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
impl<T: SemanticHash> PartialOrd for SemanticHashAlgorithm<T> {
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}
impl<T: SemanticHash> Ord for SemanticHashAlgorithm<T> {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}
impl<T: SemanticHash> Eq for SemanticHashAlgorithm<T> {}
impl<T: SemanticHash> Hash for SemanticHashAlgorithm<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::any::type_name::<T>().hash(state);
    }
}

impl<T: SemanticHash> HashAlgorithm<T> for SemanticHashAlgorithm<T> {
    fn digest(input: &T) -> Digest {
        input.semantic_hash()
    }
}

pub type SemanticallyHashed<T> = Sha256Hashed<T, SemanticHashAlgorithm<T>>;

impl SemanticHash for Ind {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256Hasher::new();

        hasher.write_u8(discriminator::IND);

        hasher.write(self.name.0.as_ref());

        hasher.write_u8(discriminator::IND_UNIVERSE);
        hasher.write_usize(self.universe_level.0);
        hasher.write_u8(discriminator::END);

        hasher.write(self.index_types.digest.as_ref());

        hasher.write(self.vcon_defs.digest.as_ref());

        hasher.write_u8(discriminator::END);

        hasher.digest()
    }
}

impl SemanticHash for Vcon {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256Hasher::new();

        hasher.write_u8(discriminator::VCON);

        hasher.write(self.ind.digest.as_ref());

        hasher.write_u8(discriminator::VCON_INDEX);
        hasher.write_usize(self.vcon_index);
        hasher.write_u8(discriminator::END);

        hasher.write_u8(discriminator::END);

        hasher.digest()
    }
}

impl SemanticHash for Match {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256Hasher::new();

        hasher.write_u8(discriminator::MATCH);

        hasher.write(self.matchee.digest().as_ref());
        hasher.write(self.return_type.digest().as_ref());
        hasher.write(self.cases.digest.as_ref());

        hasher.write_u8(discriminator::END);

        hasher.digest()
    }
}

impl SemanticHash for Fun {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256Hasher::new();

        hasher.write_u8(discriminator::FUN);

        if let Some(i) = self.decreasing_index {
            hasher.write_u8(discriminator::SOME);
            hasher.write_usize(i);
        } else {
            hasher.write_u8(discriminator::NONE);
            hasher.write_usize(0);
        }

        hasher.write(self.param_types.digest.as_ref());
        hasher.write(self.return_type.digest().as_ref());
        hasher.write(self.return_val.digest().as_ref());

        hasher.write_u8(discriminator::END);

        hasher.digest()
    }
}

impl SemanticHash for App {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256Hasher::new();

        hasher.write_u8(discriminator::APP);

        hasher.write(self.callee.digest().as_ref());
        hasher.write(self.args.digest.as_ref());

        hasher.write_u8(discriminator::END);

        hasher.digest()
    }
}

impl SemanticHash for For {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256Hasher::new();

        hasher.write_u8(discriminator::FOR);

        hasher.write(self.param_types.digest.as_ref());
        hasher.write(self.return_type.digest().as_ref());

        hasher.write_u8(discriminator::END);

        hasher.digest()
    }
}

impl SemanticHash for DebNode {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256Hasher::new();

        hasher.write_u8(discriminator::DEB);

        hasher.write_usize(self.deb.0);

        hasher.write_u8(discriminator::END);

        hasher.digest()
    }
}

impl SemanticHash for UniverseNode {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256Hasher::new();

        hasher.write_u8(discriminator::UNIVERSE);

        hasher.write_usize(self.level.0);

        hasher.write_u8(discriminator::END);

        hasher.digest()
    }
}

impl SemanticHash for Box<[Expr]> {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256Hasher::new();

        hasher.write_u8(discriminator::EXPR_SLICE);

        for expr in self.iter() {
            hasher.write(expr.digest().as_ref());
        }

        hasher.write_u8(discriminator::END);

        hasher.digest()
    }
}

impl SemanticHash for Box<[VconDef]> {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256Hasher::new();

        hasher.write_u8(discriminator::VARIANT_CONSTRUCTOR_DEF_SLICE);

        for def in self.iter() {
            hasher.write_u8(discriminator::VARIANT_CONSTRUCTOR_DEF);

            hasher.write(def.param_types.digest.as_ref());
            hasher.write(def.index_args.digest.as_ref());

            hasher.write_u8(discriminator::END);
        }

        hasher.write_u8(discriminator::END);

        hasher.digest()
    }
}

impl SemanticHash for Box<[MatchCase]> {
    fn semantic_hash(&self) -> Digest {
        let mut hasher = Sha256Hasher::new();

        hasher.write_u8(discriminator::MATCH_CASE_SLICE);

        for case in self.iter() {
            hasher.write_u8(discriminator::MATCH_CASE);

            hasher.write_usize(case.arity);
            hasher.write(case.return_val.digest().as_ref());

            hasher.write_u8(discriminator::END);
        }

        hasher.write_u8(discriminator::END);

        hasher.digest()
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
