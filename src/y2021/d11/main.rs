use aoc_rust::util::input::read_as_matrix;

fn main() {
    let octos = read_as_matrix::<i32>(2021, 11, 1);
    println!("Part 1: {}", part1(octos.clone()));
    println!("Part 2: {}", part2(octos.clone()));
}

fn get_coords_vec(octos: Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut coords: Vec<(usize, usize)> = Vec::new();

    for i in 0..octos.len() {
        for j in 0..octos[i].len() {
            coords.push((i, j));
        }
    }

    return coords;
}

fn is_in_field(octos: &mut Vec<Vec<i32>>, row: i32, col: i32) -> bool {
    row >= 0 && row < octos.len() as i32
        && col >= 0 && col < octos[0].len() as i32
}

fn flash_octo(octos: &mut Vec<Vec<i32>>, row: usize, col: usize, flash_count: &mut i32) {
    if octos[row][col] > 9 {
        octos[row][col] = -100;
        *flash_count += 1;

        for step in [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)] {
            if is_in_field(octos, row as i32 + step.0, col as i32 + step.1) {
                octos[(row as i32 + step.0) as usize][(col as i32 + step.1) as usize] += 1;
                flash_octo(octos, (row as i32 + step.0) as usize, (col as i32 + step.1) as usize, flash_count);
            }
        }
    }
}

fn part1(mut octos: Vec<Vec<i32>>) -> i32 {
    let mut flash_count = 0;
    let coords = get_coords_vec(octos.clone());

    for _ in 0..100 {
        for coord in coords.iter() {
            octos[coord.0][coord.1] += 1;
        }
        for coord in coords.iter() {
            flash_octo(&mut octos, coord.0, coord.1, &mut flash_count);
        }
        for coord in coords.iter() {
            if octos[coord.0][coord.1] < 0 {
                octos[coord.0][coord.1] = 0;
            }
        }
    }

    return flash_count;
}

fn part2(mut octos: Vec<Vec<i32>>) -> i32 {
    let mut i = 0;
    let coords = get_coords_vec(octos.clone());

    loop {
        i += 1;
        for coord in coords.iter() {
            octos[coord.0][coord.1] += 1;
        }
        for coord in coords.iter() {
            flash_octo(&mut octos, coord.0, coord.1, &mut 0);
        }
        for coord in coords.iter() {
            if octos[coord.0][coord.1] < 0 {
                octos[coord.0][coord.1] = 0;
            }
        }
        if coords.iter().map(| c | octos[c.0][c.1]).max().unwrap() == 0 {
            return i;
        }
    }
}