use std::hash::{Hash, Hasher};

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
