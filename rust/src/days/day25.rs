use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;

///////////////////////////////////////////////////////////////////////////////

const MOD: u64 = 20201227;

///////////////////////////////////////////////////////////////////////////////

pub fn run() {
    let time = Instant::now();

    let f = BufReader::new(File::open("../input/day25.txt").unwrap());
    let mut lines = f.lines();
    
    let card_pk: u64 = lines.next().unwrap().unwrap().parse().unwrap();
    let mut door_pk: u64 = lines.next().unwrap().unwrap().parse().unwrap();
    
    let mut aux = 1;
    let mut card_loop = 0;
    
    // Find the card private exponent
    while aux != card_pk {
        aux = (aux * 7) % MOD;
        card_loop += 1;
    }
    
    // Raise door_pk to the card exponent modulo MOD
    let mut key = 1;
    while card_loop != 0 {
        if card_loop & 1 != 0 {
            key = (key * door_pk) % MOD;
        }
        door_pk = (door_pk * door_pk) % MOD;
        card_loop >>= 1;
    }

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", key);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}
