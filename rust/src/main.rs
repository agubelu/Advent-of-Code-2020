mod days;
mod etc;

use days::{day01, day02, day03, day04, day05,
           day06, day07, day08, day09, day10,
           day11, day12, day13, day14, day15,
           day16, day17, day18, day19, day20,
           day21, day22, day23, day24, day25};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide the day to run as a command-line argument.");
    }

    let days: Vec<u8> = args.iter()
        .skip(1)
        .map(|x| x.parse().unwrap_or_else(|v| panic!("Not a valid day: {}", v)))
        .collect();

    for day in days {
        let func = match day {
             1 => day01::run,
             2 => day02::run,
             3 => day03::run,
             4 => day04::run,
             5 => day05::run,
             6 => day06::run,
             7 => day07::run,
             8 => day08::run,
             9 => day09::run,
            10 => day10::run,
            11 => day11::run,
            12 => day12::run,
            13 => day13::run,
            14 => day14::run,
            15 => day15::run,
            16 => day16::run,
            17 => day17::run,
            18 => day18::run,
            19 => day19::run,
            20 => day20::run,
            21 => day21::run,
            22 => day22::run,
            23 => day23::run,
            24 => day24::run,
            25 => day25::run,
             _ => panic!("Day not implemented."),
        };

        println!("\n=== Day {:02} ===", day);
        func();
    }
}
