use aoc_rust::{ read_split_on_comma_as_i32 };
use std::collections::HashMap;

fn main() {
    let fish = read_split_on_comma_as_i32(2021, 6);
    println!("Part 1: {}", part1(fish.clone()));
    println!("Part 2: {}", part2(fish.clone()));
}

fn part1(fish: Vec<i32>) -> usize {
    return simulate(fish, 80);
}

fn part2(fish: Vec<i32>) -> usize {
    return simulate(fish, 256);
}

fn simulate(fish: Vec<i32>, days: i32) -> usize {
    let mut fish_counts = fish_to_fish_counts(fish);

    for _ in 0..days {
        for (f, n) in fish_counts.clone().iter() {
            if *f == -1 {continue}
            let lowered = fish_counts.entry(*f-1).or_insert(0);
            *lowered = *n;
        }

        let negative = *fish_counts.entry(-1).or_insert(0);
        let six = fish_counts.entry(6).or_insert(0);
        *six += negative;
        let eight = fish_counts.entry(8).or_insert(0);
        *eight = negative;
    }

    *fish_counts.entry(-1).or_insert(0) = 0;

    return fish_counts.iter().map(| (_, n) | n).sum();
}

fn fish_to_fish_counts(fish: Vec<i32>) -> HashMap<i32, usize> {
    let mut counts: HashMap<i32, usize> = HashMap::new();

    for i in -1..9 {
        counts.insert(
            i,
            fish.iter().filter(| f | **f == i).count()
        );
    }

    return counts;
}