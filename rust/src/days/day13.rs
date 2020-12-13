use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;

use itertools::izip;
use modinverse::modinverse;

///////////////////////////////////////////////////////////////////////////////

pub fn run() {
    let time = Instant::now();

    let f = BufReader::new(File::open("../input/day13.txt").unwrap());
    let (tstamp, buses, indices) = process_input(f);

    let mut min_wait = (i64::MAX, 0);

    for bus in buses.iter() {
        let wait = bus - (tstamp % bus);
        if wait < min_wait.0 {
            min_wait = (wait, *bus);
        }
    }

    let rems: Vec<i64> = izip!(&indices, &buses)
    .map(|(i, bus)| (bus - i).rem_euclid(*bus))
    .collect();
    
    let sol_part_1 = min_wait.0 * min_wait.1;
    let sol_part_2 = chinese_rem_theorem(&rems, &buses);

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}

///////////////////////////////////////////////////////////////////////////////

fn chinese_rem_theorem(rems: &Vec<i64>, mods: &Vec<i64>) -> i64 {
    let prod: i64 = mods.iter().product();
    let prods_i: Vec<i64> = mods.iter().map(|x| prod / x).collect();
    let invs_i: Vec<i64> = izip!(&prods_i, mods).map(|(ni, m)| modinverse(*ni, *m).unwrap()).collect();
    let sum: i64 = izip!(rems, &prods_i, &invs_i).map(|(r, n, x)| r * n * x).sum();
    return sum % prod;
}

fn process_input(f: BufReader<File>) -> (i64, Vec<i64>, Vec<i64>) {
    let lines: Vec<String> = f.lines().collect::<Result<_, _>>().unwrap();
    let tstamp = lines[0].parse().unwrap();
    let spl: Vec<&str> = lines[1].split(",").collect(); 
    let n_elems = spl.len();

    let mut buses = Vec::with_capacity(n_elems);
    let mut indices = Vec::with_capacity(n_elems);

    for (i, elem) in spl.iter().enumerate() {
        match *elem {
            "x" => continue,
            val => {
                indices.push(i as i64);
                buses.push(val.parse().unwrap());
            },
        }
    }

    return (tstamp, buses, indices);
}