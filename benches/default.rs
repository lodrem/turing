#![feature(test)]

extern crate test;

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use test::Bencher;

#[bench]
fn bench_default_hasher_1_000(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..1_000_u64 {
            let mut hasher = DefaultHasher::new();
            hasher.write(&i.to_be_bytes());
            hasher.finish();
        }
    });
}
