#![feature(test)]

extern crate test;

use std::hash::Hasher;
use test::Bencher;

use turing::fnv::{FNV1Hasher, FNV1aHasher};

#[bench]
fn bench_fnv_1_with_32bits_1_000(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..1_000_u64 {
            let mut hasher = FNV1Hasher::with_32();
            hasher.write(&i.to_be_bytes());
            hasher.finish();
        }
    });
}

#[bench]
fn bench_fnv_1a_with_32bits_1_000(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..1_000_u64 {
            let mut hasher = FNV1aHasher::with_32();
            hasher.write(&i.to_be_bytes());
            hasher.finish();
        }
    });
}

#[bench]
fn bench_fnv_1_with_64bits_1_000(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..1_000_u64 {
            let mut hasher = FNV1Hasher::with_64();
            hasher.write(&i.to_be_bytes());
            hasher.finish();
        }
    });
}

#[bench]
fn bench_fnv_1a_with_64bits_1_000(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..1_000_u64 {
            let mut hasher = FNV1aHasher::with_64();
            hasher.write(&i.to_be_bytes());
            hasher.finish();
        }
    });
}
