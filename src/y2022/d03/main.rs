use aoc_rust::util::input::read_lines;
use aoc_rust::util::vector::group_by_n;

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

fn common_char_2(s1: &str, s2: &str) -> char {
    for c1 in s1.chars() {
        for c2 in s2.chars() {
            if c1 == c2 {
                return c1;
            }
        }
    }

    panic!("No common character");
}

fn common_char_3(s1: &str, s2: &str, s3: &str) -> char {
    for c1 in s1.chars() {
        for c2 in s2.chars() {
            for c3 in s3.chars() {
                if c1 == c2 && c2 == c3 {
                    return c1;
                }
            }
        }
    }

    panic!("No common character");
}

fn part1(lines: Vec<String>) -> i32 {
    let mut res = 0;

    for line in lines.iter() {
        let (first_half, second_half) = line.split_at(line.len() / 2);
        let c = common_char_2(first_half, second_half);
        res += char_to_priority(c);
    }

    return res;
}

fn part2(lines: Vec<String>) -> i32 {
    let mut res = 0;

    println!("{:?}", group_by_n(lines.clone(), 3));

    for group in group_by_n(lines, 3) {
        let c = common_char_3(&group[0], &group[1], &group[2]);
        res += char_to_priority(c);
    }

    return res;
}