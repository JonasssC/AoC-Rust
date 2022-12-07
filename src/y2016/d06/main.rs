use std::collections::HashMap;
use aoc_rust::util::input::read_lines;

fn main() {
    let lines = read_lines(2016, 6);
    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines.clone()));
}

fn part1(codes: Vec<String>) -> String {
    let mut res = String::new();

    for i in 0..codes[0].len() {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for code in codes.iter() {
            let c = code.chars().nth(i).unwrap();
            let count = counts.entry(c).or_insert(0);
            *count += 1;
        }

        let most_common = counts.iter().max_by_key(| &(_, &value) | value ).unwrap();
        res.push(*most_common.0);
    }

    res
}

fn part2(codes: Vec<String>) -> String {
    let mut res = String::new();

    for i in 0..codes[0].len() {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for code in codes.iter() {
            let c = code.chars().nth(i).unwrap();
            let count = counts.entry(c).or_insert(0);
            *count += 1;
        }

        let most_common = counts.iter().min_by_key(| &(_, &value) | value ).unwrap();
        res.push(*most_common.0);
    }

    res
}