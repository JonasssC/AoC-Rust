use md5::{ Md5, Digest };
use substring::Substring;
use aoc_rust::util::input::read_string;

fn main() {
    let door_id = read_string(2016, 5);
    println!("Part 1: {}", part1(door_id.clone()));
    println!("Part 2: {}", part2(door_id.clone()));
}

fn part1(door_id: String) -> String {
    let mut i: i64 = 0;
    let mut res = String::new();

    while res.len() != 8 {
        let mut hasher = Md5::new();
        hasher.update(format!("{door_id}{i}"));
        let hex_string = hex::encode(hasher.finalize());

        if hex_string.substring(0, 5) == "00000" {
            res.push(hex_string.chars().nth(5).unwrap());
            println!("{res} - {i}");
        }

        i += 1;
    }

    res
}

fn part2(door_id: String) -> String {
    let mut i: i64 = 0;
    let mut res = String::from("________");

    while res.contains("_") {
        let mut hasher = Md5::new();
        hasher.update(format!("{door_id}{i}"));
        let hex_string = hex::encode(hasher.finalize());

        if hex_string.substring(0, 5) == "00000" {
            let pos = hex_string.chars().nth(5).unwrap().to_string().parse::<usize>().unwrap_or(10);
            if pos <= 7 && res.chars().nth(pos).unwrap() == '_' {
                res.replace_range(pos..pos+1, hex_string.chars().nth(6).unwrap().to_string().as_str());
                println!("{res} - {i}");
            }
        }

        i += 1;
    }

    res
}