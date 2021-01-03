use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;

///////////////////////////////////////////////////////////////////////////////
// This method for computing Part 2 sadly doesn't work with the big boy input,
// since it relies on the fact that all adaptors have a difference of either
// 1 or 3, which is not true for that input.

pub fn run() {
    let time = Instant::now();

    let f = BufReader::new(File::open("../input/day10.txt").unwrap());
    let mut ls: Vec<u64> = f.lines().map(|l| l.unwrap().parse().unwrap()).collect();
    ls.sort_unstable();
    ls.insert(0, 0);
    
    let mut ones: u64 = 0;
    let mut threes: u64 = 1;

    let mut sol_part_2: u64 = 1;
    let mut x = 1;

    for i in 1..ls.len() {
        match ls[i] - ls[i-1] {
            1 => {
                ones += 1;
                x += 1;
            },
            3 => {
                threes += 1;
                if x >= 3 {
                    sol_part_2 *= trib(x);
                }
                x = 1;
            },
            _ => continue,
        };
    }

    if x >= 3 {
        sol_part_2 *= trib(x);
    }

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", ones * threes);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}

fn trib(n: u64) -> u64 {
    match n {
        3 => 2,
        4 => 4,
        5 => 7,
        _ => {
            let (mut a, mut b, mut c) = (2,4,7);
            for _ in 0..n-5 {
                let aux = a;
                a = b;
                b = c;
                c = aux+a+b;
            }
            c
        }
    }
}
