use std::boxed::Box;
use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs;
use std::collections::HashSet;
use std::time::Instant;

///////////////////////////////////////////////////////////////////////////////


pub fn run() -> Result<(), Box<dyn Error>> {

    let time = Instant::now();

    let mut shared_answers = 0;
    let mut common_answers = 0;

    let f = BufReader::new(fs::File::open("../input/day06.txt").unwrap());

    let mut combined_answers_set: HashSet<char> = HashSet::new();
    let mut common_answers_set: HashSet<char> = HashSet::new();

    let mut first = true;

    for line in f.lines() {
        let line = line.unwrap();

        if line == "" {
            shared_answers += combined_answers_set.len();
            common_answers += common_answers_set.len();

            combined_answers_set = HashSet::new();
            common_answers_set = HashSet::new();

            first = true;

            continue;
        }

        let mut person_answers_set = HashSet::new();
        for ch in line.chars() {
            person_answers_set.insert(ch);
            combined_answers_set.insert(ch);
        }

        if first {
            first = false;
            common_answers_set = person_answers_set;
        } else {
            common_answers_set = &common_answers_set & &person_answers_set;
        }
    }

    shared_answers += combined_answers_set.len();
    common_answers += common_answers_set.len();

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    println!("Part 1: {}", shared_answers);
    println!("Part 2: {}", common_answers);
    println!("Elapsed: {:.3} ms", elapsed_ms);
    Ok(())
}

