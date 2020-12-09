use std::boxed::Box;
use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;
/* use std::collections::HashSet;
 * read commented function below
 */

const BUFFER_SIZE: usize = 25;

///////////////////////////////////////////////////////////////////////////////

pub fn run() -> Result<(), Box<dyn Error>> {
    let time = Instant::now();

    let f = BufReader::new(File::open("../input/day09.txt").unwrap());
    let ls = read_numbers(f);

    let target = find_first_not_valid(&ls);
    let (i, j) = find_indices_range(&ls, target);
    let (min, max) = find_min_max(&ls[i..=j]);

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", target);
    println!("Part 2: {}", min + max);
    println!("Elapsed: {:.3} ms", elapsed_ms);
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////

fn find_min_max(buf: &[u64]) -> (u64, u64) {
    let mut max: u64 = buf[0];
    let mut min = max;

    for v in buf {
        let val = *v;
        if val > max {
            max = val;
        } else if val < min {
            min = val;
        }
    }

    return (min, max);
}

fn find_indices_range(ls: &Vec<u64>, target: u64) -> (usize, usize) {
    let n = ls.len();

    for i in 0..n {
        let mut acc = ls[i];
        for j in i+1..n {
            acc += ls[j];

            if acc == target {
                return (i, j);
            } else if acc > target {
                break;
            }
        }
    }
    panic!("Not found");
}

fn find_first_not_valid(ls: &Vec<u64>) -> u64 {
    for i in BUFFER_SIZE..ls.len() {
        let target = ls[i];
        let buf = &ls[i-BUFFER_SIZE..i];
        if !is_valid(target, buf) {
            return target;
        }
    }
    panic!("Not found");
}

/*
 * This one is O(n) w.r.t. buffer size, while the uncommented one is O(n^2)
 * However, the buffer size is small enough for this one to run slower,
 * since the overhead caused by the HashSet is not worth it.
 * 

fn is_valid(target: u64, buf: &[u64]) -> bool {
    let mut set = HashSet::with_capacity(BUFFER_SIZE);
    for x in buf {
        if set.contains(&(target - x)) {
            return true;
        }
        set.insert(x);
    }

    return false;
}
*/

fn is_valid(target: u64, buf: &[u64]) -> bool {
    let n = buf.len();
    for i in 0..n {
        for j in i..n {
            if buf[i] + buf[j] == target {
                return true;
            }
        }
    } 

    return false;
}

fn read_numbers(f: BufReader<File>) -> Vec<u64> {
    f
     .lines()
     .map(|x| x.unwrap().parse().unwrap())
     .collect()
}