use std::collections::HashSet;
use aoc_rust::util::input::read_lines;

fn main() {
    let lines = read_lines(2022, 14);
    let occupied_coords = occupied_coords(lines.clone());
    println!("Part 1: {}", part1(occupied_coords.clone()));
    println!("Part 2: {}", part2(occupied_coords.clone()));
}

fn occupied_coords(lines: Vec<String>) -> HashSet<(i32, i32)> {
    let mut res = HashSet::new();

    for line in lines.iter() {
        let coords = line.split(" -> ")
            .map(| c | c.split_once(",").unwrap())
            .map(| c | (c.0.parse::<i32>().unwrap(), c.1.parse::<i32>().unwrap()))
            .collect::<Vec<(i32, i32)>>();

        for i in 1..coords.len() {
            let step = (
                (coords[i].0 - coords[i-1].0).signum(),
                (coords[i].1 - coords[i-1].1).signum()
            );

            let mut c = coords[i-1];
            while c != coords[i] {
                res.insert(c);
                c = (c.0 + step.0, c.1 + step.1);
            }
            res.insert(c);
        }
    }

    res
}

fn part1(mut occupied_coords: HashSet<(i32, i32)>) -> usize {
    let mut unit_count = 0;

    let max_y = occupied_coords.iter().map(|c | c.1).max().unwrap();

    'outer_loop: loop {
        let mut sand = (500, 0);
        loop {
            if !occupied_coords.contains(&(sand.0, sand.1 + 1)) {
                sand = (sand.0, sand.1 + 1);
            } else if !occupied_coords.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1)
            } else if !occupied_coords.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1)
            } else {
                occupied_coords.insert(sand);
                unit_count += 1;
                break;
            }

            if sand.1 >= max_y {
                break 'outer_loop;
            }
        }
    }

    unit_count
}

fn part2(mut occupied_coords: HashSet<(i32, i32)>)  -> usize {
    let mut unit_count = 0;

    let floor = occupied_coords.iter().map(|c| c.1).max().unwrap()+2;

    'outer_loop: loop {
        let mut sand = (500, 0);
        loop {
            if !occupied_coords.contains(&(sand.0, sand.1 + 1)) && sand.1 + 1 != floor {
                sand = (sand.0, sand.1 + 1);
            } else if !occupied_coords.contains(&(sand.0 - 1, sand.1 + 1)) && sand.1 + 1 != floor {
                sand = (sand.0 - 1, sand.1 + 1)
            } else if !occupied_coords.contains(&(sand.0 + 1, sand.1 + 1)) && sand.1 + 1 != floor {
                sand = (sand.0 + 1, sand.1 + 1)
            } else {
                occupied_coords.insert(sand);
                unit_count += 1;
                if sand == (500, 0) {
                    break 'outer_loop;
                }
                break;
            }
        }
    }

    unit_count
}