use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

pub fn random(s: u32) -> u32 {
    let x = RandomState::new().build_hasher().finish();
    let x = (x >> 32) as u32 ^ (x & 0xffffffff) as u32;
    let mut m = x as u64 * s as u64;
    let mut l = (m & 0xffffffff) as u32;
    if l < s {
        let t = s.wrapping_neg() % s;
        while l < t {
            let x = RandomState::new().build_hasher().finish();
            let x = (x >> 32) as u32 ^ (x & 0xffffffff) as u32;
            m = x as u64 * s as u64;
            l = (m & 0xffffffff) as u32;
        }
    }
    (m >> 32) as u32
}
