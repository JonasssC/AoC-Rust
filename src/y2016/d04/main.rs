use std::char::from_u32;
use std::collections::HashSet;
use aoc_rust::util::input::read_lines;
use aoc_rust::util::string::{count_char, regex_parse};

fn main() {
    let triangles: Vec<(String, i32, String)> = read_lines(2016, 4).iter()
        .map(| l | regex_parse::<String>(l , r"([a-z\-]+)-(\d+)\[([a-z]+)\]"))
        .map(| c | ( c[0].clone(), c[1].parse::<i32>().unwrap(), c[2].clone()))
        .collect();
    println!("Part 1: {}", part1(triangles.clone()));
    println!("Part 2: {}", part2(triangles.clone()));
}

fn part1(rooms: Vec<(String, i32, String)>) -> i32 {
    let mut sum: i32 = 0;

    for room in rooms.iter() {
        let mut unique_chars: Vec<(char, usize)> = room.0.chars()
            .filter(| &c |  c != '-')
            .collect::<HashSet<char>>()
            .iter()
            .map(| &c | (c, count_char(room.0.clone(), c)))
            .collect();

        unique_chars.sort_by(| a, b |
            if a.1 != b.1 {
                b.1.cmp(&a.1)
            } else {
                a.0.cmp(&b.0)
            }
        );

        let mut code = String::from("");

        for i in 0..5 {
            code.push(unique_chars[i].0);
        }

        if code == room.2 {
            sum += room.1;
        }
    }

    sum
}

fn part2(rooms: Vec<(String, i32, String)>) -> i32 {
    for room in rooms.iter() {
        let mut res = room.0.clone();
        for _ in 0..room.1 {
            let mut step = String::from("");
            for mut c in res.chars() {
                if c != '-' && c != 'z' {
                    c = from_u32(c as u32 + 1).unwrap();
                } else if c == 'z' {
                    c = 'a';
                }
                step.push(c);
            }
            res = step;
        }
        if res.contains("north") {
            return room.1
        }
    }

    -1
}