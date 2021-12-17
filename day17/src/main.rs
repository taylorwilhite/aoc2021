struct Probe {
    x: isize,
    y: isize,
    x_vel: isize,
    y_vel: isize,
}

impl Probe {
    fn new(x_vel: isize, y_vel: isize) -> Self {
        Self {
            x: 0,
            y: 0,
            x_vel: x_vel,
            y_vel: y_vel,
        }
    }

    fn calculate_next_pos(&mut self) {
        self.x += self.x_vel;
        self.y += self.y_vel;
        self.y_vel -= 1;
        if self.x_vel != 0 {
            self.x_vel -= 1;
        }
    }

    fn is_in_target(&self, target: ((isize, isize), (isize, isize))) -> bool {
        let (min, max) = target;
        self.x >= min.0 && self.x <= max.0 && self.y >= min.1 && self.y <= max.1
    }

    fn will_not_hit_target(&self, target: ((isize, isize), (isize, isize))) -> bool {
        let (min, max) = target;
        if self.x > max.0 {
            return true;
        }
        if self.x_vel == 0 && self.x < min.0 {
            return true;
        }
        if self.y_vel <= 0 && self.y < min.1 {
            return true;
        }
        return false;
    }
}
pub fn main() {
    let (x, y): (&str, &str) = include_str!("../input.txt")
        .get(13..)
        .unwrap()
        .split_once(", ")
        .unwrap();
    let (xmin, xmax) = x.get(2..).unwrap().split_once("..").unwrap();
    let (ymin, ymax) = y.get(2..).unwrap().split_once("..").unwrap();
    let target = (
        (xmin.parse().unwrap(), ymin.parse().unwrap()),
        (xmax.parse().unwrap(), ymax.parse().unwrap()),
    );
    let part1 = part_one(target);
    println!("Part 1: {}", part1);
    let part2 = part_two(target);
    println!("Part 2: {}", part2);
}

fn part_one(target: ((isize, isize), (isize, isize))) -> isize {
    let mut highest = 0;
    let (min, max) = target;
    for x in 1..=max.0 {
        if x * 2 > max.0 && x < min.0 {
            continue;
        }
        if will_never_reach(x, min.0) {
            continue;
        }
        for y in 0..=min.1.abs() {
            let mut probe = Probe::new(x, y);
            let mut highpoint = 0;
            'ytest: loop {
                probe.calculate_next_pos();
                if probe.y > highpoint {
                    highpoint = probe.y;
                }

                if probe.will_not_hit_target(target) || probe.is_in_target(target) {
                    break 'ytest;
                }
            }
            if probe.is_in_target(target) {
                if highest < highpoint {
                    highest = highpoint;
                }
            }
        }
    }
    return highest;
}

fn part_two(target: ((isize, isize), (isize, isize))) -> isize {
    let mut hit = 0;
    let (min, max) = target;
    for x in 1..=max.0 {
        for y in min.1..=min.1.abs() {
            let mut probe = Probe::new(x, y);
            'ytest: loop {
                probe.calculate_next_pos();

                if probe.will_not_hit_target(target) || probe.is_in_target(target) {
                    break 'ytest;
                }
            }
            if probe.is_in_target(target) {
                hit += 1;
            }
        }
    }
    return hit;
}

fn will_never_reach(velocity: isize, target: isize) -> bool {
    if (velocity * (velocity + 1)) / 2 < target {
        return true;
    }
    return false;
}

#[test]
fn example_p1() {
    let (x, y): (&str, &str) = include_str!("../test.txt")
        .get(13..)
        .unwrap()
        .split_once(", ")
        .unwrap();
    println!("testing");
    let (xmin, xmax) = x.get(2..).unwrap().split_once("..").unwrap();
    let (ymin, ymax) = y.get(2..).unwrap().split_once("..").unwrap();
    let target = (
        (xmin.parse().unwrap(), ymin.parse().unwrap()),
        (xmax.parse().unwrap(), ymax.parse().unwrap()),
    );
    let result = part_one(target);
    assert_eq!(result, 45);
}
#[test]
fn example_p2() {
    let (x, y): (&str, &str) = include_str!("../test.txt")
        .get(13..)
        .unwrap()
        .split_once(", ")
        .unwrap();
    println!("testing");
    let (xmin, xmax) = x.get(2..).unwrap().split_once("..").unwrap();
    let (ymin, ymax) = y.get(2..).unwrap().split_once("..").unwrap();
    let target = (
        (xmin.parse().unwrap(), ymin.parse().unwrap()),
        (xmax.parse().unwrap(), ymax.parse().unwrap()),
    );
    let result = part_two(target);
    assert_eq!(result, 112);
}
