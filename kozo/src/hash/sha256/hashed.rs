use super::*;

#[derive(Clone, Debug)]
pub struct Hashed<T, A> {
    pub value: T,
    pub digest: Digest,
    _marker: std::marker::PhantomData<A>,
}

pub type SemanticallyHashed<T> = Hashed<T, SemanticHashAlgorithm>;

impl<T, A> Hashed<T, A>
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

impl<T, A> Hash for Hashed<T, A> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write(self.digest.as_ref());
    }
}
