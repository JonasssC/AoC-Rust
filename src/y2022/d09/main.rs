use std::collections::HashSet;
use aoc_rust::util::input::read_lines;

fn main() {
    let lines = read_lines(2022, 9);
    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines.clone()));
}

fn step_head(head: &mut (i32, i32), instruction: &str) {
    match instruction {
        "U" => head.0 += 1,
        "D" => head.0 -= 1,
        "R" => head.1 += 1,
        "L" => head.1 -= 1,
        _ => panic!("Invalid instruction")
    }
}

fn follow_tail(head: (i32, i32), tail: &mut (i32, i32)) {
    if (head.0 - tail.0).abs() > 1 {
        if head.0 > tail.0 {
            tail.0 += 1;
        } else {
            tail.0 -= 1;
        }
        if tail.1 > head.1 {
            tail.1 -= 1;
        } else if tail.1 < head.1 {
            tail.1 += 1;
        }
    } else if (head.1 - tail.1).abs() > 1 {
        if head.1 > tail.1 {
            tail.1 += 1;
        } else {
            tail.1 -= 1;
        }
        if tail.0 > head.0 {
            tail.0 -= 1;
        } else if tail.0 < head.0 {
            tail.0 += 1;
        }
    }
}

fn part1(lines: Vec<String>) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    visited.insert(tail.clone());

    for line in lines.iter() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let n = split[1].parse::<usize>().unwrap();

        for _ in 0..n {
            step_head(&mut head, split[0]);
            follow_tail(head, &mut tail);
            visited.insert(tail.clone());
        }
    }

    visited.len()
}

fn part2(lines: Vec<String>) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut knots = [(0, 0); 10];
    visited.insert(knots[knots.len() - 1].clone());

    for line in lines.iter() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let n = split[1].parse::<usize>().unwrap();

        for _ in 0..n {
            step_head(&mut knots[0], split[0]);
            for i in 1..knots.len() {
                follow_tail(knots[i-1], &mut knots[i]);
            }
            visited.insert(knots[knots.len() - 1].clone());
        }

    }

    visited.len()
}