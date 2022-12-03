use aoc_rust::util::input::read_lines;

fn main() {
    let lines = read_lines(2021, 3);
    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines.clone()));
}

fn to_dec(bin: Vec<bool>) -> i32 {
    return bin
        .iter()
        .rev()
        .enumerate()
        .fold(0, | res: i32, (i, b) | res + if *b { 2_i32.pow(i as u32) } else { 0 });
}

fn count_at_index(lines: Vec<String>, i: usize, ch: char) -> usize {
    return lines.iter()
        .filter(| l | l.chars().nth(i).unwrap() == ch)
        .count();
}

fn part1(lines: Vec<String>) -> i32 {
    let counts: Vec<usize> = (0..lines[0].len()).collect::<Vec<usize>>().iter()
        .map(| i | count_at_index(lines.clone(), *i, '1'))
        .collect();

    let gamma: Vec<bool> = counts.iter()
        .map(| c | c > &(lines.len() / 2))
        .collect();

    let epsilon: Vec<bool> = gamma.iter()
        .map(| b | !b)
        .collect();

    return to_dec(gamma) * to_dec(epsilon);
}

/* ------------------ */

fn part2(lines: Vec<String>) -> i32 {
    return calc_rating(lines.clone(), true) * calc_rating(lines.clone(), false);
}

fn calc_rating(mut lines: Vec<String>, most_common: bool) -> i32 {
    let mut i = 0;
    while lines.len() > 1 && i < lines[0].len() {
        let count = count_at_index(lines.clone(), i, '1');

        let ch = if most_common {
            if count as f32 >= lines.len() as f32/2.0 { '1' } else { '0' }
        } else {
            if count as f32 >= lines.len() as f32/2.0 { '0' } else { '1' }
        };

        lines = lines.iter()
            .filter(| l | l.chars().nth(i).unwrap() == ch)
            .cloned()
            .collect();

        i += 1;
    }

    let result: Vec<bool> = lines[0].chars()
        .map(| ch | ch == '1')
        .collect();

    return to_dec(result);
}