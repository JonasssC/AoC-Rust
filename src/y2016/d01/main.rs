use aoc_rust::util::input::read_split_on_comma;

fn main() {
    let steps: Vec<(String, i32)> = read_split_on_comma(2016, 1)
        .iter()
        .map(| s | s.trim().split_at(1))
        .map(| (dir, size) | (dir.to_string(), size.parse::<i32>().unwrap()))
        .collect();
    println!("Part 1: {}", part1(steps.clone()));
    println!("Part 2: {}", part2(steps.clone()));
}

fn part1(steps: Vec<(String, i32)>) -> i32 {
    let mut dir = 0;
    let mut pos = (0, 0);

    for step in steps.iter() {
        dir = if step.0 == "R" { (dir + 1) % 4 } else { (dir + 3) % 4 };
        match dir {
            0 => pos.1 += step.1,
            1 => pos.0 += step.1,
            2 => pos.1 -= step.1,
            3 => pos.0 -= step.1,
            _ => panic!("UNKNOWN DIRECTION")
        };
    }

    return pos.0.abs() + pos.1.abs();
}

fn part2(steps: Vec<(String, i32)>) -> i32 {
    let mut dir = 0;
    let mut pos = (0, 0);
    let mut visited: Vec<(i32, i32)> = Vec::new();

    'outerloop: for step in steps.iter() {
        dir = if step.0 == "R" { (dir + 1) % 4 } else { (dir + 3) % 4 };

        for _ in 0..step.1 {
            match dir {
                0 => pos.1 += 1,
                1 => pos.0 += 1,
                2 => pos.1 -= 1,
                3 => pos.0 -= 1,
                _ => panic!("UNKNOWN DIRECTION")
            };
            if visited.contains(&pos) {
                break 'outerloop;
            }
            visited.push(pos);
        }
    }

    return pos.0.abs() + pos.1.abs();
}