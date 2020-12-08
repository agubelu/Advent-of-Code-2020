use std::boxed::Box;
use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;
use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::visit::Dfs;

use crate::utils::console_day08::{HandheldConsole, Instruction};

///////////////////////////////////////////////////////////////////////////////

pub fn run() -> Result<(), Box<dyn Error>> {
    let time = Instant::now();

    let f = BufReader::new(File::open("../input/day08.txt").unwrap());
    let mut instructions: Vec<Instruction> = f
        .lines()
        .map(|line| line_to_instruction(&line.unwrap()))
        .collect();
    let graph = instruction_flow_graph(&instructions);
    let console = HandheldConsole{ instructions: &instructions };

    // Part 1
    let sol_part_1 = console.run_until_loop();

    // Part 2
    let last_node = instructions.len() - 1;
    let mut dfs_end = Dfs::new(&graph, NodeIndex::new(last_node));
    let mut comp_end: HashSet<usize> = HashSet::new();
    while let Some(nx) = dfs_end.next(&graph) {
        comp_end.insert(nx.index());
    }

    let mut dfs_start = Dfs::new(&graph, NodeIndex::new(0));
    while let Some(nx) = dfs_start.next(&graph) {
        let index = nx.index();
        let (alt_target, alt_instr) = match instructions[index] {
            Instruction::Add{val: _} => continue,
            Instruction::Nop{val} => ((index as i32 + val) as usize, Instruction::Jmp{val}),
            Instruction::Jmp{val} => (index + 1, Instruction::Nop{val})
        };

        if comp_end.contains(&alt_target) {
            instructions[index] = alt_instr;
        }
    }

    let console = HandheldConsole{ instructions: &instructions };
    let sol_part_2 = console.run_yolo();

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////

fn line_to_instruction(line: &String) -> Instruction {
    let val: i32 = line[4..].parse().unwrap();
    match &line[..3] {
        "nop" => Instruction::Nop{val},
        "jmp" => Instruction::Jmp{val},
        "acc" => Instruction::Add{val},
        _ => panic!("wtf")
    }
}

fn instruction_flow_graph(instructions: &Vec<Instruction>) -> UnGraph<usize, ()> {
    let edges: Vec<(NodeIndex, NodeIndex)> = instructions
        .iter()
        .enumerate()
        .map(|(i, instr)| {
            let target = match instr {
                Instruction::Jmp{val} => NodeIndex::new(i + *val as usize),
                _ => NodeIndex::new(i + 1)
            };
            (NodeIndex::new(i), target)
        }).collect();
    return UnGraph::from_edges(&edges);
}
