use itertools::Itertools;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point(isize, isize, isize);

impl Point {
    fn turn_clockwise(&self, i: usize) -> Self {
        let mut point = Point(self.0, self.1, self.2);
        let mut iter = i;
        while iter != 0 {
            let newx = -point.2;
            let newz = point.0;
            point.0 = newx;
            point.2 = newz;
            iter -= 1;
        }
        point
    }

    fn tilt_up(&self, i: usize) -> Self {
        let mut point = Point(self.0, self.1, self.2);
        let mut iter = i;
        while iter != 0 {
            let newy = point.2;
            let newz = -point.1;
            point.1 = newy;
            point.2 = newz;
            iter -= 1;
        }
        point
    }
    fn rotate_clockwise(&self, i: usize) -> Self {
        let mut point = Point(self.0, self.1, self.2);
        let mut iter = i;
        while iter != 0 {
            let newx = point.1;
            let newy = -point.0;
            point.0 = newx;
            point.1 = newy;
            iter -= 1;
        }
        point
    }

    fn compare_pos(&self, point: &Point) -> (isize, isize, isize) {
        (self.0 - point.0, self.1 - point.1, self.2 - point.2)
    }
}
#[derive(Clone)]
struct Scanner {
    position: Option<Point>,
    beacons: Vec<Point>,
}

impl Scanner {
    fn compare_points(&self, points: &mut HashSet<Point>) -> Option<(isize, isize, isize)> {
        let beacons = &self.beacons;
        for x in 0..4 {
            let xbeacons: Vec<Point> = beacons.iter().map(|b| b.tilt_up(x)).collect();
            for y in 0..4 {
                let ybeacons: Vec<Point> = xbeacons.iter().map(|b| b.turn_clockwise(y)).collect();
                for z in 0..4 {
                    let rotated: Vec<Point> =
                        ybeacons.iter().map(|b| b.rotate_clockwise(z)).collect();
                    let deltas: Vec<(isize, isize, isize)> = points
                        .iter()
                        .cartesian_product(&rotated)
                        .map(|(point1, point2)| point1.compare_pos(point2))
                        .collect();
                    for (dx, dy, dz) in deltas {
                        let translated = rotated
                            .iter()
                            .map(|Point(x, y, z)| Point(x + dx, y + dy, z + dz));
                        if translated.clone().filter(|v| points.contains(v)).count() >= 12 {
                            points.extend(translated);
                            return Some((dx, dy, dz));
                        }
                    }
                }
            }
        }
        None
    }
}

pub fn main() {
    let mut scanners: Vec<Scanner> = include_str!("../input.txt")
        .split("\n\n")
        .map(|s| {
            let (_, coords) = s.split_once("\n").unwrap();
            let points: Vec<Point> = coords
                .lines()
                .map(|l| {
                    let (first, rest) = l.split_once(",").unwrap();
                    let (second, third) = rest.split_once(",").unwrap();
                    Point(
                        first.parse().unwrap(),
                        second.parse().unwrap(),
                        third.parse().unwrap(),
                    )
                })
                .collect();
            Scanner {
                position: None,
                beacons: points,
            }
        })
        .collect();
    scanners[0].position = Some(Point(0, 0, 0));

    let (part1, deltas) = part_one(&mut scanners.clone());
    println!("Part 1: {}", part1);
    let part2 = part_two(deltas);
    println!("Part 2: {}", part2);
}

fn part_one(scanners: &mut Vec<Scanner>) -> (usize, Vec<(isize, isize, isize)>) {
    let mut confirmed_positions: HashSet<Point> = HashSet::from_iter(scanners[0].beacons.clone());
    scanners.remove(0);
    let mut deltas = Vec::new();
    while !scanners.is_empty() {
        for i in (0..scanners.len()).rev() {
            if let Some(d) = scanners[i].compare_points(&mut confirmed_positions) {
                deltas.push(d);
                scanners.swap_remove(i);
            }
        }
    }

    return (confirmed_positions.len(), deltas);
}

fn part_two(deltas: Vec<(isize, isize, isize)>) -> isize {
    deltas
        .iter()
        .tuple_combinations()
        .map(|((x1, y1, z1), (x2, y2, z2))| (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs())
        .max()
        .unwrap()
}

#[test]
fn example_p1() {
    let mut scanners: Vec<Scanner> = include_str!("../test.txt")
        .split("\n\n")
        .map(|s| {
            let (_, coords) = s.split_once("\n").unwrap();
            let points: Vec<Point> = coords
                .lines()
                .map(|l| {
                    let (first, rest) = l.split_once(",").unwrap();
                    let (second, third) = rest.split_once(",").unwrap();
                    Point(
                        first.parse().unwrap(),
                        second.parse().unwrap(),
                        third.parse().unwrap(),
                    )
                })
                .collect();
            Scanner {
                position: None,
                beacons: points,
            }
        })
        .collect();
    scanners[0].position = Some(Point(0, 0, 0));

    let (part1, deltas) = part_one(&mut scanners.clone());
    assert_eq!(part1, 79);
    let part2 = part_two(deltas);
    assert_eq!(part2, 3621);
}
