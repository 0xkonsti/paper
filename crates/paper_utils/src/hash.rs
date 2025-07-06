use std::hash::{Hash, Hasher};

use ahash::AHasher;

pub fn hash<T: Hash>(value: &T) -> u64 {
    let mut hasher = AHasher::default();
    value.hash(&mut hasher);
    hasher.finish()
}
