use crate::{ast::*, nohash_hashmap::NoHashHashMap};

use std::num::NonZeroUsize;

#[derive(Clone, Debug, Default)]
pub struct DebShiftCache {
    up_caches: Vec<NoHashHashMap<Digest, Expr>>,
    down_caches: Vec<NoHashHashMap<Digest, Expr>>,
}

impl DebShiftCache {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_up(&self, level: NonZeroUsize, digest: Digest) -> Option<Expr> {
        let level_index = level.get() - 1;
        self.up_caches
            .get(level_index)
            .and_then(|cache| cache.get(&digest))
            .cloned()
    }

    pub fn get_down(&self, level: NonZeroUsize, digest: Digest) -> Option<Expr> {
        let level_index = level.get() - 1;
        self.down_caches
            .get(level_index)
            .and_then(|cache| cache.get(&digest))
            .cloned()
    }

    pub fn set_up(&mut self, level: NonZeroUsize, digest: Digest, expr: Expr) {
        let level_index = level.get() - 1;
        if level_index >= self.up_caches.len() {
            self.up_caches
                .resize_with(level_index + 1, Default::default);
        }

        self.up_caches[level_index].insert(digest, expr);
    }

    pub fn set_down(&mut self, level: NonZeroUsize, digest: Digest, expr: Expr) {
        let level_index = level.get() - 1;
        if level_index >= self.down_caches.len() {
            self.down_caches
                .resize_with(level_index + 1, Default::default);
        }

        self.down_caches[level_index].insert(digest, expr);
    }
}
