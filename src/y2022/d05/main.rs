use aoc_rust::util::input::read_lines_split_on_empty_line;
use aoc_rust::util::string::regex_parse;

#[derive(Clone, Debug)]
struct Crane {
    crates: Vec<char>
}

impl Crane {
    fn parse(lines: Vec<String>) -> Vec<Crane> {
        let mut crates: [Vec<char>; 9] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];

        for i in 0..lines.len()-1 {
            for j in 0..crates.len() {
                let c: char = lines[i].chars().nth(1 + j * 4).unwrap_or(' ');
                if c != ' ' {
                    crates[j].push(c);
                }
            }
        }

        crates.iter().enumerate()
            .map(| c |  Crane {
                crates: c.1.iter().rev().cloned().collect()
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
struct Instruction {
    mv: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn parse(lines: Vec<String>) -> Vec<Instruction> {
        let mut instructions: Vec<Instruction> = Vec::new();
        for line in lines.iter() {
            let nums = regex_parse::<usize>(line, r"move (\d+) from (\d+) to (\d+)");
            instructions.push(Instruction {
                mv: nums[0],
                from: nums[1],
                to: nums[2]
            })
        }

        return instructions;
    }
}

fn main() {
    let lines = read_lines_split_on_empty_line(2022, 5);
    let cranes = Crane::parse(lines[0].clone());
    let instructions = Instruction::parse(lines[1].clone());
    println!("Part 1: {}", part1(cranes.clone(), instructions.clone()));
    println!("Part 2: {}", part2(cranes.clone(), instructions.clone()));
}

fn part1(mut cranes: Vec<Crane>, instructions: Vec<Instruction>) -> String {
    for instruction in instructions.iter() {
        for _ in 0..instruction.mv {
            let c = cranes[instruction.from - 1].crates.pop().unwrap();
            cranes[instruction.to - 1].crates.push(c);
        }
    }

    let mut res = String::from("");

    for crane in cranes.iter() {
        res.push(crane.crates[crane.crates.len() - 1]);
    }

    return res;
}

fn part2(mut cranes: Vec<Crane>, instructions: Vec<Instruction>) -> String {
    for instruction in instructions.iter() {
        let mut moving: Vec<char> = Vec::new();
        for _ in 0..instruction.mv {
            moving.push(cranes[instruction.from - 1].crates.pop().unwrap());
        }

        moving.reverse();

        cranes[instruction.to - 1].crates.append(&mut moving);
    }

    let mut res = String::from("");

    for crane in cranes.iter() {
        res.push(crane.crates[crane.crates.len() - 1]);
    }

    return res;
}