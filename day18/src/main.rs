use itertools::Itertools;
use std::cmp::max;
#[derive(Clone)]
enum Snailfish {
    Pair(Box<Snailfish>, Box<Snailfish>),
    Single(usize),
}

impl Snailfish {
    fn parse(s: &[u8], i: usize) -> (usize, Self) {
        match s[i] {
            b'[' => {
                let (i, left) = Self::parse(s, i + 1);
                let (i, right) = Self::parse(s, i + 1);
                (i + 1, Self::Pair(Box::new(left), Box::new(right)))
            }
            _ => (i + 1, Self::Single((s[i] - b'0') as usize)),
        }
    }

    fn reduce(mut self) -> Self {
        loop {
            // Check for 4 levels nested
            let depth = self.max_depth();
            // Explode if found
            if depth > 4 {
                if let Some((_, new, _)) = self.explode(depth) {
                    self = new;
                    continue;
                }
            }
            // check for numbers > 10
            // Split if found
            match self.split() {
                Some(new) => self = new,
                None => break self,
            }
        }
    }

    fn add_from_left(&self, snail: Option<Self>) -> Self {
        match (self, snail) {
            (_, None) => self.clone(),
            (Self::Single(v), Some(Self::Single(v2))) => Self::Single(*v + v2),
            (Self::Pair(v1, v2), snail) => {
                Self::Pair(Box::new(v1.add_from_left(snail)), v2.clone())
            }
            _ => unreachable!(),
        }
    }

    fn add_from_right(&self, snail: Option<Self>) -> Self {
        match (self, snail) {
            (_, None) => self.clone(),
            (Self::Single(v), Some(Self::Single(v2))) => Self::Single(*v + v2),
            (Self::Pair(v1, v2), snail) => {
                Self::Pair(v1.clone(), Box::new(v2.add_from_right(snail)))
            }
            _ => unreachable!(),
        }
    }

    fn explode(&mut self, depth: usize) -> Option<(Option<Self>, Self, Option<Self>)> {
        if let Self::Pair(first, second) = self {
            // move down recursively until hitting the needed depth
            if depth == 1 {
                // Set that to 0 and return left and right for adding
                return Some((Some(*first.clone()), Self::Single(0), Some(*second.clone())));
            }
            // Then move out adding the left to the outer left and right to outer right
            if let Some((left, result, right)) = first.explode(depth - 1) {
                let number = Self::Pair(Box::new(result), Box::new(second.add_from_left(right)));
                return Some((left, number, None));
            }
            if let Some((left, result2, right)) = second.explode(depth - 1) {
                let number = Self::Pair(Box::new(first.add_from_right(left)), Box::new(result2));
                return Some((None, number, right));
            }
        }
        None
    }

    fn split(&mut self) -> Option<Self> {
        match self {
            Self::Single(v) if *v >= 10 => {
                let left = *v / 2;
                let right = *v - left;
                return Some(Self::Pair(
                    Box::new(Self::Single(left)),
                    Box::new(Self::Single(right)),
                ));
            }
            Self::Pair(v1, v2) => {
                if let Some(left) = v1.split() {
                    return Some(Self::Pair(Box::new(left), v2.clone()));
                }
                if let Some(right) = v2.split() {
                    return Some(Self::Pair(v1.clone(), Box::new(right)));
                }
            }
            _ => {}
        }
        None
    }

    fn max_depth(&self) -> usize {
        match self {
            Self::Single(_) => 0,
            Self::Pair(first, second) => 1 + max(first.max_depth(), second.max_depth()),
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Self::Single(v) => *v,
            Self::Pair(v1, v2) => 3 * v1.magnitude() + 2 * v2.magnitude(),
        }
    }
}

pub fn main() {
    let numbers: Vec<Snailfish> = include_str!("../input.txt")
        .lines()
        .map(|l| Snailfish::parse(l.as_bytes(), 0).1)
        .collect();
    let part1 = part_one(&numbers);
    println!("Part 1: {}", part1);
    let part2 = part_two(&numbers);
    println!("Part 2: {}", part2);
}

fn part_one(numbers: &Vec<Snailfish>) -> usize {
    let result = numbers[1..]
        .iter()
        .fold(numbers[0].clone(), |first, second| add(&first, second))
        .magnitude();
    return result;
}

fn part_two(numbers: &Vec<Snailfish>) -> usize {
    let result = numbers
        .iter()
        .tuple_combinations()
        .flat_map(|(first, second)| {
            [
                add(first, second).magnitude(),
                add(second, first).magnitude(),
            ]
        })
        .max()
        .unwrap();
    return result;
}

fn add(first: &Snailfish, second: &Snailfish) -> Snailfish {
    Snailfish::Pair(Box::new(first.clone()), Box::new(second.clone())).reduce()
}

#[test]
fn example_p1() {
    let numbers: Vec<Snailfish> = include_str!("../test.txt")
        .lines()
        .map(|l| Snailfish::parse(l.as_bytes(), 0).1)
        .collect();
    let result = part_one(&numbers);
    assert_eq!(result, 4140);
}
#[test]
fn example_p2() {
    let numbers: Vec<Snailfish> = include_str!("../test.txt")
        .lines()
        .map(|l| Snailfish::parse(l.as_bytes(), 0).1)
        .collect();
    let result = part_two(&numbers);
    assert_eq!(result, 3993);
}
