use aoc_rust::util::input::read_lines;
use aoc_rust::util::string::regex_parse;

fn main() {
    let triangles: Vec<Vec<i32>> = read_lines(2016, 3).iter()
        .map(| l | regex_parse::<i32>(l , r"(\d+) +(\d+) +(\d+)"))
        .collect();
    println!("Part 1: {}", part1(triangles.clone()));
    println!("Part 2: {}", part2(triangles.clone()));
}

fn part1(triangles: Vec<Vec<i32>>) -> i32 {
    let mut count: i32 = 0;

    'outerloop: for triangle in triangles.iter() {
        for i in 0..3 {
            if triangle[i] + triangle[(i+1)%3] <= triangle[(i+2)%3] {
                continue 'outerloop;
            }
        }
        count += 1;
    }

    count
}

fn part2(triangles: Vec<Vec<i32>>) -> i32 {
    let mut count: i32 = 0;

    for triangle in triangles.chunks(3) {
        'i_loop: for i in 0..3 {
            for j in 0..3 {
                if triangle[j][i] + triangle[(j+1)%3][i] <= triangle[(j+2)%3][i] {
                    continue 'i_loop;
                }
            }
            count += 1;
        }
    }

    count
}