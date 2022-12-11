use aoc_rust::util::input::read_lines_split_on_empty_line;
use aoc_rust::util::math::max_n;
use aoc_rust::util::string::regex_parse;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: String,
    test_divisible_by: i64,
    test_true_to: usize,
    test_false_to: usize
}

impl Monkey {
    fn parse(line_groups: Vec<Vec<String>>) -> Vec<Monkey> {
        let mut res: Vec<Monkey> = Vec::new();

        for lines in line_groups.iter() {
            res.push(
                Monkey {
                    items: lines[1].split_once(": ")
                        .unwrap().1
                        .split(", ")
                        .map(| n | n.parse::<i64>().unwrap())
                        .collect(),
                    operation: lines[2].clone(),
                    test_divisible_by: regex_parse::<i64>(&lines[3], r"Test: divisible by (\d+)")[0].clone(),
                    test_true_to: regex_parse::<usize>(&lines[4], r"If true: throw to monkey (\d+)")[0],
                    test_false_to: regex_parse::<usize>(&lines[5], r"If false: throw to monkey (\d+)")[0],
                }
            );
        }
        res
    }
}

fn perform_operation(operation: String, old: i64) -> i64 {
    let groups = regex_parse::<String>(&operation, r"  Operation: new = (.+) (\+|\*) (.+)");
    let n1 = if &groups[0] == "old" { old } else { groups[0].parse::<i64>().unwrap() };
    let n2 = if &groups[2] == "old" { old } else { groups[2].parse::<i64>().unwrap() };
    if &groups[1] == "+" {
        n1 + n2
    } else {
        n1 * n2
    }
}

fn main() {
    let line_groups = read_lines_split_on_empty_line(2022, 11);
    let monkeys = Monkey::parse(line_groups.clone());
    println!("{} - {}", line_groups.len(), monkeys.len());
    println!("Part 1: {}", part1(monkeys.clone()));
    println!("Part 2: {}", part2(monkeys.clone()));
}

fn part1(mut monkeys: Vec<Monkey>) -> usize {
    let mut inspect_count = [0; 8].to_vec();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            inspect_count[i] += monkeys[i].items.len();
            for j in 0..monkeys[i].items.len() {
                let mut a = perform_operation(monkeys[i].operation.clone(), monkeys[i].items[j].clone());
                a /= 3;
                let to = if a % monkeys[i].clone().test_divisible_by == 0 {
                    monkeys[i].test_true_to
                } else {
                    monkeys[i].test_false_to
                };
                monkeys[to].items.push(a);
            }
            monkeys[i].items.clear();
        }
    }

    let max_2 = max_n(inspect_count, 2);
    max_2[0] * max_2[1]
}

fn least_common_multiple(monkeys: Vec<Monkey>) -> i64 {
    let mut res = monkeys[0].test_divisible_by;
    for i in 1..monkeys.len() {
        res = lcm(res, monkeys[i].test_divisible_by)
    }
    res
}

fn lcm(first: i64, second: i64) -> i64 {
    first * second / gcd(first, second)
}

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

// FUCKING INEFFICIENT LANG
fn part2(mut monkeys: Vec<Monkey>) -> usize {
    let mut inspect_count = [0; 8].to_vec();

    let multiple = least_common_multiple(monkeys.clone());

    for round in 0..10000 {
        for i in 0..monkeys.len() {
            inspect_count[i] += monkeys[i].items.len();
            for j in 0..monkeys[i].items.len() {
                let mut a = perform_operation(monkeys[i].operation.clone(), monkeys[i].items[j].clone());
                a %= multiple;
                let to = if a % monkeys[i].clone().test_divisible_by == 0 {
                    monkeys[i].test_true_to
                } else {
                    monkeys[i].test_false_to
                };
                monkeys[to].items.push(a);
            }
            monkeys[i].items.clear();
        }
        println!("{round}");
    }

    println!("{inspect_count:?}");

    let max_2 = max_n(inspect_count, 2);
    max_2[0] * max_2[1]
}