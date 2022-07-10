use crate::progress::bar;
use mouse_rs::types::Point;
use mouse_rs::Mouse;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use term_size;

const RANDOM_POOL_SIZE: usize = 1000;
const MOVE_DELTA: i32 = 100;

pub fn get_seed() -> [u8; 32] {
    let (terminal_width, _) = term_size::dimensions().unwrap();
    let mouse = Mouse::new();
    let mut pool: Vec<u8> = Vec::new();
    let mut curr_pos = mouse.get_position().unwrap();
    bar(pool.len(), RANDOM_POOL_SIZE, terminal_width);
    while pool.len() < RANDOM_POOL_SIZE {
        let pos = mouse.get_position().unwrap();
        if distance(&pos, &curr_pos) > MOVE_DELTA {
            let value = (pos.x ^ pos.y).to_ne_bytes();
            pool.extend(value);
            curr_pos = pos;
            bar(pool.len(), RANDOM_POOL_SIZE, terminal_width);
        }
    }
    println!();
    let mut rng = StdRng::from_entropy();
    let idx = rng.gen_range(0..RANDOM_POOL_SIZE - 32);
    pool[idx..idx + 32].try_into().unwrap()
}

pub fn randomize(x: u32, y: u32, seed: [u8; 32]) -> Vec<u32> {
    let mut rng = StdRng::from_seed(seed);
    { 0..x }.map(|_| rng.gen_range(1..=y)).collect()
}

fn distance(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}
