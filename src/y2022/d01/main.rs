use aoc_rust::util::input::read_lines_split_on_empty_line_as_i32;

fn main() {
    let elfs = read_lines_split_on_empty_line_as_i32(2022, 1);
    println!("Part 1: {}", part1(elfs.clone()));
    println!("Part 2: {}", part2(elfs.clone()));
}

fn part1(elfs: Vec<Vec<i32>>) -> i32 {
    return elfs.iter()
        .map(| elf | elf.iter().sum())
        .max().unwrap();
}

fn part2(elfs: Vec<Vec<i32>>) -> i32 {
    let mut weights: Vec<i32> = elfs.iter()
        .map(| elf | elf.iter().sum())
        .collect();

    weights.sort();

    return weights[weights.len()-3..].iter().sum();
}