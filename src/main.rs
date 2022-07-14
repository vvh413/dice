use crate::constants::MOUSE_TICK_COUNT;
use crate::progress::Bar;
use crate::seeder::{get_seed, BAR, SEED};

use regex::Regex;
use std::env;

mod constants;
mod help;
mod mat;
mod progress;
mod random;
mod seeder;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help::default();
    }

    let dice = args[1].as_str();
    let (x, y) = parse_dice(dice);

    unsafe {
        BAR = Bar::new(MOUSE_TICK_COUNT);
    }

    get_seed().await;

    let results = unsafe { random::randomize(x, y, SEED) };
    let sum: u32 = results.iter().sum();
    println!("{:} = {:?} = {:}", dice, results, sum);
}

fn parse_dice(dice: &str) -> (u32, u32) {
    let re = Regex::new(r"\d+d\d+").unwrap();
    if !re.is_match(dice) {
        help::default();
    }
    let params: Vec<&str> = dice.split('d').collect();
    let x: u32 = params[0].to_string().parse::<u32>().unwrap();
    let y: u32 = params[1].to_string().parse::<u32>().unwrap();
    (x, y)
}
