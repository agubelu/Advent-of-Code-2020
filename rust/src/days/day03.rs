use std::boxed::Box;
use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs;
use std::time::Instant;

///////////////////////////////////////////////////////////////////////////////

pub fn run() -> Result<(), Box<dyn Error>> {
    let time = Instant::now();

    let f = BufReader::new(fs::File::open("../input/day03.txt").unwrap());
    let mut lines_iter = f.lines().peekable();

    let mut slope_11: u64 = 0;
    let mut slope_31: u64 = 0;
    let mut slope_51: u64 = 0;
    let mut slope_71: u64 = 0;
    let mut slope_12: u64 = 0;

    let n = lines_iter.peek().unwrap().as_ref().unwrap().len();

    for (row, line) in lines_iter.enumerate() {
        let line = line.unwrap();
        for (i, ch) in line.chars().enumerate() {
            if ch != '#' { continue; }
            if i == row % n { slope_11 += 1; }
            if i == row*3 % n { slope_31 += 1; }
            if i == row*5 % n { slope_51 += 1; }
            if i == row*7 % n { slope_71 += 1; }
            if row % 2 == 0 && i == (row/2 % n) { slope_12 += 1; } 
        }
    }

    let sol_part_2 = slope_11 * slope_31 * slope_51 * slope_71 * slope_12;
    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", slope_31);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
    Ok(())
}
