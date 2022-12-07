use aoc_rust::util::input::read_lines;

static KEYPAD_1: [[i32; 3]; 3] = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
];

static KEYPAD_2: [[char; 7]; 7] = [
    [' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', '1', ' ', ' ', ' '],
    [' ', ' ', '2', '3', '4', ' ', ' '],
    [' ', '5', '6', '7', '8', '9', ' '],
    [' ', ' ', 'A', 'B', 'C', ' ', ' '],
    [' ', ' ', ' ', 'D', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' '],
];

fn main() {
    let instructions: Vec<String> = read_lines(2016, 2);
    println!("Part 1: {}", part1(instructions.clone()));
    println!("Part 2: {}", part2(instructions.clone()));
}

fn part1(instructions: Vec<String>) -> i32 {
    let mut coord = (1, 1);
    let mut res: i32 = 0;

    for instruction in instructions.iter() {
        for step in instruction.chars().collect::<Vec<char>>() {
            println!("{step}");
            match step {
                'U' => if coord.0 > 0 { coord.0 = coord.0 - 1 },
                'D' => if coord.0 < 2 { coord.0 = coord.0 + 1 },
                'L' => if coord.1 > 0 { coord.1 = coord.1 - 1 },
                'R' => if coord.1 < 2 { coord.1 = coord.1 + 1 },
                 _  => panic!("Invalid move")
            }
        }

        res *= 10;
        res += KEYPAD_1[coord.0][coord.1];
    }

    res
}

fn part2(instructions: Vec<String>) -> String {
    let mut coord = (3, 1);
    let mut res: String = String::from("");

    for instruction in instructions.iter() {
        for step in instruction.chars().collect::<Vec<char>>() {
            match step {
                'U' => if KEYPAD_2[coord.0-1][coord.1] != ' ' { coord.0 = coord.0 - 1 },
                'D' => if KEYPAD_2[coord.0+1][coord.1] != ' ' { coord.0 = coord.0 + 1 },
                'L' => if KEYPAD_2[coord.0][coord.1-1] != ' ' { coord.1 = coord.1 - 1 },
                'R' => if KEYPAD_2[coord.0][coord.1+1] != ' ' { coord.1 = coord.1 + 1 },
                 _  => panic!("Invalid move")
            }
        }

        res.push(KEYPAD_2[coord.0][coord.1]);
    }

    res
}