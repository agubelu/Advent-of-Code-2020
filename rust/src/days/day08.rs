use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;

use petgraph::graph::{DiGraph, NodeIndex, EdgeIndex};
use petgraph::visit::EdgeRef;
use petgraph::algo::has_path_connecting;

///////////////////////////////////////////////////////////////////////////////

type EdgeInfo = (EdgeIndex, NodeIndex<u32>, NodeIndex<u32>, NodeIndex<u32>);
#[derive(Clone)]
enum InstrType {
    Add { val: i32 },
    Jmp { val: i32 },
    Nop { val: i32 },
}

///////////////////////////////////////////////////////////////////////////////

pub fn run() {
    let time = Instant::now();

    let f = BufReader::new(File::open("../input/day08.txt").unwrap());
    let (mut graph, n_instructions) = get_flow_graph(f);

    let source = NodeIndex::new(0);
    let target = NodeIndex::<u32>::new(n_instructions - 1);

    // Part 1 + aux for Part 2
    let (sol_part_1, candidate_changes) = navigate_until_loop(&graph, source, n_instructions);

    // Part 2
    for (edge_id, orig_src, orig_tgt, cand_tgt) in candidate_changes {
        graph.remove_edge(edge_id);
        let new_edge_id = graph.add_edge(orig_src, cand_tgt, InstrType::Nop{val: 0});
        
        if has_path_connecting(&graph, source, target, None) {
            break;
        }
        
        graph.remove_edge(new_edge_id);
        graph.add_edge(orig_src, orig_tgt, InstrType::Nop{val: 0});
    }
    let sol_part_2 = navigate_yolo(&graph, source, target);

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}

///////////////////////////////////////////////////////////////////////////////

fn navigate_until_loop(
    graph: &DiGraph<u32, InstrType>, 
    source: NodeIndex,
    n_instructions: usize
) 
    -> (i32, Vec<EdgeInfo>) 
{
    let mut cur_node = source;
    let mut visited_nodes: HashSet<NodeIndex> = HashSet::new();
    let mut candidate_edges: Vec<_> = Vec::new();
    let mut acc: i32 = 0;

    while !visited_nodes.contains(&cur_node) {
        visited_nodes.insert(cur_node);
        let out_edge = graph.edges(cur_node).next().unwrap(); // Each node has exactly 1 outgoing edge
        let target = out_edge.target();
        match out_edge.weight() {
            InstrType::Add{val} => acc += val,
            InstrType::Jmp{val: _} => candidate_edges.push(
                (out_edge.id(), cur_node, target, NodeIndex::new(cur_node.index() + 1))
            ),
            InstrType::Nop{val} => {
                let jmp = (cur_node.index() as i32 + val) as usize;
                if jmp < n_instructions {
                     candidate_edges.push((out_edge.id(), cur_node, target, NodeIndex::new(jmp)))
                }
            },
        }
        cur_node = target;
    }

    return (acc, candidate_edges);
}

fn navigate_yolo(graph: &DiGraph<u32, InstrType>, source: NodeIndex, target: NodeIndex) -> i32 {
    let mut cur_node = source;
    let mut acc: i32 = 0;

    loop {
        let out_edge = graph.edges(cur_node).next().unwrap(); // Each node has exactly 1 outgoing edge
        if let InstrType::Add{val} = out_edge.weight() {
            acc += val;
        }

        if cur_node == target {
            break;
        } else {
            cur_node = out_edge.target();
        }
    }

    return acc;
}

fn get_flow_graph(f: BufReader<File>) -> (DiGraph<u32, InstrType>, usize) {
    let lines: Vec<String> = f.lines().collect::<Result<_, _>>().unwrap();
    let n_instructions = lines.len();

    let edges: Vec<(NodeIndex, NodeIndex, InstrType)> = lines.iter()
        .enumerate()
        .map(|(i, line)| {
            let val: i32 = line[4..].parse().unwrap();
            let (instr, target) = match &line[..3] {
                "nop" => (InstrType::Nop{val}, i + 1),
                "jmp" => (InstrType::Jmp{val}, (i as i32 + val) as usize),
                "acc" => (InstrType::Add{val}, i + 1),
                _ => panic!("bruh")
            };
            (NodeIndex::new(i), NodeIndex::new(target), instr)
        })
        .filter(|(_, target, _)| target.index() <= n_instructions)
        .collect();
    return (DiGraph::from_edges(&edges), n_instructions);
}
