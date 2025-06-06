use std::hash::{DefaultHasher, Hash, Hasher};

pub fn hash_f32<H: Hasher>(hasher: &mut H, data: f32) {
    let bytes = data.to_ne_bytes();
    hasher.write(&bytes);
}

pub fn hash_f64<H: Hasher>(hasher: &mut H, data: f64) {
    let bytes = data.to_ne_bytes();
    hasher.write(&bytes);
}

pub fn hash_f32_array<H: Hasher>(hasher: &mut H, data: &[f32]) {
    for &value in data {
        hash_f32(hasher, value);
    }
}

pub fn hash_f64_array<H: Hasher>(hasher: &mut H, data: &[f64]) {
    for &value in data {
        hash_f64(hasher, value);
    }
}

pub fn calculate_hash<T: Hash>(data: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}
