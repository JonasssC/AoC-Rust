use aoc_rust::util::input::read_lines;
use std::collections::HashMap;
use regex::Regex;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn step(&self, step_size: [i32; 2]) -> Point {
        Point {
            x: self.x + step_size[0],
            y: self.y + step_size[1]
        }
    }
}

#[derive(Clone, Debug)]
struct Vent {
    from: Point,
    to: Point
}

impl Vent {
    fn parse(str: String) -> Vent {
        let re = Regex::new(r"(?P<from_x>\d+),(?P<from_y>\d+) -> (?P<to_x>\d+),(?P<to_y>\d+)").expect("Invalid RegEx");
        let caps = re.captures(str.as_str()).unwrap();
        if caps.len() != 5 {
            panic!("Incorrect format");
        }

        let get_cap = | key | {
            caps.name(key).unwrap().as_str().parse::<i32>().unwrap()
        };

        Vent {
            from: Point {
                x: get_cap("from_x"),
                y: get_cap("from_y")
            },
            to: Point {
                x: get_cap("to_x"),
                y: get_cap("to_y")
            }
        }
    }

    fn step_size(&self) -> [i32; 2] {
        return [
            if self.from.x > self.to.x { -1 }
            else if self.from.x < self.to.x { 1 }
            else { 0 },
            if self.from.y > self.to.y { -1 }
            else if self.from.y < self.to.y { 1 }
            else { 0 }
        ];
    }

    fn mark(&self, field: &mut HashMap<Point, i32>) {
        let step_size = self.step_size();
        let mut p = self.from.clone();
        loop {
            let count = field.entry(p.clone()).or_insert(0);
            *count += 1;
            if p == self.to {
                break;
            }
            p = p.step(step_size)
        }
    }
}

fn main() {
    let lines = read_lines(2021, 5);
    let vents: Vec<Vent> = lines.iter()
        .map(| s | Vent::parse(s.clone()))
        .collect();
    println!("Part 1: {}", part1(vents.clone()));
    println!("Part 2: {}", part2(vents.clone()));
}

fn part1(vents: Vec<Vent>) -> i32 {
    let mut field = HashMap::new();
    for vent in vents {
        if vent.from.x != vent.to.x && vent.from.y != vent.to.y {
            continue;
        }
        vent.mark(&mut field);
    }

    let mut count = 0;
    for (_, mark_count) in field.iter() {
        if *mark_count > 1 {
            count += 1;
        }
    }

    return count;
}

fn part2(vents: Vec<Vent>) -> i32 {
    let mut field = HashMap::new();
    for vent in vents {
        vent.mark(&mut field);
    }

    let mut count = 0;
    for (_, mark_count) in field.iter() {
        if *mark_count > 1 {
            count += 1;
        }
    }

    return count;
}
