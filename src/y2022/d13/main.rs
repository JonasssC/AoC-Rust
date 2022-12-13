use std::cmp::{min, Ordering};
use json::JsonValue;
use aoc_rust::util::input::read_lines_split_on_empty_line;

fn main() {
    let groups = read_lines_split_on_empty_line(2022, 13)
        .iter()
        .map(| g | g.iter()
            .map(| l | {
                json::parse(l).unwrap()
            })
            .collect::<Vec<JsonValue>>())
        .collect::<Vec<Vec<JsonValue>>>();
    println!("Part 1: {}", part1(groups.clone()));
    println!("Part 2: {}", part2(groups.clone()));
}

fn compare(left: JsonValue, right: JsonValue) -> Ordering {
    if left.is_number() && right.is_number() {
        compare_num(left, right)
    } else if left.is_array() && right.is_array() {
        compare_array(left, right)
    } else if left.is_number() {
        compare_array(JsonValue::Array(vec![left]), right)
    } else {
        compare_array(left, JsonValue::Array(vec![right]))
    }
}

fn compare_num(left: JsonValue, right: JsonValue) -> Ordering {
    let dif = right.as_i32().unwrap() - left.as_i32().unwrap();
    if dif == 0 {
        Ordering::Equal
    } else if dif > 0 {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

fn compare_array(left: JsonValue, right: JsonValue) -> Ordering {
    for i in 0..min(left.len(), right.len()) {
        let c = compare(left[i].clone(), right[i].clone());
        if c == Ordering::Less {
            return Ordering::Less;
        } else if c == Ordering::Greater {
            return Ordering::Greater;
        }
    }

    return if left.len() == right.len() {
        Ordering::Equal
    } else if left.len() > right.len() {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn part1(groups: Vec<Vec<JsonValue>>) -> usize {
    let mut sum = 0;
    'grouploop: for (i, group) in groups.iter().enumerate() {
        if compare(group[0].clone(), group[1].clone()) == Ordering::Less {
            continue 'grouploop;
        }
        sum += i + 1;
    }
    sum
}

fn part2(groups: Vec<Vec<JsonValue>>) -> usize {
    let mut flattened = groups.iter()
        .flatten()
        .cloned()
        .collect::<Vec<JsonValue>>();

    let new_1 = json::parse("[[2]]").unwrap();
    let new_2 = json::parse("[[6]]").unwrap();
    flattened.push(new_1.clone());
    flattened.push(new_2.clone());
    flattened.sort_by(| a, b | compare(b.clone(), a.clone()));

    (flattened.iter().position(| packet | packet.clone() == new_1).unwrap() + 1)
        * (flattened.iter().position(| packet | packet.clone() == new_2).unwrap() + 1)
}

