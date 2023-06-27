use super::*;

/// A SHA256 digest.
#[derive(Clone, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct Digest(pub [u8; 32]);

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
