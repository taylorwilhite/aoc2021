pub fn main() {
    let lines: Vec<(&str, usize)> = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|t| (t.0, t.1.parse().unwrap()))
        .collect();
    println!("Part 1: {}", part_one(&lines));
    println!("Part 2: {}", part_two(&lines));
}

fn part_one(lines: &Vec<(&str, usize)>) -> usize {
    let mut depth = 0;
    let mut horizontal = 0;

    for l in lines {
        match l.0 {
            "up" => depth -= l.1,
            "down" => depth += l.1,
            "forward" => horizontal += l.1,
            _ => panic!("unexpected input"),
        }
    }
    depth * horizontal
}

fn part_two(lines: &Vec<(&str, usize)>) -> usize {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for l in lines {
        match l.0 {
            "up" => aim -= l.1,
            "down" => aim += l.1,
            "forward" => {
                horizontal += l.1;
                depth += aim * l.1;
            }
            _ => panic!("unexpected input"),
        }
    }

    depth * horizontal
}
