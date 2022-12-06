use aoc_rust::util::input::read_lines;

#[derive(Clone, Debug)]
struct Path {
    path: Vec<String>,
    has_small_double: bool
}

impl Path {
    fn get_last(&self) -> String {
        self.path[self.path.len() - 1].clone()
    }

    fn expand_1(&self, connections: Vec<(String, String)>) -> Vec<Path> {
        let mut res: Vec<Path> = Vec::new();

        for dest in get_possible_destinations(connections.clone(), self.get_last()) {
            if dest.to_lowercase() == dest && dest != String::from("end") && self.path.contains(&dest) {
                continue;
            }
            let mut copied_path = self.clone();
            copied_path.path.push(dest.clone());
            res.push(copied_path);
        }

        res
    }

    fn expand_2(&self, connections: Vec<(String, String)>) -> Vec<Path> {
        let mut res: Vec<Path> = Vec::new();

        for dest in get_possible_destinations(connections.clone(), self.get_last()) {
            if dest == "start" {
                continue;
            }
            let mut copied_path = self.clone();
            if dest.to_lowercase() == dest && dest != String::from("end") && self.path.contains(&dest) {
                if self.has_small_double {
                    continue;
                } else {
                    copied_path.has_small_double = true;
                }
            }
            copied_path.path.push(dest.clone());
            res.push(copied_path);
        }

        res
    }
}

fn main() {
    let connections: Vec<(String, String)> = read_lines(2021, 12).iter()
        .map(| l | l.split_once('-').unwrap())
        .map(| (a, b) | (a.to_string(), b.to_string()))
        .collect();
    println!("Part 1: {}", part1(connections.clone()));
    println!("Part 2: {}", part2(connections.clone()));
}

fn get_possible_destinations(connections: Vec<(String, String)>, from: String) -> Vec<String> {
    connections.iter()
        .map(| c | if c.0 == from { c.1.clone() }
                    else if c.1 == from { c.0.clone() }
                    else { String::from("") }
        )
        .filter(| r | r != "")
        .collect()
}

fn part1(connections: Vec<(String, String)>) -> usize {
    let base_path = Path {
        path: vec![String::from("start")],
        has_small_double: false
    };

    let mut res: Vec<Path> = Vec::new();

    let mut possible: Vec<Path> = base_path.expand_1(connections.clone());

    while !possible.is_empty() {
        let path = possible.pop().unwrap();
        for exp_path in path.expand_1(connections.clone()) {
            if exp_path.path[exp_path.path.len() - 1] == "end" {
                res.push(exp_path);
            } else {
                possible.push(exp_path);
            }
        }
    }

    res.len()
}

fn part2(connections: Vec<(String, String)>) -> usize {
    let base_path = Path {
        path: vec![String::from("start")],
        has_small_double: false
    };

    let mut res: Vec<Path> = Vec::new();

    let mut possible: Vec<Path> = base_path.expand_2(connections.clone());

    while !possible.is_empty() {
        let path = possible.pop().unwrap();
        for exp_path in path.expand_2(connections.clone()) {
            if exp_path.path[exp_path.path.len() - 1] == "end" {
                res.push(exp_path);
            } else {
                possible.push(exp_path);
            }
        }
    }

    res.len()
}