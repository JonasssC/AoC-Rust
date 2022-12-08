use aoc_rust::util::input::read_lines;

fn main() {
    let lines: Vec<Vec<u32>> = read_lines(2022, 8)
        .iter()
        .map(| l | l.chars().map(| c | c as u32 - '0' as u32)
            .collect::<Vec<u32>>()
        )
        .collect();
    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines.clone()));
}

fn is_in_field(field: Vec<Vec<u32>>, row: usize, col: usize, dir: &(i32, i32), n: usize) -> bool {
    row as i32 + dir.0 * n as i32 >= 0 && (row as i32 + dir.0 * n as i32) < field.len() as i32
        && col as i32 + dir.1 * n as i32 >= 0 && (col as i32 + dir.1 * n as i32) < field[row].len() as i32
}

fn part1(field: Vec<Vec<u32>>) -> usize {
    let mut visible_count = 0;

    for row in 0..field.len() {
        'colloop: for col in 0..field[row].len() {
            'dirloop: for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
                let mut i = 1;
                while is_in_field(field.clone(), row, col, dir, i) {
                    if field[row][col] <= field[(row as i32 + dir.0 * i as i32) as usize][(col as i32 + dir.1 * i as i32) as usize] {
                        continue 'dirloop;
                    }
                    i += 1;
                }
                visible_count += 1;
                continue 'colloop;
            }
        }
    }

    visible_count
}

fn part2(field: Vec<Vec<u32>>) -> usize {
    let mut max = 0;

    for row in 0..field.len() {
        for col in 0..field[row].len() {
            let mut res = 1;
            'dirloop: for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
                let mut i = 1;
                while is_in_field(field.clone(), row, col, dir, i) {
                    if field[row][col] <= field[(row as i32 + dir.0 * i as i32) as usize][(col as i32 + dir.1 * i as i32) as usize] {
                        res *= i;
                        continue 'dirloop;
                    }
                    i += 1;
                }
                res *= i - 1;
            }

            if res > max {
                max = res;
            }
        }
    }

    max
}