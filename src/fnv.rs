use std::hash::Hasher;
use std::num::Wrapping;

const PRIME_32BITS: u32 = 16777619;
const PRIME_64BITS: u64 = 1099511628211;
// const PRIME_128BITS: u128 = 0x0000000001000000000000000000013B;

const OFFSET_32BITS: u32 = 2166136261;
const OFFSET_64BITS: u64 = 14695981039346656037;
// const OFFSET_128BITS: u128 = 0x6c62272e07bb014262b821756295c58d;

pub struct FNV1Hasher<T> {
    value: T,
}

pub struct FNV1aHasher<T> {
    value: T,
}

impl FNV1Hasher<u32> {
    pub fn with_32() -> Self {
        Self {
            value: OFFSET_32BITS,
        }
    }
}

impl FNV1aHasher<u32> {
    pub fn with_32() -> Self {
        Self {
            value: OFFSET_32BITS,
        }
    }
}

impl FNV1Hasher<u64> {
    pub fn with_64() -> Self {
        Self {
            value: OFFSET_64BITS,
        }
    }
}

impl FNV1aHasher<u64> {
    pub fn with_64() -> Self {
        Self {
            value: OFFSET_64BITS,
        }
    }
}

impl Hasher for FNV1Hasher<u32> {
    fn finish(&self) -> u64 {
        self.value as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.value = (Wrapping(self.value) * Wrapping(PRIME_32BITS)).0;
            self.value ^= *b as u32;
        }
    }
}

impl Hasher for FNV1aHasher<u32> {
    fn finish(&self) -> u64 {
        self.value as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.value ^= *b as u32;
            self.value = (Wrapping(self.value) * Wrapping(PRIME_32BITS)).0;
        }
    }
}

impl Hasher for FNV1Hasher<u64> {
    fn finish(&self) -> u64 {
        self.value
    }

    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.value = (Wrapping(self.value) * Wrapping(PRIME_64BITS)).0;
            self.value ^= *b as u64;
        }
    }
}

impl Hasher for FNV1aHasher<u64> {
    fn finish(&self) -> u64 {
        self.value
    }

    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.value ^= *b as u64;
            self.value = (Wrapping(self.value) * Wrapping(PRIME_64BITS)).0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hasher() {
        {
            let mut hasher = FNV1Hasher::with_32();
            hasher.write("".as_bytes());
            assert_eq!(2166136261, hasher.finish());

            let mut hasher = FNV1Hasher::with_32();
            hasher.write("a".as_bytes());
            assert_eq!(84696446, hasher.finish());

            let mut hasher = FNV1Hasher::with_32();
            hasher.write("ab".as_bytes());
            assert_eq!(1886858552, hasher.finish());

            let mut hasher = FNV1Hasher::with_32();
            hasher.write("abc".as_bytes());
            assert_eq!(1134309195, hasher.finish());
        }

        {
            let mut hasher = FNV1aHasher::with_32();
            hasher.write("".as_bytes());
            assert_eq!(2166136261, hasher.finish());

            let mut hasher = FNV1aHasher::with_32();
            hasher.write("a".as_bytes());
            assert_eq!(3826002220, hasher.finish());

            let mut hasher = FNV1aHasher::with_32();
            hasher.write("ab".as_bytes());
            assert_eq!(1294271946, hasher.finish());

            let mut hasher = FNV1aHasher::with_32();
            hasher.write("abc".as_bytes());
            assert_eq!(440920331, hasher.finish());
        }

        {
            let mut hasher = FNV1Hasher::with_64();
            hasher.write("".as_bytes());
            assert_eq!(
                u64::from_be_bytes([0xcb, 0xf2, 0x9c, 0xe4, 0x84, 0x22, 0x23, 0x25]),
                hasher.finish()
            );

            let mut hasher = FNV1Hasher::with_64();
            hasher.write("a".as_bytes());
            assert_eq!(
                u64::from_be_bytes([0xaf, 0x63, 0xbd, 0x4c, 0x86, 0x01, 0xb7, 0xbe]),
                hasher.finish()
            );

            let mut hasher = FNV1Hasher::with_64();
            hasher.write("ab".as_bytes());
            assert_eq!(
                u64::from_be_bytes([0x08, 0x32, 0x67, 0x07, 0xb4, 0xeb, 0x37, 0xb8]),
                hasher.finish()
            );

            let mut hasher = FNV1Hasher::with_64();
            hasher.write("abc".as_bytes());
            assert_eq!(
                u64::from_be_bytes([0xd8, 0xdc, 0xca, 0x18, 0x6b, 0xaf, 0xad, 0xcb]),
                hasher.finish()
            );
        }

        {
            let mut hasher = FNV1aHasher::with_64();
            hasher.write("".as_bytes());
            assert_eq!(
                u64::from_be_bytes([0xcb, 0xf2, 0x9c, 0xe4, 0x84, 0x22, 0x23, 0x25]),
                hasher.finish()
            );

            let mut hasher = FNV1aHasher::with_64();
            hasher.write("a".as_bytes());
            assert_eq!(
                u64::from_be_bytes([0xaf, 0x63, 0xdc, 0x4c, 0x86, 0x01, 0xec, 0x8c]),
                hasher.finish()
            );

            let mut hasher = FNV1aHasher::with_64();
            hasher.write("ab".as_bytes());
            assert_eq!(
                u64::from_be_bytes([0x08, 0x9c, 0x44, 0x07, 0xb5, 0x45, 0x98, 0x6a]),
                hasher.finish()
            );

            let mut hasher = FNV1aHasher::with_64();
            hasher.write("abc".as_bytes());
            assert_eq!(
                u64::from_be_bytes([0xe7, 0x1f, 0xa2, 0x19, 0x05, 0x41, 0x57, 0x4b]),
                hasher.finish()
            );
        }
    }
}
