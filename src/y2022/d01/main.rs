use aoc_rust::{ read_split_on_empty_line };

fn main() {
    let elfs = read_split_on_empty_line(2022, 1);
    println!("Part 1: {}", part1(elfs.clone()));
    println!("Part 2: {}", part2(elfs.clone()));
}

fn part1(elfs: Vec<String>) -> i32 {
    let mut most = 0;
    for elf in elfs.iter() {
        let n = count_calories(elf);
        if n > most {
            most = n;
        }
    }
    return most;
}

fn part2(elfs: Vec<String>) -> i32 {
    let mut weights: Vec<i32> = Vec::new();

    for elf in elfs.iter() {
        weights.push(count_calories(elf));
    }

    weights.sort();
    weights.reverse();

    return weights[0] + weights[1] + weights[2];
}

fn count_calories(elf: &String) -> i32 {
    elf.split("\n")
        .map(| s | s.parse::<i32>().unwrap())
        .sum()
}