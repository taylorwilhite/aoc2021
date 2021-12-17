pub fn main() {
    let input: Vec<(&str, &str)> = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(" | ").unwrap())
        .collect();
    let part1 = part_one(&input);
    println!("Part 1: {}", part1);
    let part2 = part_two(&input);
    println!("Part 2: {}", part2);
}

fn part_one(input: &Vec<(&str, &str)>) -> usize {
    let results: Vec<&str> = input
        .iter()
        .map(|i| i.1)
        .map(|j| j.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
        .concat();
    let count: usize = results.iter().fold(0, |acc, x| match x.len() {
        2 | 3 | 4 | 7 => return acc + 1,
        _ => return acc,
    });
    return count;
}

fn part_two(input: &Vec<(&str, &str)>) -> usize {
    let mut count = 0;
    for line in input {
        let sum = get_sum(line);
        count += sum;
    }
    return count;
}

fn get_sum(input: &(&str, &str)) -> usize {
    let digits: Vec<&str> = input.0.split(" ").collect();
    let results: Vec<&str> = input.1.split(" ").collect();
    let digit_map = DigitMap::new(digits);
    digit_map.decode(results)
}

struct DigitMap {
    one: Vec<char>,
    four: Vec<char>,
}

impl DigitMap {
    fn new(digits: Vec<&str>) -> Self {
        let one: Vec<char> = digits
            .iter()
            .find(|d| d.len() == 2)
            .unwrap()
            .chars()
            .collect();
        let four: Vec<char> = digits
            .iter()
            .find(|d| d.len() == 4)
            .unwrap()
            .chars()
            .collect();

        Self {
            one: one,
            four: four,
        }
    }

    fn common_one(&self, s: &str) -> usize {
        self.one
            .iter()
            .fold(0, |acc, &c| if s.contains(c) { acc + 1 } else { acc })
    }

    fn common_four(&self, s: &str) -> usize {
        self.four
            .iter()
            .fold(0, |acc, &c| if s.contains(c) { acc + 1 } else { acc })
    }

    fn decode_digit(&self, s: &str) -> usize {
        match (s.len(), self.common_one(s), self.common_four(s)) {
            (2, _, _) => 1,
            (5, 1, 2) => 2,
            (5, 2, 3) => 3,
            (4, _, _) => 4,
            (5, 1, 3) => 5,
            (6, 1, 3) => 6,
            (3, _, _) => 7,
            (7, _, _) => 8,
            (6, 2, 4) => 9,
            (6, 2, 3) => 0,
            (_, _, _) => panic!("invalid"),
        }
    }

    fn decode(&self, digits: Vec<&str>) -> usize {
        let thousand = self.decode_digit(digits[0]) * 1000;
        let hundred = self.decode_digit(digits[1]) * 100;
        let ten = self.decode_digit(digits[2]) * 10;
        let one = self.decode_digit(digits[3]);

        thousand + hundred + ten + one
    }
}
