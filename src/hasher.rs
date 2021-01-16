pub struct Hasher(u64);

impl core::hash::Hasher for Hasher {
    fn finish(&self) -> u64 {
        self.0.wrapping_mul(12605985483714917081).swap_bytes()
        //return ((self.0 >> ((self.0 >> 59) + 5)) ^ self.0) * 12605985483714917081;
    }
    fn write(&mut self, _: &[u8]) {}
    fn write_u64(&mut self, i: u64) {
        self.0 = i
    }
}

#[derive(Default)]
pub struct Builder();

impl core::hash::BuildHasher for Builder {
    type Hasher = Hasher;
    fn build_hasher(&self) -> Self::Hasher {
        Hasher(0)
    }
}
