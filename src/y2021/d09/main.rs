use std::collections::HashSet;
use aoc_rust::util::input::read_lines;
use aoc_rust::util::math::max_n;

fn main() {
    let matrix: Vec<Vec<i32>> = read_lines(2021, 9)
        .iter()
        .map(| l | l.chars()
            .map(| c | c.to_string().parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        )
        .collect();
    println!("Part 1: {}", part1(matrix.clone()));
    println!("Part 2: {}", part2(matrix.clone()));
}

fn is_in_field(matrix: Vec<Vec<i32>>, row: i32, col: i32) -> bool {
    row >= 0 && row < matrix.len() as i32
        && col >= 0 && col < matrix[0].len() as i32
}

fn is_lower_then_neighbours(matrix: Vec<Vec<i32>>, row: usize, col: usize) -> bool {
    (!is_in_field(matrix.clone(), row as i32, col as i32-1) || matrix[row][col] < matrix[row][col-1])
        && (!is_in_field(matrix.clone(), row as i32, col as i32+1) || matrix[row][col] < matrix[row][col+1])
        && (!is_in_field(matrix.clone(), row as i32-1, col as i32) || matrix[row][col] < matrix[row-1][col])
        && (!is_in_field(matrix.clone(), row as i32+1, col as i32) || matrix[row][col] < matrix[row+1][col])
}

fn find_low_points(matrix: Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if is_lower_then_neighbours(matrix.clone(), i, j) {
                res.push((i, j));
            }
        }
    }

    return res;
}

fn part1(matrix: Vec<Vec<i32>>) -> i32 {
    find_low_points(matrix.clone()).iter()
        .map(| c | matrix[c.0][c.1] + 1)
        .sum()
}

fn build_basin(matrix: Vec<Vec<i32>>, point: (i32, i32)) -> Vec<String> {
    let mut res: HashSet<String> = HashSet::new();
    affect_adjacent(matrix, &mut res, point);
    return res.iter()
        .map(| s | s.clone())
        .collect();
}

fn is_within_map(matrix: Vec<Vec<i32>>, point: (i32, i32)) -> bool {
    point.0 >= 0 && point.0 < matrix.len() as i32
        && point.1 >= 1 && point.1 < matrix[0].len() as i32
}

fn affect_adjacent(matrix: Vec<Vec<i32>>, basin: &mut HashSet<String>, point: (i32, i32)) {
    let point_key = format!("{}-{}", point.0, point.1);
    if is_within_map(matrix.clone(), point.clone()) && !basin.contains(&point_key.clone()) && matrix[point.0 as usize][point.1 as usize] != 9 {
        basin.insert(point_key.clone());

        affect_adjacent(matrix.clone(), basin, (point.0, point.1 + 1));
        affect_adjacent(matrix.clone(), basin, (point.0, point.1 - 1));
        affect_adjacent(matrix.clone(), basin, (point.0 + 1, point.1));
        affect_adjacent(matrix.clone(), basin, (point.0 - 1, point.1));
    }
}

fn part2(matrix: Vec<Vec<i32>>) -> usize {
    let basin_sizes: Vec<usize> = find_low_points(matrix.clone()).iter()
        .map(| c | (c.0 as i32, c.1 as i32))
        .map(| c | build_basin(matrix.clone(), c))
        .collect::<HashSet<Vec<String>>>()
        .iter()
        .map(| b | b.len())
        .collect();

    return max_n(basin_sizes, 3).iter()
        .copied()
        .reduce(| a, b | a * b)
        .unwrap();
}