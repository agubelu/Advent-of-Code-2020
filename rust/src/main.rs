mod days;
pub mod utils;

use days::{day05, day06};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide the day to run as a command-line argument.");
    }

    let day: i32 = args[1].parse().unwrap_or_else(|_| {
        panic!("Not a valid day!");
    });

    let func = match day {
        5 => day05::run,
        6 => day06::run,
        _ => panic!("Not implemented."),
    };

    if let Err(error) = func() {
        panic!("Fatal: {}", error);
    }
}