use std::collections::VecDeque;
fn main() {
    let mut fish: VecDeque<usize> = include_str!("../input.txt")
        .split(",")
        .map(|s| s.parse().unwrap())
        .fold(VecDeque::from(vec![0; 9]), |mut acc, x: usize| {
            acc[x] += 1;
            return acc;
        });
    for _ in 0..256 {
        let new = fish.pop_front().unwrap();
        fish[6] += new;
        fish.push_back(new);
    }

    println!("Part 1: {}", fish.iter().fold(0, |acc, x| acc + x));
}
