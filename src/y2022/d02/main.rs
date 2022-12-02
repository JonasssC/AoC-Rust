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
        let opp = match opp_c {
            'A' => 1,
            'B' => 2,
            'C' => 3,
             _  => panic!("Invalid opponent")
        };
        let you = match you_c {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _  => panic!("Invalid opponent")
        };

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
        let opp = match opp_c {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _  => panic!("Invalid opponent")
        };
        let res = match res_c {
            'X' => 2,
            'Y' => 0,
            'Z' => 1,
            _  => panic!("Invalid opponent")
        };

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