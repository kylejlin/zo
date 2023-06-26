use std::fmt::Debug;

pub use crate::sha256_hasher::Digest;

#[derive(Clone, Debug)]
pub struct Sha256Hashed<T, A: HashAlgorithm<T>> {
    pub value: T,
    pub digest: Digest,
    _marker: std::marker::PhantomData<A>,
}

impl<T, A: HashAlgorithm<T>> Sha256Hashed<T, A> {
    pub fn new(value: T) -> Self {
        let digest = A::digest(&value);
        Self {
            value,
            digest,
            _marker: std::marker::PhantomData,
        }
    }
}

pub trait HashAlgorithm<T> {
    fn digest(input: &T) -> Digest;
}
