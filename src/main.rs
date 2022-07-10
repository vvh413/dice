use mouse_rs::types::Point;
use mouse_rs::Mouse;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use regex::Regex;
use std::env;
use std::io::{stdout, Write};

const RANDOM_POOL_SIZE: usize = 200;
const MOVE_DELTA: i32 = 100;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
    }

    let re = Regex::new(r"\d+d\d+").unwrap();
    let dice = args[1].as_str();
    if !re.is_match(dice) {
        help();
    }
    let params: Vec<&str> = dice.split('d').collect();
    let x: i32 = params[0].to_string().parse::<i32>().unwrap();
    let y: i32 = params[1].to_string().parse::<i32>().unwrap();
    let seed: [u8; 32] = get_seed();
    let results = randomize(x, y, seed);
    let sum: i32 = results.iter().sum();
    println!("\r{:} = {:?} = {:}      ", dice, results, sum);
}

fn help() {
    println!("Usage: dice XdY");
    std::process::exit(0);
}

fn get_seed() -> [u8; 32] {
    let mouse = Mouse::new();
    let mut pool: Vec<u8> = Vec::new();
    let mut curr_pos = mouse.get_position().unwrap();
    let loading_symbol_dimension = 100 as f64 / (RANDOM_POOL_SIZE as f64);
    print!("\rDrag cursor: 0%");
    stdout().flush().unwrap();
    while pool.len() < RANDOM_POOL_SIZE {
        let pos = mouse.get_position().unwrap();
        if distance(&pos, &curr_pos) > MOVE_DELTA {
            let mut value = (pos.x ^ pos.y).to_ne_bytes();
            pool.extend(value);
            curr_pos = pos;
            let loading_symbols_count = pool.len() as f64 * loading_symbol_dimension;
            print!("\rDrag cursor: {:}%", loading_symbols_count as i32);
            stdout().flush().unwrap();
        }
    }
    // println!();
    let mut rng = StdRng::from_entropy();
    let idx = rng.gen_range(0..RANDOM_POOL_SIZE - 32);
    pool[idx..idx + 32].try_into().unwrap()
}

fn randomize(x: i32, y: i32, seed: [u8; 32]) -> Vec<i32> {
    let mut rng = StdRng::from_seed(seed);

    { 0..x }.map(|_| rng.gen_range(1..=y)).collect()
}

fn distance(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}
