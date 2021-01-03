use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;

use rayon::prelude::*;

///////////////////////////////////////////////////////////////////////////////

type NeighborMap = HashMap<usize, Vec<usize>>;
const DIRS: [(isize, isize); 8] = [(-1, -1), (0, -1), (1, -1),
                                   (-1,  0),          (1,  0),
                                   (-1,  1), (0,  1), (1,  1)];

///////////////////////////////////////////////////////////////////////////////
/// This is basically the same idea that i've done in Python

pub fn run() {
    let time = Instant::now();

    let f = BufReader::new(File::open("../input/day11.txt").unwrap());
    let lines: Vec<String> = f.lines().collect::<Result<_, _>>().unwrap();
    let (max_x, max_y) = (lines[0].len(), lines.len());
    let (seats, adj1, adj2) = process_lines(&lines, max_x, max_y);
    
    let sol_part_1 = solve(&seats, &adj1, 4);
    let sol_part_2 = solve(&seats, &adj2, 5);
    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;

    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}

///////////////////////////////////////////////////////////////////////////////

fn solve(
    ls: &Vec<usize>,
    adj_map: &HashMap<usize, Vec<usize>>,
    max_around: usize
) -> usize {
    let mut state = ls.clone();
    let mut next_state = state.par_iter()
                              .enumerate()
                              .map(|(i, val)| get_next(*val, i, max_around, &adj_map, &state))
                              .collect();

    while state != next_state {
        state = next_state;
        next_state = state.par_iter()
                          .enumerate()
                          .map(|(i, val)| get_next(*val, i, max_around, &adj_map, &state))
                          .collect();
    }

    return state.par_iter().sum();
}

fn get_next(elem: usize, i: usize, max: usize, adj: &HashMap<usize, Vec<usize>>, state: &Vec<usize>) -> usize {
    let count: usize = adj[&i]
                  .iter()
                  .map(|k| state[*k])
                  .sum();
    if elem == 0 && count == 0 {
        return 1;
    } else if elem == 1 && count >= max {
        return 0;
    } else {
        return elem;
    }
}

fn find_adj_1(
    coords: &(usize, usize),
    coords_map: &HashMap<(usize, usize), usize>,
    max_x: isize,
    max_y: isize
) -> Vec<usize> {
    let mut res = Vec::with_capacity(8);

    for (dx, dy) in &DIRS {
        let x = coords.0 as isize + dx;
        let y = coords.1 as isize + dy;
        let tup = (x as usize, y as usize);
        if x >= 0 && x < max_x && y >= 0 && y < max_y && coords_map.contains_key(&tup) {
            res.push(coords_map[&tup]);
        }
    } 

    return res;
}

fn find_adj_2(
    coords: &(usize, usize),
    coords_map: &HashMap<(usize, usize), usize>,
    max_x: isize,
    max_y: isize
) -> Vec<usize> {
    let mut res = Vec::with_capacity(8);

    for (dx, dy) in &DIRS {
        let mut x = coords.0 as isize + dx;
        let mut y = coords.1 as isize + dy;
        
        while x >= 0 && x < max_x && y >= 0 && y < max_y {
            let tup = (x as usize, y as usize);
            if coords_map.contains_key(&tup) {
                res.push(coords_map[&tup]);
                break;
            }

            x += dx;
            y += dy;
        }
    } 

    return res;
}

fn process_lines(lines: &[String], max_x: usize, max_y: usize) -> (Vec<usize>, NeighborMap, NeighborMap) {
    let mut inserted = 0;
    let cap = max_x * max_y;
    let mut coords = HashMap::with_capacity(cap);
    let mut seats = Vec::with_capacity(cap);

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == 'L' {
                coords.insert((x, y), inserted);
                seats.push(0);
                inserted += 1;
            }
        }
    }

    let mx = max_x as isize;
    let my = max_y as isize;

    let keys: Vec<&(usize, usize)> = coords.keys().collect();

    let adj1 = keys.par_iter()
                   .map(|k| (coords[k], find_adj_1(k, &coords, mx, my)))
                   .collect();

    let adj2 = keys.par_iter()
                   .map(|k| (coords[k], find_adj_2(k, &coords, mx, my)))
                   .collect();

    return (seats, adj1, adj2);
}