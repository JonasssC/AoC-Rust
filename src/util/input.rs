use std::fmt::Debug;
use std::fs::read_to_string;
use std::str::FromStr;

pub fn read_string(year: i32, day: i32) -> String {
    let path = format!("src/y{}/d{:0>2}/input.txt", year, day);
    return read_to_string(path)
        .expect("Couldn't read file")
        .to_string();
}

pub fn read_lines(year: i32, day: i32) -> Vec<String> {
    return read_string(year, day)
        .lines()
        .map(| l | l.to_string())
        .collect();
}

pub fn read_lines_as_i32(year: i32, day: i32) -> Vec<i32> {
    return read_lines(year, day)
        .into_iter()
        .map(| l | l.parse::<i32>().unwrap())
        .collect()
}

pub fn read_split_on_empty_line(year: i32, day: i32) -> Vec<String> {
    return read_string(year, day)
        .split("\n\n")
        .map(| l | l.to_string())
        .collect();
}

pub fn read_lines_split_on_empty_line(year: i32, day: i32) -> Vec<Vec<String>> {
    return read_split_on_empty_line(year, day)
        .iter()
        .map(| g | g.lines()
            .map(|  l | l.to_string())
            .collect()
        )
        .collect();
}

pub fn read_lines_split_on_empty_line_as_i32(year: i32, day: i32) -> Vec<Vec<i32>> {
    return read_lines_split_on_empty_line(year, day)
        .iter()
        .map(| g | g.iter()
            .map(| l | l.parse::<i32>().unwrap())
            .collect()
        )
        .collect();
}

pub fn read_split_on_comma(year: i32, day: i32) -> Vec<String> {
    return read_string(year, day)
        .split(",")
        .map(| l | l.to_string())
        .collect();
}

pub fn read_split_on_comma_as_i32(year: i32, day: i32) -> Vec<i32> {
    return read_split_on_comma(year, day)
        .into_iter()
        .map(| l | l.parse::<i32>().unwrap())
        .collect();
}

pub fn read_as_matrix<T: FromStr>(year: i32, day: i32, chunk_size: usize) -> Vec<Vec<T>> where <T as FromStr>::Err: Debug {
    return read_lines(year, day)
        .iter()
        .map(| l | l.chars()
            .collect::<Vec<char>>()
            .chunks(chunk_size)
            .map(| c | c.iter().collect::<String>())
            .map(| s | s.parse::<T>().unwrap())
            .collect::<Vec<T>>()
        )
        .collect();
}