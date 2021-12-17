use std::collections::HashSet;
#[derive(Copy, Clone)]
enum Fold {
    Up(usize),
    Left(usize),
}
pub fn main() {
    let (coords, foldstring) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let points: HashSet<(usize, usize)> = coords
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    let folds: Vec<Fold> = foldstring
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| {
            let (direction, number) = s.split_once("=").unwrap();
            match direction {
                "fold along x" => Fold::Left(number.parse().unwrap()),
                "fold along y" => Fold::Up(number.parse().unwrap()),
                _ => panic!("unexpected instruction"),
            }
        })
        .collect();
    let part1 = fold(&points, folds[0]);
    println!("Part 1: {}", part1.len());
    part_two(points, folds);
}

fn part_two(grid: HashSet<(usize, usize)>, folds: Vec<Fold>) {
    let part2 = folds.iter().fold(grid, |grid, &x| fold(&grid, x));
    let max_y = part2.iter().map(|&(_, y)| y).max().unwrap();
    let max_x = part2.iter().map(|&(x, _)| x).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            let c = if part2.contains(&(x, y)) { '#' } else { ' ' };
            print!("{}", c);
        }
        println!()
    }
}

fn fold(dots: &HashSet<(usize, usize)>, fold: Fold) -> HashSet<(usize, usize)> {
    dots.iter()
        .map(|&(x, y)| match (fold, x, y) {
            (Fold::Left(v), x, y) if x < v => (x, y),
            (Fold::Left(v), x, y) => (v * 2 - x, y),
            (Fold::Up(v), x, y) if y < v => (x, y),
            (Fold::Up(v), x, y) => (x, v * 2 - y),
            _ => unreachable!(),
        })
        .collect()
}
