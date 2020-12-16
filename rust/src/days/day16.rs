use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;
use std::ops::RangeInclusive;

use regex::Regex;
use rayon::prelude::*;

///////////////////////////////////////////////////////////////////////////////

const RE_FIELD: &str = r"(.*): (\d+)-(\d+) or (\d+)-(\d+)";

type IntType = u32;

struct Field {
    name: String,
    range1: RangeInclusive<IntType>,
    range2: RangeInclusive<IntType>,
}

impl Field {
    pub fn accepts_value(&self, val: IntType) -> bool {
        self.range1.contains(&val) || self.range2.contains(&val)
    }
}

struct Ticket { vals: Vec<IntType> }
impl Ticket {
    pub fn get_invalid_vals_sum(&self, fields: &Vec<Field>) -> IntType {
        self.vals.iter().filter(|v| {
            fields.iter().all(|f| !f.accepts_value(**v))
        }).sum()
    }
}

///////////////////////////////////////////////////////////////////////////////

pub fn run() {
    let time = Instant::now();
    let f = BufReader::new(File::open("../input/day16.txt").unwrap());

    let (fields, my_ticket, tickets) = process_input(f);

    let (valid_tickets, sol_part_1) = filter_tickets(tickets, &fields);
    let sol_part_2 = 2;
    
    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}

///////////////////////////////////////////////////////////////////////////////

fn filter_tickets(tickets: Vec<Ticket>, fields: &Vec<Field>) -> (Vec<Ticket>, IntType) {
    let mut sol_part_1 = 0;
    let mut valid_tickets = Vec::new();

    for t in tickets {
        let v = t.get_invalid_vals_sum(&fields);
        if v == 0 {
            valid_tickets.push(t);
        } else {
            sol_part_1 += v;
        }
    }

    return (valid_tickets, sol_part_1);
}

fn process_input(f: BufReader<File>) -> (Vec<Field>, Ticket, Vec<Ticket>) {
    let mut fields = Vec::new();
    let mut tickets = Vec::new();
    let regex_field = Regex::new(&RE_FIELD).unwrap();
    
    let mut line: String;
    let mut lines = f.lines();

    while {line = lines.next().unwrap().unwrap(); line != ""} {
        let caps = regex_field.captures(&line).unwrap();
        let name = caps[1].to_owned();
        let ns: Vec<IntType> = (2..=5).map(|i| caps[i].parse().unwrap()).collect();
        fields.push(Field{name, range1: ns[0]..=ns[1], range2: ns[2]..=ns[3]});
    }

    // Skip 1 and parse my ticket
    lines.next();
    let my_ticket = line_to_ticket(&lines.next().unwrap().unwrap());

    // Skip 2 and parse the other tickets
    lines.next(); lines.next();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let ticket = line_to_ticket(&line);
        tickets.push(ticket);
    }

    return (fields, my_ticket, tickets);
}

fn line_to_ticket(s: &String) -> Ticket {
    let vals: Vec<IntType> = s.split(',')
                              .map(|x| x.parse().unwrap())
                              .collect();
    return Ticket{vals};
}