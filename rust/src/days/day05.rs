use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;

///////////////////////////////////////////////////////////////////////////////

const ROW_ADDR_SIZE: usize = 7;
const ROW_MULT: usize = 8;

pub fn run() {
    let time = Instant::now();
 
    let f = BufReader::new(File::open("../input/day05.txt").unwrap());
    let mut sol_part_1: usize = 0;
    let mut seats: Vec<usize> = Vec::new();

    for line in f.lines() {
        let line = line.unwrap();
        let sid = get_sid(&line);

        if sid > sol_part_1 {
            sol_part_1 = sid;
        }

        seats.push(sid);
    }

    seats.sort_unstable();

    let mut sol_part_2 = 0;
    let mut aux = 0;
    let offset = seats[0];

    for (i, val) in seats.iter().enumerate() {
        aux ^= i ^ (val - offset);
        if aux != 0 {
            sol_part_2 = *val - 1;
            break;
        }
    }

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}

fn get_sid(code: &str) -> usize {
    let bin: String = code.chars().map(|x| match x {
        'F' => '0',
        'L' => '0',
        'B' => '1',
        'R' => '1',
        _ => x
    }).collect();
    let row: usize = usize::from_str_radix(&bin[..ROW_ADDR_SIZE], 2).unwrap();
    let col: usize = usize::from_str_radix(&bin[ROW_ADDR_SIZE..], 2).unwrap();
    return row * ROW_MULT + col;
}