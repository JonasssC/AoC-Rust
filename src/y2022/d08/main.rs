use aoc_rust::util::input::read_as_matrix;
use aoc_rust::util::matrix::is_in_field;

fn main() {
    let field = read_as_matrix::<u32>(2022, 8, 1);
    println!("Part 1: {}", part1(field.clone()));
    println!("Part 2: {}", part2(field.clone()));
}

fn part1(field: Vec<Vec<u32>>) -> usize {
    let mut visible_count = 0;

    for row in 0..field.len() {
        'colloop: for col in 0..field[row].len() {
            'dirloop: for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
                let mut i = 1;
                let mut pos = (row as i32 + dir.0, col as i32 + dir.1);
                while is_in_field(field.len(), field[0].len(), pos.0, pos.1) {
                    if field[row][col] <= field[pos.0 as usize][pos.1 as usize] {
                        continue 'dirloop;
                    }
                    i += 1;
                    pos = (row as i32 + dir.0 * i, col as i32 + dir.1 * i);
                }
                visible_count += 1;
                continue 'colloop;
            }
        }
    }

    visible_count
}

fn part2(field: Vec<Vec<u32>>) -> i32 {
    let mut max = 0;

    for row in 0..field.len() {
        for col in 0..field[row].len() {
            let mut res = 1;
            'dirloop: for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
                let mut i = 1;
                let mut pos = (row as i32 + dir.0, col as i32 + dir.1);
                while is_in_field(field.len(), field[0].len(), pos.0, pos.1) {
                    if field[row][col] <= field[pos.0 as usize][pos.1 as usize] {
                        res *= i;
                        continue 'dirloop;
                    }
                    i += 1;
                    pos = (row as i32 + dir.0 * i, col as i32 + dir.1 * i);
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