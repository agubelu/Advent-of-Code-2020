use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;

use counter::Counter;

///////////////////////////////////////////////////////////////////////////////

type Coord = i16;
type CubeCoords = (Coord, Coord, Coord, Coord);

///////////////////////////////////////////////////////////////////////////////

pub fn run() {
    let time = Instant::now();
    
    let f = BufReader::new(File::open("../input/day17.txt").unwrap());
    let initial = get_initial_cubes(f);

    let sol_part_1 = get_sol(&initial, 3);
    let sol_part_2 = get_sol(&initial, 4);
    
    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}

///////////////////////////////////////////////////////////////////////////////

fn get_sol(state: &HashSet<CubeCoords>, n_dims: usize) -> usize {
    let mut state = state.clone();

    for _ in 0..6 {
        let counter = state.iter()
                           .flat_map(|cube| get_neighbors(cube, n_dims))
                           .collect::<Counter<_>>();

        let mut stay_on: HashSet<CubeCoords> = state.iter()
                                                .filter(|cube| {
                                                    let cnt = counter[cube];
                                                    (2..=3).contains(&cnt)
                                                })
                                                .copied()
                                                .collect();
        let become_on = counter.iter()
                               .filter(|(coords, cnt)| **cnt == 3 && !state.contains(coords))
                               .map(|(coords, _)| *coords);
        
        stay_on.extend(become_on);
        state = stay_on;
    }

    return state.len();
}

fn get_neighbors(cube: &CubeCoords, dim: usize) -> Vec<CubeCoords> {
    let cap = if dim == 3 { 26 } else { 80 };
    let mut res = Vec::with_capacity(cap);

    let wstart = if dim == 3 { 0 } else { -1 };
    let wend = if dim == 3 { 0 } else { 1 };

    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                for dw in wstart..=wend {
                    if dx != 0 || dy != 0 || dz != 0 || dw != 0 {
                        res.push((cube.0 + dx, cube.1 + dy, cube.2 + dz, cube.3 + dw));
                    }
                }
            }
        }
    }

    return res;
}

fn get_initial_cubes(f: BufReader<File>) -> HashSet<CubeCoords> {
    let mut res = HashSet::new();

    for (y, line) in f.lines().enumerate() {
        for (x, ch) in line.unwrap().chars().enumerate() {
            if ch == '#' {
                res.insert((x as Coord, y as Coord, 0, 0));
            }
        }
    }

    return res;
}