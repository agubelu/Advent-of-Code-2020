use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;
use petgraph::graphmap::DiGraphMap;
use petgraph::Direction;
use regex::Regex;

///////////////////////////////////////////////////////////////////////////////

pub fn run() {
    let time = Instant::now();

    let f = BufReader::new(File::open("../input/day07.txt").unwrap());
    let lines: Vec<String> = f.lines().collect::<Result<_, _>>().unwrap();
    let graph = create_graph(&lines);

    let sol_part_1 = compute_part_1(&graph, "shiny gold");
    let sol_part_2 = compute_part_2(&graph, "shiny gold") - 1; // Exclude the shiny gold bag itself

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}

///////////////////////////////////////////////////////////////////////////////

fn compute_part_1(graph: &DiGraphMap<&str, usize>, initial_node: &str) -> usize {
    let mut visited_nodes: HashSet<&str> = HashSet::new();
    let mut nodes_to_visit: Vec<&str> = graph
        .neighbors_directed(initial_node, Direction::Incoming)
        .collect();

    while !nodes_to_visit.is_empty() {
        let node = nodes_to_visit.pop().unwrap();
        visited_nodes.insert(node);
        nodes_to_visit.extend(graph
            .neighbors_directed(node, Direction::Incoming)
            .filter(|n| !visited_nodes.contains(n))
        );
    }

    return visited_nodes.len();
}

fn compute_part_2(graph: &DiGraphMap<&str, usize>, initial_node: &str) -> usize {
    let mut res = 1;

    for node in graph.neighbors_directed(initial_node, Direction::Outgoing) {
        res += graph.edge_weight(initial_node, node).unwrap() * compute_part_2(graph, node);
    }

    return res;
}

fn create_graph(lines: &[String]) -> DiGraphMap<&str, usize> {
    let re_outer = Regex::new(r"(.*) bags contain").unwrap();
    let re_inner = Regex::new(r"(\d+) (.*?) bags?").unwrap();

    let mut graph = DiGraphMap::new();

    for line in lines {
        let outer_color = re_outer.captures(&line).unwrap().get(1).unwrap().as_str();
        for c in re_inner.captures_iter(&line) {
            let num: usize = c[1].parse().unwrap();
            let inner_color = c.get(2).unwrap().as_str();
            graph.add_edge(outer_color, inner_color, num);
        }
    }

    return graph;
}
