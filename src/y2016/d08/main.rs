use aoc_rust::util::input::read_lines;

fn main() {
    let lines = read_lines(2016, 8);
    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: \n{}", part2(lines.clone()));
}

fn draw_rect(screen: &mut [[bool; 50]; 6], rect: &str) {
    let size = rect.split_once("x").unwrap();
    for col in 0..size.0.parse::<usize>().unwrap() {
        for row in 0..size.1.parse::<usize>().unwrap() {
            screen[row][col] = true;
        }
    }
}

fn rotate_down(screen: &mut [[bool; 50]; 6], col: usize, n: usize) {
    for _ in 0..n {
        let temp = screen[screen.len()-1][col];
        for i in (1..screen.len()).rev() {
            screen[i][col] = screen[i-1][col];
        }
        screen[0][col] = temp;
    }
}

fn count(screen: [[bool; 50]; 6]) -> usize {
    screen.map(| row | row.iter()
            .filter(| &&b | b)
            .count()
    ).iter().sum()
}

fn part1(lines: Vec<String>) -> usize {
    let mut screen = [[false; 50]; 6];

    for line in lines.iter() {
        let split_line = line.split_whitespace().collect::<Vec<&str>>();

        match split_line[..] {
            ["rect", rect] => draw_rect(&mut screen, rect),
            ["rotate", "row", row_s, "by", n_s] => {
                let row = row_s.split_once("=").unwrap().1.parse::<usize>().unwrap();
                let n = n_s.parse::<usize>().unwrap();
                screen[row].rotate_right(n);
            },
            ["rotate", "column", col_s, "by", n_s] => {
                let col = col_s.split_once("=").unwrap().1.parse::<usize>().unwrap();
                let n = n_s.parse::<usize>().unwrap();
                rotate_down(&mut screen, col, n);
            },
            _ => panic!("Unknown instruction")
        }

    }

    count(screen.clone())
}

fn part2(lines: Vec<String>) -> String {
    let mut screen = [[false; 50]; 6];

    for line in lines.iter() {
        let split_line = line.split_whitespace().collect::<Vec<&str>>();

        match split_line[..] {
            ["rect", rect] => draw_rect(&mut screen, rect),
            ["rotate", "row", row_s, "by", n_s] => {
                let row = row_s.split_once("=").unwrap().1.parse::<usize>().unwrap();
                let n = n_s.parse::<usize>().unwrap();
                screen[row].rotate_right(n);
            },
            ["rotate", "column", col_s, "by", n_s] => {
                let col = col_s.split_once("=").unwrap().1.parse::<usize>().unwrap();
                let n = n_s.parse::<usize>().unwrap();
                rotate_down(&mut screen, col, n);
            },
            _ => panic!("Unknown instruction")
        }

    }

    screen.map(| row | row.iter().map(| &b | if b { "#" } else { " " }).collect::<Vec<&str>>().join("")).join("\n")
}