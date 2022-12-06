use aoc_rust::util::input::read_string;

fn main() {
    let code = read_string(2022, 6);
    println!("Part 1: {}", part1(code.clone()));
    println!("Part 2: {}", part2(code.clone()));
}

fn part1(code: String) -> usize {
    solve(code, 4)
}

fn part2(code: String) -> usize {
    solve(code, 14)
}

fn solve(code: String, size: usize) -> usize {
    'outerloop: for i in size..code.len() {
        let sub = code.chars().collect::<Vec<char>>()[i-size..i].to_vec();

        for j in 0..size {
            if sub.iter().filter(| &&c | sub[j] == c).count() != 1 {
                continue 'outerloop;
            }
        }

        return i;
    }

    0
}