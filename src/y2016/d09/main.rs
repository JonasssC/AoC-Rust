use regex::Regex;
use aoc_rust::util::input::read_string;

fn main() {
    let encoded = read_string(2016, 9);
    println!("Part 1: {}", part1(encoded.clone()));
    println!("Part 2: {}", part2(encoded.clone()));
}

fn parse_rep_size(s: &mut String) -> (usize, usize) {
    let re = Regex::new(r"\((\d+)x(\d+)\)").expect("Invalid RegEx");
    let clone = s.clone();
    let caps = re.captures(clone.as_str()).unwrap();

    *s = s.split_off(caps.get(0).unwrap().as_str().len());

    (
        caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        caps.get(2).unwrap().as_str().parse::<usize>().unwrap()
    )
}

fn decode1(mut encoded: String) -> String {
    let mut decoded = String::new();

    while !encoded.is_empty() {
        let i = encoded.find("(").unwrap_or(encoded.len());
        if i == 0 {
            let (size, n) = parse_rep_size(&mut encoded);
            let encoded_new = encoded.split_off(size);
            for _ in 0..n {
                decoded.push_str(&*encoded);
            }
            encoded = encoded_new;
        } else {
            let encoded_new = encoded.split_off(i);
            decoded.push_str(&*encoded);
            encoded = encoded_new;
        }
    }

    decoded
}

fn part1(encoded: String) -> usize {
    decode1(encoded).len()
}

fn decode2(mut encoded: String) -> String {
    let mut decoded = String::new();

    while !encoded.is_empty() {
        let i = encoded.find("(").unwrap_or(encoded.len());
        if i == 0 {
            let (size, n) = parse_rep_size(&mut encoded);
            let encoded_new = encoded.split_off(size);
            let sub_decoded = decode2(encoded.clone());
            for _ in 0..n {
                decoded.push_str(&*sub_decoded);
            }
            encoded = encoded_new;
        } else {
            let encoded_new = encoded.split_off(i);
            decoded.push_str(&*encoded);
            encoded = encoded_new;
        }
    }

    decoded
}

fn part2(encoded: String) -> usize {
    decode2(encoded).len()
}