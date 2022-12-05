use aoc_rust::util::input::read_lines;
use aoc_rust::util::string::regex_parse;

struct Assignment {
    from: i32,
    to: i32
}

impl Assignment {
    fn new(from: i32, to: i32) -> Assignment {
        Assignment{ from, to }
    }

    fn contains(&self, other: &Self) -> bool {
        other.from >= self.from
            && other.to <= self.to
    }
}

struct PairAssignment {
    elf_1: Assignment,
    elf_2: Assignment
}

impl PairAssignment {
    fn parse(line: &str) -> PairAssignment {
        let matches = regex_parse::<i32>(line, r"(\d+)-(\d+),(\d+)-(\d+)");

        PairAssignment {
            elf_1: { Assignment::new(matches[0], matches[1])},
            elf_2: { Assignment::new(matches[2], matches[3])}
        }
    }

    fn contains(&self) -> bool {
        self.elf_1.contains(&self.elf_2)
            || self.elf_2.contains(&self.elf_1)
    }

    fn overlaps(&self) -> bool {
        self.elf_1.contains(&Assignment { to: self.elf_2.to, from: self.elf_2.to })
            || self.elf_1.contains(&Assignment { to: self.elf_2.from, from: self.elf_2.from })
            || self.elf_2.contains(&Assignment { to: self.elf_1.to, from: self.elf_1.to })
            || self.elf_2.contains(&Assignment { to: self.elf_1.from, from: self.elf_1.from })
    }
}

fn main() {
    let lines = read_lines(2022, 4);
    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines.clone()));
}

fn part1(lines: Vec<String>) -> usize {
    lines.iter()
        .map(| line | PairAssignment::parse(line))
        .filter(| p | p.contains())
        .count()
}

fn part2(lines: Vec<String>) -> usize {
    lines.iter()
        .map(| line | PairAssignment::parse(line))
        .filter(| p | p.overlaps())
        .count()
}