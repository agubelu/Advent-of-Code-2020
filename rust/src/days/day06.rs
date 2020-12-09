use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;

///////////////////////////////////////////////////////////////////////////////

pub fn run() {
    let time = Instant::now();

    let f = BufReader::new(File::open("../input/day06.txt").unwrap());

    let mut sol_part_1 = 0;
    let mut sol_part_2 = 0;

    let mut answers_and: u32 = 0;
    let mut answers_or: u32 = 0;

    for line in f.lines() {
        let line = line.unwrap();

        if line == "" {
            sol_part_1 += answers_or.count_ones();
            sol_part_2 += answers_and.count_ones();

            answers_or = 0;
            answers_and = 0;

            continue;
        }

        let mut answers: u32 = 0;
        for ch in line.chars() {
            answers |= 1 << (ch as u8);
        }

        if answers_or == 0 {
            answers_and = answers;
        } else {
            answers_and &= answers;
        }

        answers_or |= answers;
    }

    sol_part_1 += answers_or.count_ones();
    sol_part_2 += answers_and.count_ones();

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}
