use aoc_rust::util::input::read_lines;

fn main() {
    let lines = read_lines(2021, 2);
    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines.clone()));
}

fn part1(lines: Vec<String>) -> i32 {
    let mut depth = 0;
    let mut hor = 0;

    for line in lines {
        match line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["forward", step] => hor += step.parse::<i32>().unwrap(),
            ["down", step] => depth += step.parse::<i32>().unwrap(),
            ["up", step]  => depth -= step.parse::<i32>().unwrap(),
            [..] => println!("Unknown instruction '{}'", line),
        }
    }

    return depth * hor;
}

fn part2(lines: Vec<String>) -> i32 {
    let mut depth = 0;
    let mut hor = 0;
    let mut aim = 0;

    for line in lines {
        match line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["forward", step] => {
                hor += step.parse::<i32>().unwrap();
                depth += aim * step.parse::<i32>().unwrap();
            },
            ["down", step] => aim += step.parse::<i32>().unwrap(),
            ["up", step]  => aim -= step.parse::<i32>().unwrap(),
            [..] => println!("Unknown instruction '{}'", line),
        }
    }

    return depth * hor;
}
