//! This module provides a thin wrapper around
//! `hmac_sha256::Hash`.
//! For some reason, `hmac_sha256::Hash` does not
//! implement `std::hash::Hasher`.
//! This makes it inconvenient to use.
//! To solve this, this module provides
//! a wrapper that implements `std::hash::Hasher`.

use super::*;

use std::hash::Hasher;

#[derive(Clone, Copy, Default)]
pub struct Sha256Hasher(hmac_sha256::Hash);

impl Sha256Hasher {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Sha256Hasher {
    pub fn digest(self) -> Digest {
        Digest(self.0.finalize())
    }
}

impl Hasher for Sha256Hasher {
    fn write(&mut self, bytes: &[u8]) {
        self.0.update(bytes);
    }

    /// This does not produce the full digest.
    /// Instead, it only produces the first 8 bytes.
    /// You can get the full digest by calling the `digest` method.
    fn finish(&self) -> u64 {
        let digest = self.digest().0;
        u64::from_be_bytes([
            digest[0], digest[1], digest[2], digest[3], digest[4], digest[5], digest[6], digest[7],
        ])
    }
}
