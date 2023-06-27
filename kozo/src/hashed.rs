use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

pub use crate::sha256_hasher::*;

#[derive(Clone, Debug)]
pub struct Sha256Hashed<T, A> {
    pub value: T,
    pub digest: Digest,
    _marker: std::marker::PhantomData<A>,
}

impl<T, A> Sha256Hashed<T, A>
where
    T: HashWithAlgorithm<A>,
{
    pub fn new(value: T) -> Self {
        let mut hasher = Sha256Hasher::new();
        value.hash(&mut hasher);
        let digest = hasher.digest();

        Self {
            value,
            digest,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T, A> Hash for Sha256Hashed<T, A> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write(self.digest.as_ref());
    }
}

pub trait HashWithAlgorithm<A> {
    fn hash<H: Hasher>(&self, state: &mut H);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefaultHashAlgorithm;

impl<T> HashWithAlgorithm<DefaultHashAlgorithm> for T
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(self, state);
    }
}

pub trait GetDigest {
    fn digest(&self) -> &Digest;
}
