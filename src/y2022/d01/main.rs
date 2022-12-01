use aoc_rust::{ read_lines_split_on_empty_line_as_i32 };

fn main() {
    let elfs = read_lines_split_on_empty_line_as_i32(2022, 1);
    println!("Part 1: {}", part1(elfs.clone()));
    println!("Part 2: {}", part2(elfs.clone()));
}

fn part1(elfs: Vec<Vec<i32>>) -> i32 {
    let mut most = 0;
    for elf in elfs.iter() {
        let n = elf.iter().sum();
        if n > most {
            most = n;
        }
    }
    return most;
}

fn part2(elfs: Vec<Vec<i32>>) -> i32 {
    let mut weights: Vec<i32> = Vec::new();

    for elf in elfs.iter() {
        weights.push(elf.iter().sum());
    }

    weights.sort();
    weights.reverse();

    return weights[0] + weights[1] + weights[2];
}