use std::io::{BufReader, BufRead, Error};
use std::fs::File;
use std::time::Instant;
use std::str::Chars;
use std::collections::HashMap;

///////////////////////////////////////////////////////////////////////////////

enum Instr {
    Mask {ones: u64, zeros: u64, floating: u64},
    Mem {addr: u64, val: u64},
}

use Instr::*;

///////////////////////////////////////////////////////////////////////////////

pub fn run() {
    let time = Instant::now();

    let f = BufReader::new(File::open("../input/day14.txt").unwrap());
    let instructions = process_input(f);

    let (sol_part_1, sol_part_2) = get_sols(&instructions);

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}

///////////////////////////////////////////////////////////////////////////////

fn get_sols(ls: &Vec<Instr>) -> (u64, u64) {
    let mut memory1 = HashMap::new();
    let mut memory2 = HashMap::new();

    let mut mask_ones = 0;
    let mut mask_zeros = 0;
    let mut mask_floating = 0;

    for instr in ls {
        match instr {
            Mask{ones, zeros, floating} => {
                mask_ones = *ones;
                mask_zeros = *zeros;
                mask_floating = *floating;
            },
            Mem{addr, val} => {
                let val_masked = *val & mask_zeros | mask_ones;
                let addr_masked = *addr | mask_ones;
                memory1.insert(*addr, val_masked);

                for addr_alt in get_alt_addrs(addr_masked, mask_floating) {
                    memory2.insert(addr_alt, *val);
                }
            }
        }
    }

    return (memory1.values().sum(), memory2.values().sum());
}

fn get_alt_addrs(addr: u64, floating: u64) -> Vec<u64> {
    let mut res = Vec::new();

    for i in 0..(2 as u64).pow(floating.count_ones()) {
        let mut k = i;
        let mut mask_ones = 0;
        let mut mask_zeros = u64::MAX;
        let mut float_bits = floating;
        let mut pos = 0;

        while float_bits != 0 {
            let b = float_bits & 1;
            
            if b != 0 {
                if k & 1 == 1 {
                    mask_ones |= 1 << pos;
                } else {
                    mask_zeros &= !(1 << pos);
                }

                k >>= 1;
            }
            
            float_bits >>= 1;
            pos += 1;
        }

        res.push(addr & mask_zeros | mask_ones);
    }

    return res;
}

fn process_input(f: BufReader<File>) -> Vec<Instr> {
    f.lines()
     .map(line_to_instr)
     .collect()
}

fn line_to_instr(line: Result<String, Error>) -> Instr {
    let line = line.unwrap();
    let mut chars = line.chars();

    // Skip the first one and analyze the second one
    chars.next();
    let ch = chars.next().unwrap();

    if ch == 'a' {
        // Skip 5 and go right to the beggining of the mask
        for _ in 0..5 { chars.next(); }
        return get_mask(&mut chars);
    } else {
        // Skip 2 and go right to the beggining of the position
        for _ in 0..2 { chars.next(); }
        return get_mem(&mut chars);

    }
}

fn get_mem(chars: &mut Chars) -> Instr {
    let mut addr: u64 = 0;
    let mut val: u64 = 0;
    let mut ch;

    while {ch = chars.next().unwrap(); ch != ']'} {
        let x = ch.to_digit(10).unwrap() as u64;
        addr = addr * 10 + x;
    }

    // Skip 3
    for _ in 0..3 { chars.next(); }

    while let Some(ch) = chars.next() {
        let x = ch.to_digit(10).unwrap() as u64;
        val = val * 10 + x;
    }

    return Mem{addr, val};
}

fn get_mask(chars: &mut Chars) -> Instr {
    let mut ones: u64 = 0;
    let mut zeros: u64 = u64::MAX;
    let mut floating: u64 = 0;
    let mut pos = 36;

    while let Some(ch) = chars.next() {
        pos -= 1;
        match ch {
            'X' => floating |= 1 << pos,
            '1' => ones |= 1 << pos,
            '0' => zeros &= !(1 << pos),
             _  => panic!("wtf"),
        }
    }

    return Mask{ones, zeros, floating};
}