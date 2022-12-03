use aoc_rust::util::input::read_lines;
use aoc_rust::util::vector::group_by_n;
use aoc_rust::common_chars;

fn main() {
    let lines = read_lines(2022, 3);
    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines.clone()));
}

fn char_to_priority(c: char) -> i32 {
    return if c.is_uppercase() {
        c as i32 - 'A' as i32 + 27
    } else {
        c as i32 as i32 - 'a' as i32 as i32 + 1
    }
}

fn part1(lines: Vec<String>) -> i32 {
    let mut res = 0;

    for line in lines.iter() {
        let (first_half, second_half) = line.split_at(line.len() / 2);
        let c = common_chars!(first_half, second_half).chars().nth(0).expect("No common char found");
        res += char_to_priority(c);
    }

    return res;
}

fn part2(lines: Vec<String>) -> i32 {
    let mut res = 0;

    for group in group_by_n(lines, 3) {
        let c = common_chars!(&group[0], &group[1], &group[2]).chars().nth(0).expect("No common char found");
        res += char_to_priority(c);
    }

    return res;
}