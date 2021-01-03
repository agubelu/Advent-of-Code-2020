use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;

///////////////////////////////////////////////////////////////////////////////

const ITERS: u32 = 30_000_000;

///////////////////////////////////////////////////////////////////////////////

pub fn run() {
    let time = Instant::now();
    let f = BufReader::new(File::open("../input/day15.txt").unwrap());
    let (mut mem, last_step) = get_initial_numbers(f);

    let sol_part_1 = get_val(&mut mem, last_step + 2, 2020, 0);
    let sol_part_2 = get_val(&mut mem, 2021, ITERS, sol_part_1);
    
    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}

///////////////////////////////////////////////////////////////////////////////

fn get_val(mem: &mut Vec<u32>, initial_turn: u32, final_turn: u32, initial_val: u32) -> u32 {
    let mut last_num = initial_val;

    for i in initial_turn..=final_turn {
        let last_seen = mem[last_num as usize];
        mem[last_num as usize] = i-1;
        last_num = match last_seen {
            0 => 0,
            x => i - x - 1,
        };
    }

    return last_num;
}

fn get_initial_numbers(mut f: BufReader<File>) -> (Vec<u32>, u32) {
    let mut s = String::new();
    let _ = f.read_line(&mut s);
    let split: Vec<&str> = s.split(',').collect();

    let mut res = vec![0_u32; ITERS as usize];

    split.iter()
         .enumerate()
         .for_each(|(i, spl)| {
             let val = spl.parse::<u32>().unwrap();
             res[val as usize] = i as u32 + 1; 
         });

    return (res, split.len() as u32);
}