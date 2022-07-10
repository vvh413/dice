use regex::Regex;
use std::env;

mod progress;
mod random;

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
    let x: u32 = params[0].to_string().parse::<u32>().unwrap();
    let y: u32 = params[1].to_string().parse::<u32>().unwrap();
    let seed: [u8; 32] = random::get_seed();
    let results = random::randomize(x, y, seed);
    let sum: u32 = results.iter().sum();
    println!("{:} = {:?} = {:}", dice, results, sum);
}

fn help() {
    println!("Usage: dice XdY");
    std::process::exit(0);
}
