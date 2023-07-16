use super::*;

use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct Hashed<T> {
    pub hashee: T,
    pub digest: Digest,
}

impl<T> Hashed<T>
where
    T: Hash,
{
    pub fn new(value: T) -> Self {
        let mut hasher = Sha256Hasher::new();
        value.hash(&mut hasher);
        let digest = hasher.digest();

        Self {
            hashee: value,
            digest,
        }
    }
}

impl<T> Hash for Hashed<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write(self.digest.as_ref());
    }
}

impl<T> PartialEq for Hashed<T> {
    fn eq(&self, other: &Self) -> bool {
        self.digest == other.digest
    }
}

impl<T> Eq for Hashed<T> {}
