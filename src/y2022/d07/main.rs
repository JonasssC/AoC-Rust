use std::collections::HashMap;
use aoc_rust::util::input::read_string;

fn main() {
    let commands: Vec<Vec<String>> = read_string(2022, 7)
        .split("$ ")
        .map(| s | s.trim().lines()
            .map(| l | l.to_string())
            .collect::<Vec<String>>()
        )
        .collect();
    println!("Part 1: {}", part1(commands.clone()));
    println!("Part 2: {}", part2(commands.clone()));
}

fn calc_dir_sizes(commands: Vec<Vec<String>>) -> HashMap<String, usize> {
    let mut path: Vec<String> = Vec::new();
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();

    for command in commands[1..].iter() {
        match command[0].split_whitespace().collect::<Vec<&str>>()[..] {
            ["cd", dir] => {
                if dir == "/" { path = vec![String::from("")] }
                else if dir == ".." { path.pop(); }
                else { path.push(dir.to_string()) }
            },
            ["ls"] => {
                for file in command[1..].iter() {
                    if !file.starts_with("dir") {
                        let file_size = file.split_once(" ").unwrap().0.parse::<usize>().unwrap();
                        for i in 0..path.len() {
                            let size = dir_sizes.entry(path[0..i+1].join("/")).or_insert(0);
                            *size += file_size;
                        }
                    }
                }
            },
            _ => panic!("Unknown command")
        }
    }

    dir_sizes
}

fn part1(commands: Vec<Vec<String>>) -> usize {
    calc_dir_sizes(commands.clone()).values().filter(| &&size | size <= 100000 ).sum()
}

fn part2(commands: Vec<Vec<String>>) -> usize {
    let dir_sizes = calc_dir_sizes(commands.clone());
    let used_space = *dir_sizes.get("").unwrap();
    *calc_dir_sizes(commands.clone()).values().filter(| &&size | used_space - size <= 40000000).min().unwrap()
}