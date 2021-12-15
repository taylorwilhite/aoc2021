use itertools::Itertools;
use std::collections::HashMap;
fn main() {
    let (compound, rules) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let rules: HashMap<&[u8], char> = rules
        .lines()
        .map(|l| {
            let (slice, input) = l.split_once(" -> ").unwrap();
            (slice.as_bytes(), input.chars().collect::<Vec<char>>()[0])
        })
        .collect();
    let map_rules: HashMap<(char, char), char> = rules
        .iter()
        .map(|(slice, &c)| ((slice[0] as char, slice[1] as char), c))
        .collect();
    let part1 = add_polymer(compound, &map_rules, 10);
    println!("Part 1: {}", part1);
    let part2 = add_polymer(compound, &map_rules, 40);
    println!("Part 2: {}", part2);
}

fn add_polymer(compound: &str, rules: &HashMap<(char, char), char>, steps: usize) -> usize {
    let init_counts = compound.chars().tuple_windows().counts();
    let pair_counts = (0..steps).fold(init_counts, |counts, _| {
        let mut next = HashMap::new();
        for ((a, b), count) in counts {
            let c = rules[&(a, b)];
            *next.entry((a, c)).or_insert(0) += count;
            *next.entry((c, b)).or_insert(0) += count;
        }
        next
    });
    let mut count = HashMap::new();
    for ((a, _), c) in pair_counts {
        *count.entry(a).or_insert(0) += c;
    }
    *count.entry(compound.chars().last().unwrap()).or_insert(0) += 1;
    let (min, max) = count.values().minmax().into_option().unwrap();
    max - min
}
