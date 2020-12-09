mod days;

use days::{day01, day02, day03, day04, day05,
           day06, day07, day08};
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
        1 => day01::run,
        2 => day02::run,
        3 => day03::run,
        4 => day04::run,
        5 => day05::run,
        6 => day06::run,
        7 => day07::run,
        8 => day08::run,
        _ => panic!("Not implemented."),
    };

    if let Err(error) = func() {
        panic!("Fatal: {}", error);
    }
}