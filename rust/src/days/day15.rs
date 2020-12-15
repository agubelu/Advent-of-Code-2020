use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;
use std::collections::hash_map::Entry;

use rustc_hash::FxHashMap;

///////////////////////////////////////////////////////////////////////////////

pub fn run() {
    let time = Instant::now();

    let f = BufReader::new(File::open("../input/day15.txt").unwrap());
    let mut mem = get_initial_numbers(f);
    let ini = mem.len() as u32 + 2;

    let sol_part_1 = get_val(&mut mem, ini, 2020, 0);
    let sol_part_2 = get_val(&mut mem, 2021, 30_000_000, sol_part_1);
    
    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}

///////////////////////////////////////////////////////////////////////////////

fn get_val(mem: &mut FxHashMap<u32, u32>, initial_turn: u32, final_turn: u32, initial_val: u32) -> u32 {
    let mut last_num = initial_val;

    for i in initial_turn..=final_turn {
        match mem.entry(last_num) {
            Entry::Occupied(e) => {
                let v = e.into_mut();
                last_num = i - *v - 1;
                *v = i-1;
            },
            Entry::Vacant(e) => {
                e.insert(i-1);
                last_num = 0;
            }
        }
    }

    return last_num;
}

fn get_initial_numbers(mut f: BufReader<File>) -> FxHashMap<u32, u32> {
    let mut s = String::new();
    let _ = f.read_line(&mut s);
    let split: Vec<&str> = s.split(',').collect();

    return split.iter()
                .enumerate()
                .map(|(i, spl)| (spl.parse().unwrap(), i as u32 + 1))
                .collect();
}