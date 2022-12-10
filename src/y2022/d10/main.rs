use aoc_rust::util::input::read_lines;

fn main() {
    let lines = read_lines(2022, 10);
    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: \n{}", part2(lines.clone()));
}

fn part1(lines: Vec<String>) -> i32 {
    let mut x = 1;
    let mut cycles = 0;
    let mut signal_strengths: Vec<i32> = Vec::new();

    for line in lines.iter() {
        cycles += 1;

        if cycles % 40 == 20 {
            signal_strengths.push(x * cycles);
        }

        match line.split(" ").collect::<Vec<&str>>()[..] {
            ["noop"] => (),
            ["addx", v] => {
                cycles += 1;
                if cycles % 40 == 20 {
                    signal_strengths.push(x * cycles);
                }
                x += v.parse::<i32>().unwrap();
            }
            _ => panic!("unknown ")
        }
    }

    signal_strengths.iter().sum()
}

fn part2(lines: Vec<String>) -> String {
    let mut crt = String::new();
    let mut sprite = "###.....................................".chars().collect::<Vec<char>>();

    for line in lines.iter() {
        crt.push(sprite[crt.len() % 40]);

        match line.split(" ").collect::<Vec<&str>>()[..] {
            ["noop"] => (),
            ["addx", v] => {
                crt.push(sprite[crt.len() % 40]);
                let val = v.parse::<i32>().unwrap();
                if val >= 0 {
                    sprite.rotate_right(val as usize);
                } else {
                    sprite.rotate_left(val.abs() as usize);
                }
            }
            _ => panic!("unknown ")
        }
    }

    crt.chars()
        .collect::<Vec<char>>()
        .chunks(40)
        .map(| c | c.iter()
            .collect::<String>()
        )
        .collect::<Vec<String>>()
        .join("\n")
}
