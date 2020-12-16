use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;
use std::ops::RangeInclusive;

///////////////////////////////////////////////////////////////////////////////

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
    pub fn get_invalid_vals_sum(&self, fields: &Vec<Field>) -> Option<IntType> {
        let filt: Vec<&IntType> = self.vals.iter().filter(|v| {
            fields.iter().all(|f| !f.accepts_value(**v))
        }).collect();

        if filt.is_empty() {
            return None
        } else {
            return Some(filt.into_iter().sum());
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// Limitation: this only works for 64 or less fields

pub fn run() {
    let time = Instant::now();
    
    let f = BufReader::new(File::open("../input/day16.txt").unwrap());
    let (fields, my_ticket, tickets) = process_input(f);
    let (valid_tickets, sol_part_1) = filter_tickets(tickets, &fields);

    let possible_fields: Vec<Vec<u64>> = valid_tickets.iter().map(|t| ticket_to_fields(&t, &fields)).collect();
    let field_mapping = disambiguate_fields(possible_fields);
    let sol_part_2 = get_sol_2(&fields, &field_mapping, &my_ticket);
    
    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}

///////////////////////////////////////////////////////////////////////////////

fn get_sol_2(fields: &Vec<Field>, mapping: &Vec<u64>, my_ticket: &Ticket) -> u64 {
    my_ticket.vals.iter()
        .enumerate()
        .filter(|(i, _)| {
            let ind = mapping[*i].trailing_zeros() as usize;
            fields[ind].name.starts_with("departure")
        })
        .fold(1, |a, (_, b)| a * *b as u64)
}

fn disambiguate_fields(fs: Vec<Vec<u64>>) -> Vec<u64> {
    let n_tickets = fs.len();
    let n_fields = fs[0].len();
    let mut res = Vec::new();

    for i in 0..n_fields {
        let mut f = fs[0][i];
        for j in 1..n_tickets {
            f &= fs[j][i];
        }
        res.push(f);
    }

    loop {
        let mut changed = false;

        for i in 0..n_fields {
            let val = res[i];
            if val.count_ones() == 1 {
                for j in 0..n_fields {
                    if i != j && res[j].count_ones() > 1 {
                        res[j] &= !val;
                        changed = true;
                    }
                }
            }
        }

        if !changed {
            break;
        }
    }

    return res;
}

fn ticket_to_fields(ticket: &Ticket, fields: &Vec<Field>) -> Vec<u64> {
    let n_fields = fields.len();
    ticket.vals.iter()
        .map(|val| {
            let mut f = 0;
            for i in 0..n_fields {
                if fields[i].accepts_value(*val) {
                    f |= 1 << i;
                }
            }
            
            return f;
        })
        .collect()
}

fn filter_tickets(tickets: Vec<Ticket>, fields: &Vec<Field>) -> (Vec<Ticket>, IntType) {
    let mut sol_part_1 = 0;
    let mut valid_tickets = Vec::new();

    for t in tickets {
        let v = t.get_invalid_vals_sum(&fields);
        match v {
            Some(v) => sol_part_1 += v,
            None => valid_tickets.push(t),
        };
    }

    return (valid_tickets, sol_part_1);
}

fn process_input(f: BufReader<File>) -> (Vec<Field>, Ticket, Vec<Ticket>) {
    let mut fields = Vec::new();
    let mut tickets = Vec::new();
    //let regex_field = Regex::new(&RE_FIELD).unwrap();
    
    let mut line: String;
    let mut lines = f.lines();

    while {line = lines.next().unwrap().unwrap(); line != ""} {
        let field = line_to_field(&line);
        fields.push(field);
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

// Annoying to code, but fast
fn line_to_field(s: &String) -> Field {
    let mut chars = s.chars();
    let mut name = String::new();
    let mut ch: char;

    let mut rng1_start = 0;
    let mut rng1_end = 0;
    let mut rng2_start = 0;
    let mut rng2_end = 0;

    while {ch = chars.next().unwrap(); ch != ':'} {
        name.push(ch);
    }
    chars.next();

    while {ch = chars.next().unwrap(); ch != '-'} {
        rng1_start = rng1_start * 10 + ch.to_digit(10).unwrap();
    }

    while {ch = chars.next().unwrap(); ch != ' '} {
        rng1_end = rng1_end * 10 + ch.to_digit(10).unwrap();
    }

    // Skip 3
    chars.next(); chars.next(); chars.next();

    while {ch = chars.next().unwrap(); ch != '-'} {
        rng2_start = rng2_start * 10 + ch.to_digit(10).unwrap();
    }

    while let Some(ch) = chars.next() {
        rng2_end = rng2_end * 10 + ch.to_digit(10).unwrap();
    }

    return Field{name, range1: rng1_start..=rng1_end, range2: rng2_start..=rng2_end};
}

fn line_to_ticket(s: &String) -> Ticket {
    let vals: Vec<IntType> = s.split(',')
                              .map(|x| x.parse().unwrap())
                              .collect();
    return Ticket{vals};
}