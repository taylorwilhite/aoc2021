fn main() {
    let lines: Vec<&str> = include_str!("../input.txt").lines().collect();
    let (part1, incomplete) = part_one(lines);
    println!("Part 1: {}", part1);
    let part2 = part_two(&incomplete);
    println!("Part 2: {}", part2);
}

fn part_one(lines: Vec<&str>) -> (usize, Vec<&str>) {
    let mut total = 0;
    let mut incomplete: Vec<&str> = Vec::new();
    for line in lines {
        let mut is_incomplete = true;
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            match c {
                '{' | '(' | '[' | '<' => stack.push(c),
                '}' => {
                    if stack.pop().unwrap() != '{' {
                        total += bracket_points(c);
                        is_incomplete = false;
                    }
                }
                ')' => {
                    if stack.pop().unwrap() != '(' {
                        total += bracket_points(c);
                        is_incomplete = false;
                    }
                }
                ']' => {
                    if stack.pop().unwrap() != '[' {
                        total += bracket_points(c);
                        is_incomplete = false;
                    }
                }
                '>' => {
                    if stack.pop().unwrap() != '<' {
                        total += bracket_points(c);
                        is_incomplete = false;
                    }
                }
                _ => panic!("invalid char"),
            }
        }
        if is_incomplete {
            incomplete.push(line);
        }
    }
    return (total, incomplete);
}

fn part_two(lines: &Vec<&str>) -> usize {
    let mut results: Vec<usize> = Vec::new();
    for line in lines {
        let mut sum = 0;
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            match c {
                '{' | '(' | '[' | '<' => stack.push(c),
                '}' | ')' | ']' | '>' => {
                    stack.pop();
                    ()
                }
                _ => panic!("unexpected char"),
            }
        }
        while stack.len() != 0 {
            if let Some(c) = stack.pop() {
                let points = match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!("unexpected char"),
                };
                sum = sum * 5;
                sum += points;
            }
        }
        results.push(sum)
    }
    results.sort();
    let index = (results.len() - 1) / 2;

    return results[index];
}

fn bracket_points(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("invalid character"),
    }
}
