// xoshiro128++ pseudorandom number generator
pub struct Rng {
    state: [u32; 4],
}

impl Rng {
    pub fn new(mut seed: u64) -> Self {
        let processed_1 = splitmix64_next(&mut seed);
        let processed_2 = splitmix64_next(&mut seed);

        Self {
            state: [
                (processed_1 & 0xffffffff) as u32,
                (processed_1 >> 32) as u32,
                (processed_2 & 0xffffffff) as u32,
                (processed_2 >> 32) as u32,
            ],
        }
    }

    pub fn random(&mut self, s: u32) -> u32 {
        // Use Daniel Lemire's algorithm to get a random number from zero to s.
        let mut x = self.xoshiro128pp_next();
        let mut m = x as u64 * s as u64;
        let mut l = (m & 0xffffffff) as u32;

        if l < s {
            let t = s.wrapping_neg() % s;

            while l < t {
                x = self.xoshiro128pp_next();
                m = x as u64 * s as u64;
                l = (m & 0xffffffff) as u32;
            }
        }

        (m >> 32) as u32
    }

    // xoshiro128++ adapted from https://prng.di.unimi.it/xoshiro128plusplus.c
    fn xoshiro128pp_next(&mut self) -> u32 {
        let result = self.state[0]
            .overflowing_add(self.state[3])
            .0
            .rotate_left(7)
            .overflowing_add(self.state[0])
            .0;

        let t = self.state[1] << 9;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;

        self.state[3] = self.state[3].rotate_left(11);

        result
    }
}

// splitmix64 adapted from https://prng.di.unimi.it/splitmix64.c
fn splitmix64_next(x: &mut u64) -> u64 {
    *x = x.overflowing_add(0x9e3779b97f4a7c15).0;
    let z = *x;
    let z = (z ^ (z >> 30)).overflowing_mul(0xbf58476d1ce4e5b9).0;
    let z = (z ^ (z >> 27)).overflowing_mul(0x94d049bb133111eb).0;
    z ^ (z >> 31)
}
