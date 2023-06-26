use nohash_hasher::NoHashHasher;

use std::{collections::HashMap, hash::BuildHasherDefault};

pub type NoHashHashMap<K, V> = HashMap<K, V, BuildHasherDefault<NoHashHasher<K>>>;
