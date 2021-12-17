pub fn main() {
    let positions: Vec<isize> = include_str!("../input.txt")
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let part1 = part_one(&positions);
    println!("Part 1: {}", part1);
    let part2 = part_two(&positions);
    println!("Part 2: {}", part2);
}

fn part_one(positions: &Vec<isize>) -> isize {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    let mut min_total = positions.iter().fold(0, |acc, x| acc + (x - max).abs());
    let mut result = 0;
    for i in min..max {
        let change = positions.iter().fold(0, |acc, x| acc + (x - i).abs());
        if change < min_total {
            result = i;
            min_total = change;
        }
    }
    return min_total;
}

fn part_two(positions: &Vec<isize>) -> isize {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    let mut min_total = positions.iter().fold(0, |acc, x| {
        let spaces = (x - max).abs();
        let cost = (spaces * (spaces + 1)) / 2;
        acc + cost
    });
    let mut result = 0;
    for i in min..max {
        let change = positions.iter().fold(0, |acc, x| {
            let spaces = (x - i).abs();
            let cost = (spaces * (spaces + 1)) / 2;
            acc + cost
        });
        if change < min_total {
            result = i;
            min_total = change;
        }
    }
    return min_total;
}
