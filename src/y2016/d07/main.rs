use aoc_rust::util::input::read_lines;

fn main() {
    let lines = read_lines(2016, 7);
    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 2: {}", part2(lines.clone()));
}

fn part1(ips: Vec<String>) -> usize {
    let mut count = 0;

    'iploop: for ip in ips.iter() {
        let mut inside = false;
        let mut last_4 = ['\0'; 4];
        let mut valid = false;
        for c in ip.chars() {
            if c == '[' {
                inside = true;
                last_4 = ['\0'; 4];
            } else if c == ']' {
                inside = false;
                last_4 = ['\0'; 4];
            } else {
                last_4.rotate_left(1);
                last_4[3] = c;
                if last_4[0] == last_4[3] && last_4[1] == last_4[2] && last_4[0] != last_4[1] {
                    if inside {
                        continue 'iploop;
                    }

                    valid = true;
                }
            }
        }

        if valid {
            count += 1;
        }
    }

    count
}

fn invert(code: [char; 3]) -> [char; 3] {
    let mut inverted = code.clone();
    inverted.rotate_right(1);
    inverted[0] = inverted[2];
    inverted
}

fn part2(ips: Vec<String>) -> usize {
    let mut count = 0;

    'iploop: for ip in ips.iter() {
        let mut inside = false;
        let mut last_3= ['\0'; 3];
        let mut outer: Vec<[char; 3]> = Vec::new();
        let mut inner: Vec<[char; 3]> = Vec::new();

        for c in ip.chars() {
            if c == '[' {
                inside = true;
                last_3 = ['\0'; 3];
            } else if c == ']' {
                inside = false;
                last_3 = ['\0'; 3];
            } else {
                last_3.rotate_left(1);
                last_3[2] = c;
                if last_3[0] == last_3[2] && last_3[0] != last_3[1] {
                    if inside {
                        inner.push(last_3.clone());
                        if outer.contains(&invert(last_3.clone())) {
                            count += 1;
                            continue 'iploop;
                        }
                    } else {
                        outer.push(last_3.clone());
                        if inner.contains(&invert(last_3.clone())) {
                            count += 1;
                            continue 'iploop;
                        }
                    }
                }
            }
        }
    }

    count
}