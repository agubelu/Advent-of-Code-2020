use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;

use crate::etc::d18::{
    line_to_expr,
    eval_expr,
    ExprToken,
    ExprToken::*
};

///////////////////////////////////////////////////////////////////////////////

pub fn run() {
    let time = Instant::now();
    
    let f = BufReader::new(File::open("../input/day18.txt").unwrap());
    let lines: Vec<String> = f.lines().map(|line| line.unwrap().replace(" ", "")).collect();

    let priorities_1: HashMap<ExprToken, u8> = vec![(Sum, 1), (Mult, 1)].into_iter().collect();
    let priorities_2: HashMap<ExprToken, u8> = vec![(Sum, 2), (Mult, 1)].into_iter().collect();

    let sol_part_1 = get_sol(&lines, &priorities_1);
    let sol_part_2 = get_sol(&lines, &priorities_2);

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}

///////////////////////////////////////////////////////////////////////////////

fn get_sol(lines: &[String], priorities: &HashMap<ExprToken, u8>) -> u64 {
    return lines.iter()
                .map(|line| line_to_expr(line, &priorities))
                .map(|expr| eval_expr(&expr))
                .sum();
}
