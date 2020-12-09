use std::io::{BufReader, BufRead};
use std::fs::File;
use std::time::Instant;
use regex::Regex;

const RE_LIN: &str = r"([a-z]{3}):(.*?)(?:\s|$)";
const RE_BYR: &str = r"^(19[2-9]\d|200[0-2])$";
const RE_IYR: &str = r"^(201\d|2020)$";
const RE_EYR: &str = r"^(202\d|2030)$";
const RE_HGT: &str = r"^(1([5-8]\d|9[0-3])cm|(59|6\d|7[0-6])in)$";
const RE_HCL: &str = r"^#[0-9a-f]{6}$";
const RE_ECL: &str = r"^(amb|blu|brn|gry|grn|hzl|oth)$";
const RE_PID: &str = r"^[0-9]{9}$";

///////////////////////////////////////////////////////////////////////////////
// This one could be much more efficient... I'll get back to it some day, maybe

pub fn run() {
    let time = Instant::now();

    let f = BufReader::new(File::open("../input/day04.txt").unwrap());
    let mut sol_part_1 = 0;
    let mut sol_part_2 = 0;

    let mut acc1 = 0;
    let mut acc2 = 0;

    let regex_lin = Regex::new(RE_LIN).unwrap();
    let regex_byr = Regex::new(RE_BYR).unwrap();
    let regex_iyr = Regex::new(RE_IYR).unwrap();
    let regex_eyr = Regex::new(RE_EYR).unwrap();
    let regex_hgt = Regex::new(RE_HGT).unwrap();
    let regex_hcl = Regex::new(RE_HCL).unwrap();
    let regex_ecl = Regex::new(RE_ECL).unwrap();
    let regex_pid = Regex::new(RE_PID).unwrap();

    for line in f.lines() {
        let line = line.unwrap();
        if line == "" {
            if acc1 == 7 { sol_part_1 += 1; }
            if acc2 == 7 { sol_part_2 += 1; }
            acc1 = 0;
            acc2 = 0;
        }

        for cap in regex_lin.captures_iter(&line) {
            let (key, val) = (cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str());
            let re = match key {
                "byr" => &regex_byr,
                "iyr" => &regex_iyr,
                "eyr" => &regex_eyr,
                "hgt" => &regex_hgt,
                "hcl" => &regex_hcl,
                "ecl" => &regex_ecl,
                "pid" => &regex_pid,
                "cid" => continue,
                _ => panic!("unknown key"),
            };

            acc1 += 1;
            if re.is_match(val) {
                acc2 += 1;
            }
        }
    }

    if acc1 == 7 { sol_part_1 += 1; }
    if acc2 == 7 { sol_part_2 += 1; }

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("Part 1: {}", sol_part_1);
    println!("Part 2: {}", sol_part_2);
    println!("Elapsed: {:.3} ms", elapsed_ms);
}
