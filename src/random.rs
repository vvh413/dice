use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn randomize(x: u32, y: u32, seed: [u8; 32]) -> Vec<u32> {
    let mut rng = StdRng::from_seed(seed);
    { 0..x }.map(|_| rng.gen_range(1..=y)).collect()
}
