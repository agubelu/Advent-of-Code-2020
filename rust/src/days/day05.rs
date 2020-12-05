use std::boxed::Box;
use std::error::Error;
use std::collections::HashMap;
use std::time::Instant;
use crate::utils::utils;

///////////////////////////////////////////////////////////////////////////////

const ROW_ADDR_SIZE: usize = 13;
const ROW_MULT: usize = 32;

pub fn run() -> Result<(), Box<dyn Error>> {

    let lines = utils::read_file_lines("../input/day05_bigboy.txt")?;
    let time = Instant::now();
 
    let mut max_sid: usize = 0;
    let mut existing_seats: HashMap<usize, usize> = HashMap::new();
    let mut missing_seats: HashMap<usize, usize> = HashMap::new();
    let mut missing_2: HashMap<usize, usize> = HashMap::new();

    for code in &lines {
        // Part 1
        let sid = get_sid(code);
        if sid > max_sid {
            max_sid = sid;
        }

        // Part 2
        if missing_seats.contains_key(&sid) {
            missing_seats.remove(&sid);
        }
        
        if missing_2.contains_key(&sid) {
            missing_2.remove(&sid);
        }

        existing_seats.insert(sid, 1);
        for other in [sid+1, sid-1].iter() {
            if existing_seats.contains_key(&other) {
                continue;
            }
            let n_refs = missing_seats.get(&other).unwrap_or(&0) + 1;
            missing_seats.insert(*other, n_refs);
            if n_refs == 2 {
                missing_2.insert(*other, 1);
            }
        }
    }

    let my_sid = missing_2.keys().next().unwrap();

    let elapsed = time.elapsed().as_millis();
    println!("Part 1: {}", max_sid);
    println!("Part 2: {}", my_sid);
    println!("Elapsed: {} ms", elapsed);
    Ok(())
}

fn get_sid(code: &String) -> usize {
    let bin: String = code.chars().map(|x| match x {
        'F' => '0',
        'L' => '0',
        'B' => '1',
        'R' => '1',
        _ => x
    }).collect();
    let row: usize = usize::from_str_radix(&bin[..ROW_ADDR_SIZE], 2).unwrap();
    let col: usize = usize::from_str_radix(&bin[ROW_ADDR_SIZE..], 2).unwrap();
    row * ROW_MULT + col
}