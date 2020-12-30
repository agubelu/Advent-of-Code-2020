use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;

use regex::Regex;

use crate::etc::d19::{
    Rule,
    RuleIndex,
    RuleCombo,
};

///////////////////////////////////////////////////////////////////////////////

const RE_RULE: &str = r"(\d+): (.*)";

///////////////////////////////////////////////////////////////////////////////

pub fn run() {
    let time = Instant::now();
    
    let f = BufReader::new(File::open("../input/day19.txt").unwrap());
    let (rules, lines) = read_input(f);

    let r42 = rules[&42].to_regex(&rules);
    let r31 = rules[&31].to_regex(&rules);
    let re_text_part_1 = format!("^{}{}{}$", r42, r42, r31);
    let re_part_1 = Regex::new(&re_text_part_1).unwrap();
    let sol_part_1 = lines.iter().filter(|line| re_part_1.is_match(line)).count();

    // Somewhat ad-hoc stuff, especially for the change in rule 11
    let r11 = format!("({}{}|{}{}{}{}|{}{}{}{}{}{}|{}{}{}{}{}{}{}{})", 
                        r42, r31, 
                        r42, r42, r31, r31,
                        r42, r42, r42, r31, r31, r31,
                        r42, r42, r42, r42, r31, r31, r31, r31,);
    let re_text_part_2 = format!("^({})+{}$", r42, r11);
    let re_part_2 = Regex::new(&re_text_part_2).unwrap();
    let sol_part_2 = lines.iter().filter(|line| re_part_2.is_match(line)).count();

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}

///////////////////////////////////////////////////////////////////////////////

fn read_input(f: BufReader<File>) -> (HashMap<RuleIndex, Rule>, Vec<String>) {
    let mut lines = f.lines();
    let mut rules = HashMap::new();
    let mut line;

    while {line = lines.next().unwrap().unwrap(); line != ""} {
        let re = Regex::new(RE_RULE).unwrap();
        let (ind, rule) = read_rule(&line, &re);
        rules.insert(ind, rule);
    };

    let inputs = lines.map(Result::unwrap).collect();
    return (rules, inputs);
}

fn read_rule(line: &String, re: &Regex) -> (RuleIndex, Rule) {
    let m = re.captures(&line).unwrap();
    let index: RuleIndex = m[1].parse().unwrap();
    let body = m.get(2).unwrap().as_str();

    if body.starts_with("\"") {
        let val = String::from(&body[1..2]);
        return (index, Rule::Terminal{val});
    }

    let mut options: Vec<RuleCombo> = Vec::new();
    for spl in body.split("|") {
        let mut combo: RuleCombo = Vec::new();
        for ind in spl.trim().split(" ") {
            combo.push(ind.parse().unwrap());
        }
        options.push(combo);
    }

    return (index, Rule::Composite{options});
}