use std::collections::HashMap;
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Player {
    position: usize,
    points: usize,
}

impl Player {
    fn advance_by(&mut self, spaces: usize) -> Player {
        let mut position = self.position + spaces;
        if position > 10 {
            position -= 10;
        }
        Player {
            position: position,
            points: self.points + position,
        }
    }
}

struct Die {
    current: usize,
    rolled: usize,
}

impl Die {
    fn roll_deterministic(&mut self) -> usize {
        self.rolled += 3;
        match self.current {
            100 => {
                self.current = 3;
                6
            }
            99 => {
                self.current = 2;
                103 % 10
            }
            98 => {
                self.current = 1;
                200 % 10
            }
            _ => {
                let result = (self.current + 1) + (self.current + 2) + (self.current + 3);
                self.current += 3;
                result % 10
            }
        }
    }
}

pub fn main() {
    let (init1, init2) = (3, 10);
    let part1 = part_one(init1, init2);
    println!("Part 1: {}", part1);
    let part2 = part_two(init1, init2);
    println!("Part 2: {}", part2);
}

fn part_one(init1: usize, init2: usize) -> usize {
    let mut p1 = Player {
        position: init1,
        points: 0,
    };
    let mut p2 = Player {
        position: init2,
        points: 0,
    };
    let mut die = Die {
        current: 0,
        rolled: 0,
    };

    while p1.points < 1000 && p2.points < 1000 {
        let p1roll = die.roll_deterministic();
        p1.position += p1roll;
        if p1.position > 10 {
            p1.position -= 10;
        }
        p1.points += p1.position;
        if p1.points >= 1000 {
            break;
        }
        let p2roll = die.roll_deterministic();
        p2.position += p2roll;
        if p2.position > 10 {
            p2.position -= 10;
        }
        p2.points += p2.position;
    }
    let points = if p1.points > p2.points {
        p2.points
    } else {
        p1.points
    };
    return die.rolled * points;
}
struct Multiverse {
    p1: usize,
    p2: usize,
    universes: HashMap<(Player, Player, bool), usize>,
}
// (3, 1) (4, 3) (5, 6) (6, 7) (7, 6) (8, 3) (9, 1)

impl Multiverse {
    fn tick_universes(&mut self) {
        let rolls = vec![(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
        for universe in self.universes.clone().iter() {
            let (&(mut p1, mut p2, p1turn), count) = universe;
            for (roll, multiple) in &rolls {
                if p1turn {
                    let newp1 = p1.advance_by(*roll);
                    if newp1.points >= 21 {
                        self.p1 += multiple * count;
                    } else {
                        *self.universes.entry((newp1, p2, !p1turn)).or_insert(0) +=
                            multiple * count;
                    }
                } else {
                    let newp2 = p2.advance_by(*roll);
                    if newp2.points >= 21 {
                        self.p2 += multiple * count;
                    } else {
                        *self.universes.entry((p1, newp2, !p1turn)).or_insert(0) +=
                            multiple * count;
                    }
                }
            }
            self.universes.remove(&(p1, p2, p1turn));
        }
    }
}
fn part_two(init1: usize, init2: usize) -> usize {
    let mut p1 = Player {
        position: init1,
        points: 0,
    };
    let mut p2 = Player {
        position: init2,
        points: 0,
    };
    let mut multiverse = Multiverse {
        p1: 0,
        p2: 0,
        universes: HashMap::new(),
    };
    multiverse.universes.insert((p1, p2, true), 1);
    while !multiverse.universes.is_empty() {
        multiverse.tick_universes();
    }
    let result = if multiverse.p1 > multiverse.p2 {
        multiverse.p1
    } else {
        multiverse.p2
    };
    return result;
}

fn roll_dirac() {}

#[test]
fn example_p1() {
    let (init1, init2) = (4, 8);
    let result = part_one(init1, init2);
    assert_eq!(result, 739785);
}

#[test]
fn example_p2() {
    let (init1, init2) = (4, 8);
    let result = part_two(init1, init2);
    assert_eq!(result, 444356092776315);
}
