use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;

///////////////////////////////////////////////////////////////////////////////

pub fn run() {
    let time = Instant::now();
    
    let f = BufReader::new(File::open("../input/day23.txt").unwrap());
    let input = read_input(f);
    let initial = input[0];

    let mut ls1 = vec![0; 10];
    for i in 0..9 {
        ls1[input[i]] = input[(i+1) % 9];
    }

    let mut ls1_copy = ls1.clone();
    crab_game(&mut ls1_copy, initial, 100);

    let mut sol_part_1 = String::new();
    let mut x = ls1_copy[1];
    while x != 1 {
        sol_part_1 += x.to_string().as_str();
        x = ls1_copy[x];
    }

    ls1.extend(11..=1_000_001);
    ls1[input[8]] = 10;
    ls1[1_000_000] = initial;

    crab_game(&mut ls1, initial, 10_000_000);
    let v1 = ls1[1];
    let v2 = ls1[v1];
    let sol_part_2 = v1 * v2;

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}

///////////////////////////////////////////////////////////////////////////////

fn crab_game(adj: &mut Vec<usize>, initial: usize, iters: usize) {
    let n_cups = adj.len() - 1;
    let mut current = initial;

    for _ in 0..iters {
        let c1 = adj[current];
        let c2 = adj[c1];
        let c3 = adj[c2];

        let mut target = if current > 1 {current - 1} else {n_cups};
        while c1 == target || c2 == target || c3 == target {
            target = if target > 1 {target - 1} else {n_cups};
        }

        let taken_next = adj[c3];
        let target_next = adj[target];

        adj[current] = taken_next;
        adj[target] = c1;
        adj[c3] = target_next;
        current = taken_next;
    }
}

fn read_input(mut f: BufReader<File>) -> Vec<usize> {
    let mut line = String::new();
    f.read_line(&mut line).unwrap();
    return line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
}