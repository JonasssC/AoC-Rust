use aoc_rust::{ read_lines_as_i32 };

fn main() {
    let lines = read_lines_as_i32(2021, 1);
    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines.clone()));
}

fn part1(lines: Vec<i32>) -> i32 {
    let mut count = 0;

    for i in 1..lines.len() {
        if lines[i] > lines[i-1] {
            count += 1;
        }
    }

    return count;
}

fn part2(lines: Vec<i32>) -> i32 {
    let mut count = 0;

    for i in 3..lines.len() {
        if lines[i] + lines[i-1] + lines[i-2] > lines[i-1] + lines[i-2] + lines[i-3] {
            count += 1;
        }
    }

    return count;
}
