extern crate core;

use aoc_rust::read_lines;

fn main() {
    let lines = read_lines(2022, 2);
    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines.clone()));
}

fn get_plays(line: String) -> (char, char) {
    return match line.chars().collect::<Vec<char>>()[..] {
        [x, _, y] => (x, y),
        _ => panic!("Invalid line")
    };
}

fn part1(lines: Vec<String>) -> i32 {
    let mut score = 0;

    for line in lines {
        let (opp_c, you_c) = get_plays(line);
        let opp = ['A', 'B', 'C'].iter().position(| &c | c == opp_c ).unwrap() as i32 + 1;
        let you = ['X', 'Y', 'Z'].iter().position(| &c | c == you_c ).unwrap() as i32 + 1;

        let res = (you - opp + 3) % 3;

        if res == 1 {
            score += 6;
        } else if res == 0 {
            score += 3;
        }

        score += you;
    }

    return score;
}

fn part2(lines: Vec<String>) -> i32 {
    let mut score = 0;

    for line in lines {
        let (opp_c, res_c) = get_plays(line);
        let opp = ['A', 'B', 'C'].iter().position(| &c | c == opp_c ).unwrap() as i32 + 1;
        let res = ['Y', 'Z', 'X'].iter().position(| &c | c == res_c ).unwrap() as i32;

        let you = (res + opp - 1) % 3 + 1;
        if res == 1 {
            score += 6;
        } else if res == 0 {
            score += 3;
        }

        score += you;
    }

    return score;
}