use regex::Regex;
use std::env;

mod progress;
mod random;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
    }

    let dice = args[1].as_str();
    let (x, y) = parse_dice(dice);

    let seed: u64 = random::get_seed();
    let results = random::randomize(x, y, seed);
    let sum: u32 = results.iter().sum();
    println!("{:} = {:?} = {:}", dice, results, sum);
}

fn parse_dice(dice: &str) -> (u32, u32) {
    let re = Regex::new(r"\d+d\d+").unwrap();
    if !re.is_match(dice) {
        help();
    }
    let params: Vec<&str> = dice.split('d').collect();
    let x: u32 = params[0].to_string().parse::<u32>().unwrap();
    let y: u32 = params[1].to_string().parse::<u32>().unwrap();
    (x, y)
}

fn help() {
    println!("Usage: dice XdY");
    std::process::exit(0);
}
