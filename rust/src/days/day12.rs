use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;
use std::mem::swap;

///////////////////////////////////////////////////////////////////////////////

const DIRS: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub enum Instr {
    North {val: i64},
    South {val: i64},
    West {val: i64},
    East {val: i64},
    Right {val: i64},
    Left {val: i64},
    Forward {val: i64},
}

use Instr::*;

///////////////////////////////////////////////////////////////////////////////

pub fn run() {
    let time = Instant::now();

    let f = BufReader::new(File::open("../input/day12.txt").unwrap());
    let instructions = get_instructions(f);

    let (sol_part_1, sol_part_2) = get_sols(&instructions);
    
    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}

///////////////////////////////////////////////////////////////////////////////

// Geting both solutions at once is slightly more efficient, especially
// for the big boy input
fn get_sols(instructions: &[Instr]) -> (i64, i64) {
    let mut heading = 1;
    let mut pos_x_1 = 0;
    let mut pos_y_1 = 0;

    let mut pos_x_2 = 0;
    let mut pos_y_2 = 0;
    let mut pos_x_wpt = 10;
    let mut pos_y_wpt = -1;

    for instr in instructions {
        match instr {
            North{val} => {
                pos_y_1 -= val;
                pos_y_wpt -= val;
            },
            South{val} => {
                pos_y_1 += val;
                pos_y_wpt += val;
            },
            West{val} => {
                pos_x_1 -= val;
                pos_x_wpt -= val;
            },
            East{val} => {
                pos_x_1 += val;
                pos_x_wpt += val;
            },
            Right{val} => {
                let val = val / 90;
                heading = (heading + val).rem_euclid(4);

                for _ in 0..val {
                    swap(&mut pos_x_wpt, &mut pos_y_wpt);
                    pos_x_wpt *= -1;
                }
            },
            Left{val} => {
                let val = val / 90;
                heading = (heading - val).rem_euclid(4);

                for _ in 0..val {
                    swap(&mut pos_x_wpt, &mut pos_y_wpt);
                    pos_y_wpt *= -1;
                }
            },
            Forward{val} => {
                let dir = DIRS[heading as usize];
                pos_x_1 += val * dir.0;
                pos_y_1 += val * dir.1;
                pos_x_2 += val * pos_x_wpt;
                pos_y_2 += val * pos_y_wpt;
            }
        }
    }

    return (pos_x_1.abs() + pos_y_1.abs(), pos_x_2.abs() + pos_y_2.abs());
}

fn get_instructions(f: BufReader<File>) -> Vec<Instr> {
    f.lines()
     .map(|line| {
        let line = line.unwrap();
        let val: i64 = line[1..].parse().unwrap();
        match &line[0..1] {
            "N" => North{val},
            "S" => South{val},
            "W" => West{val},
            "E" => East{val},
            "R" => Right{val},
            "L" => Left{val},
            "F" => Forward{val},
             _  => panic!("wtf"),
        }
     })
     .collect()
}