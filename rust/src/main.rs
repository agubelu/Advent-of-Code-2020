mod days;
mod etc;

use days::{day01, day02, day03, day04, day05,
           day06, day07, day08, day09, day10,
           day11, day12, day13, day14, day15,
           day16, day17, day18,
                  day22, day23, day24, day25};
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
        01 => day01::run,
        02 => day02::run,
        03 => day03::run,
        04 => day04::run,
        05 => day05::run,
        06 => day06::run,
        07 => day07::run,
        08 => day08::run,
        09 => day09::run,
        10 => day10::run,
        11 => day11::run,
        12 => day12::run,
        13 => day13::run,
        14 => day14::run,
        15 => day15::run,
        16 => day16::run,
        17 => day17::run,
        18 => day18::run,



        22 => day22::run,
        23 => day23::run,
        24 => day24::run,
        25 => day25::run,
         _ => panic!("Not implemented."),
    };

    func();
}