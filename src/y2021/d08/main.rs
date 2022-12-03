use aoc_rust::util::input::read_lines;

static NUMS: [&'static [char]; 10] = [
    &['a', 'b', 'c', 'e', 'f', 'g'],
    &['c', 'f'],
    &['a', 'c', 'd', 'e', 'g'],
    &['a', 'c', 'd', 'f', 'g'],
    &['b', 'c', 'd', 'f'],
    &['a', 'b', 'd', 'f', 'g'],
    &['a', 'b', 'd', 'e', 'f', 'g'],
    &['a', 'c', 'f'],
    &['a', 'b', 'c', 'd', 'e', 'f', 'g'],
    &['a', 'b', 'c', 'd', 'f', 'g']
];


fn main() {
    let lines = read_lines(2021, 8);
    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines.clone()));
}

fn contains_num(num: &str, contains: &str) -> bool {
    for c in contains.chars().collect::<Vec<char>>().iter() {
        if !num.contains(*c) {
            return false;
        }
    }

    return true;
}

fn num_equals(num1: &str, num2: &str) -> bool {
    if num1.len() != num2.len() {
        return false;
    }

    return contains_num(num1, num2);
}

fn part1(lines: Vec<String>) -> i32 {
    let mut count = 0;

    for line in lines {
        let output_values: Vec<&str>  = line.split("|").collect::<Vec<&str>>()[1].split_whitespace().collect::<Vec<&str>>();
        for val in output_values {
            if val.len() == NUMS[1].len()
                || val.len() == NUMS[4].len()
                || val.len() == NUMS[7].len()
                || val.len() == NUMS[8].len() {
                count += 1;
            }
        }
    }

    return count;
}

fn part2(lines: Vec<String>) -> i32 {
    let mut count = 0;

    for line in lines {
        let input_values: Vec<&str> = line.split("|").collect::<Vec<&str>>()[0].split_whitespace().collect::<Vec<&str>>();
        let output_values: Vec<&str>  = line.split("|").collect::<Vec<&str>>()[1].split_whitespace().collect::<Vec<&str>>();

        let mut displays: [&str; 10] = [""; 10];

        displays[1] = input_values.iter().find(| s | s.len() == NUMS[1].len()).unwrap();
        displays[4] = input_values.iter().find(| s | s.len() == NUMS[4].len()).unwrap();
        displays[7] = input_values.iter().find(| s | s.len() == NUMS[7].len()).unwrap();
        displays[8] = input_values.iter().find(| s | s.len() == NUMS[8].len()).unwrap();

        displays[3] = input_values.iter().find(| s | s.len() == NUMS[3].len() && contains_num(s, displays[1])).unwrap();
        displays[6] = input_values.iter().find(| s | s.len() == NUMS[6].len() && !contains_num(s, displays[1])).unwrap();
        displays[9] = input_values.iter().find(| s | s.len() == NUMS[9].len() && contains_num(s, displays[4])).unwrap();
        displays[0] = input_values.iter().find(| s | s.len() == NUMS[0].len() && **s != displays[6] && **s != displays[9]).unwrap();
        displays[5] = input_values.iter().find(| s | s.len() == NUMS[5].len() && contains_num(displays[6], s)).unwrap();
        displays[2] = input_values.iter().find(| s | s.len() == NUMS[2].len() && **s != displays[5] && **s != displays[3]).unwrap();

        let mut num_str = String::from("");
        for value in output_values.iter() {
            for j in 0..10 {
                if num_equals(value, displays[j]) {
                    num_str = format!("{num_str}{j}");
                    break;
                }
            }
        }
        count += num_str.parse::<i32>().unwrap();
    }

    return count;
}