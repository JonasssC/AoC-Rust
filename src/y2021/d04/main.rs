use aoc_rust::util::input::read_split_on_empty_line;

#[derive(Clone)]
struct Card {
    values: [[i32; 5]; 5],
    marked: [[bool; 5]; 5]
}

impl Card {
    pub fn parse(str: String) -> Card {
        let mut values = [[0; 5]; 5];

        for (i, line) in str.lines().enumerate() {
            for (j, num_str) in line.split_whitespace().enumerate() {
                values[i][j] = num_str.parse::<i32>().unwrap();
            }
        }

        return Card {
            values,
            marked: [[false; 5]; 5]
        }
    }

    pub fn mark_num(&mut self, num: i32) {
        for (i, row) in self.values.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if *val == num {
                    self.marked[i][j] = true;
                }
            }
        }
    }

    pub fn has_won(&self) -> bool {
        for row in self.marked {
            if row == [true; 5] {
                return true;
            }
        }

        for i in 0..5 {
            let mut winning = true;
            for j in 0..5 {
                if !self.marked[j][i] {
                    winning = false;
                    break;

                }
            }
            if winning {
                return true;
            }
        }

        return false;
    }

    pub fn calc_score(&self) -> i32 {
        let mut score = 0;
        for (i, row) in self.values.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if !self.marked[i][j] {
                    score += val;
                }
            }
        }
        return score;
    }
}

fn parse_nums(num_str: String) -> Vec<i32> {
    return num_str.split(",")
        .map(| n | n.parse::<i32>().unwrap())
        .collect();
}

fn main() {
    let lines = read_split_on_empty_line(2021, 4);
    let nums = parse_nums(lines[0].clone());
    let cards: Vec<Card> = lines[1..].iter()
        .map(| s | Card::parse(s.clone()))
        .collect();
    println!("Part 1: {}", part1(nums.clone(), cards.iter().map(| c | (*c).clone()).collect()));
    println!("Part 2: {}", part2(nums, cards.iter().map(| c | (*c).clone()).collect()));
}

fn part1(nums: Vec<i32>, mut cards: Vec<Card>) -> i32 {
    for num in nums {
        for card in cards.iter_mut() {
            card.mark_num(num);
            if card.has_won() {
                return card.calc_score() * num;
            }
        }
    }

    return 0;
}

fn part2(nums: Vec<i32>, mut cards: Vec<Card>) -> i32 {
    for num in nums {
        let mut i = 0;
        while i < cards.len() {
            cards[i].mark_num(num);
            if cards[i].has_won() {
                if cards.len() == 1 {
                    return cards[0].calc_score() * num;
                }
                cards.remove(i);
            } else {
                i += 1;
            }
        }
    }
    return 0;
}
