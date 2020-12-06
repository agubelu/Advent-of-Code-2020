use std::boxed::Box;
use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs;
use std::time::Instant;

///////////////////////////////////////////////////////////////////////////////

pub fn run() -> Result<(), Box<dyn Error>> {
    let time = Instant::now();

    let f = BufReader::new(fs::File::open("../input/day02.txt").unwrap());
    let mut sol_part_1 = 0;
    let mut sol_part_2 = 0;
   
    for line in f.lines() {
        let line = line.unwrap();
        let mut n1 = 0;
        let mut n2 = 0;
        let mut ch: char;

        let mut chars = line.chars();

        while {ch = chars.next().unwrap(); ch != '-'} {
            n1 = n1*10 + ch.to_digit(10).unwrap();
        }

        while {ch = chars.next().unwrap(); ch != ' '} {
            n2 = n2*10 + ch.to_digit(10).unwrap();
        }

        let pwd_char = chars.next().unwrap();
        chars.next(); chars.next();  // Drop the following 2 characters

        let mut i = 0;
        let mut cntr = 0;
        let mut cond2_match = false;

        while let Some(ch) = chars.next() {
            if i >= n2 && cntr > n2 {
                break;
            } else if i == n1-1 {
                cond2_match = ch == pwd_char;
            } else if i == n2-1 {
                cond2_match = cond2_match != (ch == pwd_char);
            }

            if ch == pwd_char {
                cntr += 1;
            }

            i += 1;
        }

        if n1 <= cntr && cntr <= n2 {
            sol_part_1 += 1;
        }

        if cond2_match {
            sol_part_2 += 1;
        }
        
    }

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
    Ok(())
}
