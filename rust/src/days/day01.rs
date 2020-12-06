use std::boxed::Box;
use std::error::Error;
use std::io::{BufReader, BufRead};
use std::collections::BTreeMap;
use std::fs;
use std::time::Instant;

///////////////////////////////////////////////////////////////////////////////

const TARGET_SUM: usize = 2020;
// target sum for day1 big boy is 99920044

pub fn run() -> Result<(), Box<dyn Error>> {
    let time = Instant::now();

    let f = BufReader::new(fs::File::open("../input/day01.txt").unwrap());
    let mut vec: Vec<usize> = vec![];
    let mut map: BTreeMap<usize, usize> = BTreeMap::new();

    let mut sol_part_1 = 0;
    let mut sol_part_2 = 0;

    for line in f.lines() {
        let num: usize = line.unwrap().parse().unwrap();
        vec.push(num);
        map.insert(num, 1);
    }

    vec.sort();
        
    for i in 0..vec.len() {
        let e1 = vec[i];
        for j in i+1..vec.len() {
            let e2 = vec[j];

            if e1 + e2 == TARGET_SUM {
                sol_part_1 = e1 * e2;
                break;
            } else if e1 + e2 > TARGET_SUM {
                break;
            } else if map.contains_key(&(TARGET_SUM - e1 - e2)) {
                sol_part_2 = e1 * e2 * (TARGET_SUM - e1 - e2);
                if sol_part_1 != 0 {
                    break;
                }
            }
        }

        if sol_part_1 != 0 && sol_part_2 != 0 {
            break;
        }
    }

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
    Ok(())
}
