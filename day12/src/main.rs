use std::collections::HashMap;

struct Cave {
    large: bool,
    connects_to: Vec<&'static str>,
}

impl Cave {
    fn new(cave: &str) -> Self {
        let large = is_large(cave);

        Self {
            large: large,
            connects_to: Vec::new(),
        }
    }
}

fn is_large(name: &str) -> bool {
    match name.as_bytes()[0] {
        b'A'..=b'Z' => true,
        _ => false,
    }
}

pub fn main() {
    let links: Vec<&str> = include_str!("../input.txt").lines().collect();
    let mut links_map = HashMap::new();
    for link in links {
        let (first, second) = link.split_once("-").unwrap();
        links_map
            .entry(first)
            .or_insert(Cave::new(first))
            .connects_to
            .push(second);
        links_map
            .entry(second)
            .or_insert(Cave::new(second))
            .connects_to
            .push(first);
    }
    let part1 = part_one(&links_map);
    println!("Part 1: {}", part1);
    let part2 = part_two(&links_map);
    println!("Part 2: {}", part2);
}

fn part_one(links: &HashMap<&str, Cave>) -> usize {
    let mut paths = 0;
    let mut visited: Vec<&str> = Vec::new();
    // let start = links.get("start").unwrap();
    paths += calculate_paths("start", &links, &mut visited, false, false);
    return paths;
}

fn part_two(links: &HashMap<&str, Cave>) -> usize {
    let mut paths = 0;
    let mut visited: Vec<&str> = Vec::new();
    paths += calculate_paths("start", &links, &mut visited, true, false);
    return paths;
}

fn calculate_paths(
    cave: &'static str,
    links: &HashMap<&str, Cave>,
    visited: &mut Vec<&str>,
    second_small: bool,
    has_second_small: bool,
) -> usize {
    visited.push(cave);
    let mut end_paths = 0;
    let value = links.get(cave).unwrap();
    if value.large {
        if let None = value.connects_to.iter().find(|s| !visited.contains(s)) {
            if !second_small || has_second_small {
                return 0;
            }
        }
    }
    for i in 0..value.connects_to.len() {
        let mut new_has_second = has_second_small;
        if value.connects_to[i] == "end" {
            end_paths += 1;
            continue;
        }
        let next = links.get(&value.connects_to[i]).unwrap();
        if !next.large && visited.contains(&value.connects_to[i]) {
            if second_small && !has_second_small && value.connects_to[i] != "start" {
                new_has_second = true;
            } else {
                continue;
            }
        }
        let mut new_visits = visited.clone();
        end_paths += calculate_paths(
            value.connects_to[i],
            links,
            &mut new_visits,
            second_small,
            new_has_second,
        );
    }
    return end_paths;
}
