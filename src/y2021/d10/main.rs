use aoc_rust::util::input::read_lines;

fn main() {
    let lines = read_lines(2021, 10);
    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines.clone()));
}

static VALID_PAIRS: [(char, char, i32, i64); 4] = [
    ('(', ')', 3, 1),
    ('[', ']', 57, 2),
    ('{', '}', 1197, 3),
    ('<', '>', 25137, 4)
];

fn part1(lines: Vec<String>) -> i32 {
    let mut res = 0;

    for line in lines.iter() {
        let mut opening: Vec<char> = Vec::new();
        'charloop: for c in line.chars() {
            if "([{<".contains(c) {
                opening.push(c);
            } else {
                for pair in VALID_PAIRS.iter() {
                    if c == pair.1 && opening.pop().unwrap_or('.') != pair.0 {
                        res += pair.2;
                        break 'charloop;
                    }
                }
            }
        }
    }

    return res;
}

fn part2(lines: Vec<String>) -> i64 {
    let mut scores: Vec<i64> = Vec::new();

    'lineloop: for line in lines.iter() {
        let mut opening: Vec<char> = Vec::new();
        for c in line.chars() {
            if "([{<".contains(c) {
                opening.push(c);
            } else {
                for pair in VALID_PAIRS.iter() {
                    if c == pair.1 && opening.pop().unwrap_or('.') != pair.0 {
                        continue 'lineloop;
                    }
                }
            }
        }

        let mut line_score: i64 = 0;
        for left_open in opening.iter().rev() {
            for pair in VALID_PAIRS.iter() {
                if *left_open == pair.0 {
                    line_score*= 5;
                    line_score += pair.3;
                    break;
                }
            }
        }
        scores.push(line_score);
    }

    scores.sort();
    return scores[scores.len() / 2];
}