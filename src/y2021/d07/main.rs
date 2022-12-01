use aoc_rust::{ read_split_on_comma_as_i32 };

fn main() {
    let crabs = read_split_on_comma_as_i32(2021, 7);
    println!("Part 1: {}", part1(crabs.clone()));
    println!("Part 2: {}", part2(crabs.clone()));
}

fn part1(crabs: Vec<i32>) -> i32 {
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    let mut least_fuel = -1;

    for i in min..max {
        let mut fuel = 0;
        for crab in crabs.iter() {
            fuel += (crab - i).abs();
        }
        if least_fuel == -1 || least_fuel > fuel {
            least_fuel = fuel;
        }
    }

    return least_fuel;
}

fn part2(crabs: Vec<i32>) -> i32 {
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    let mut least_fuel = -1;

    for i in min..max {
        let mut fuel = 0;
        for crab in crabs.iter() {
            for j in 0..(crab - i).abs() {
                fuel += j+1;
            }
            if least_fuel != -1 && fuel > least_fuel {
                break;
            }
        }
        if least_fuel == -1 || least_fuel > fuel {
            least_fuel = fuel;
        }
    }

    return least_fuel;
}