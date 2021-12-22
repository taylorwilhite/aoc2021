use std::cmp::{max, min};
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    On((isize, isize), (isize, isize), (isize, isize)),
    Off((isize, isize), (isize, isize), (isize, isize)),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, coords) = s.split_once(" ").unwrap();
        let ranges: Vec<(isize, isize)> = coords
            .split(",")
            .map(|s| {
                let (min, max) = s.get(2..).unwrap().split_once("..").unwrap();
                (min.parse().unwrap(), max.parse().unwrap())
            })
            .collect();
        match direction {
            "on" => Ok(Self::On(ranges[0], ranges[1], ranges[2])),
            "off" => Ok(Self::Off(ranges[0], ranges[1], ranges[2])),
            _ => unreachable!(),
        }
    }
}
pub fn main() {
    let instructions: Vec<Instruction> = include_str!("../input.txt")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
    let part1 = part_one(&instructions);
    println!("Part 1: {}", part1);
    let part2 = part_two(&instructions);
    println!("Part 2: {}", part2);
}

fn part_one(instructions: &Vec<Instruction>) -> usize {
    let mut reactor = HashMap::new();
    for instruction in instructions {
        match instruction {
            Instruction::On(xr, yr, zr) => {
                if xr.0 > 50 || yr.0 > 50 || zr.0 > 50 || xr.1 < -50 || yr.1 < -50 || zr.1 < -50 {
                    continue;
                }
                for x in xr.0..=xr.1 {
                    if x < -50 || x > 50 {
                        continue;
                    }
                    for y in yr.0..=yr.1 {
                        if y < -50 || y > 50 {
                            continue;
                        }
                        for z in zr.0..=zr.1 {
                            if z < -50 || z > 50 {
                                continue;
                            }
                            let entry = reactor.entry((x, y, z)).or_insert(false);
                            *entry = true;
                        }
                    }
                }
            }
            Instruction::Off(xr, yr, zr) => {
                for x in xr.0..=xr.1 {
                    if x < -50 || x > 50 {
                        continue;
                    }
                    for y in yr.0..=yr.1 {
                        if y < -50 || y > 50 {
                            continue;
                        }
                        for z in zr.0..=zr.1 {
                            if z < -50 || z > 50 {
                                continue;
                            }
                            let entry = reactor.entry((x, y, z)).or_insert(false);
                            *entry = false;
                        }
                    }
                }
            }
        }
    }

    reactor.values().filter(|&v| *v).count()
}

fn part_two(instructions: &Vec<Instruction>) -> isize {
    (0..instructions.len())
        .filter(|&i| {
            if let Instruction::On(_, _, _) = instructions[i] {
                return true;
            } else {
                return false;
            }
        })
        .map(|i| corrected_volume(instructions[i], &instructions[i + 1..]))
        .sum()
}

fn corrected_volume(cube: Instruction, rest: &[Instruction]) -> isize {
    let subcubes: Vec<Instruction> = rest
        .iter()
        .filter_map(|&c2| intersects(&c2, &cube))
        .collect();
    let vsubcubes: isize = (0..subcubes.len())
        .map(|i| corrected_volume(subcubes[i], &subcubes[i + 1..]))
        .sum();
    return get_area(&cube) - vsubcubes;
}

fn intersects(cube1: &Instruction, cube2: &Instruction) -> Option<Instruction> {
    let (x1, y1, z1) = match cube1 {
        Instruction::On(xr, yr, zr) => (xr, yr, zr),
        Instruction::Off(xr, yr, zr) => (xr, yr, zr),
    };
    let (x2, y2, z2) = match cube2 {
        Instruction::On(xr, yr, zr) => (xr, yr, zr),
        Instruction::Off(xr, yr, zr) => (xr, yr, zr),
    };
    let xd = subaxis(*x1, *x2)?;
    let yd = subaxis(*y1, *y2)?;
    let zd = subaxis(*z1, *z2)?;
    match cube1 {
        Instruction::On(_, _, _) => Some(Instruction::On(xd, yd, zd)),
        Instruction::Off(_, _, _) => Some(Instruction::Off(xd, yd, zd)),
    }
}

fn subaxis((a, b): (isize, isize), (low, high): (isize, isize)) -> Option<(isize, isize)> {
    if b < low {
        return None;
    }
    if a > high {
        return None;
    }
    let a = min(max(a, low), high);
    let b = min(max(b, low), high);
    Some((a, b))
}

fn get_area(i: &Instruction) -> isize {
    let cube = match i {
        Instruction::On(xr, yr, zr) => (xr, yr, zr),
        Instruction::Off(xr, yr, zr) => (xr, yr, zr),
    };
    let dx = (cube.0 .1 - cube.0 .0 + 1);
    let dy = (cube.1 .1 - cube.1 .0 + 1);
    let dz = (cube.2 .1 - cube.2 .0 + 1);
    let cube_area = dx * dy * dz;
    return cube_area;
}

#[test]
fn get_area_test() {
    let instruction = Instruction::On((0, 9), (0, 9), (0, 9));
    let result = get_area(&instruction);
    assert_eq!(result, 1000);
}

#[test]
fn example_p2() {
    let instructions: Vec<Instruction> = include_str!("../test.txt")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
    let result = part_two(&instructions);
    assert_eq!(result, 2758514936282235);
}
