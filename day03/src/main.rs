pub fn main() {
    let columns = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars())
        .fold(vec![(0, 0); 12], |mut acc, x| {
            for (i, item) in x.enumerate() {
                match item {
                    '0' => acc[i].0 += 1,
                    '1' => acc[i].1 += 1,
                    _ => panic!("invalid digit"),
                }
            }
            return acc;
        });
    let (gamma, epsilon) = columns.iter().fold((0b0, 0b0), |mut acc, x| {
        match x.0 > x.1 {
            true => {
                acc.0 = (acc.0 << 1) | 0;
                acc.1 = (acc.1 << 1) | 1;
            }
            false => {
                acc.0 = (acc.0 << 1) | 1;
                acc.1 = (acc.1 << 1) | 0;
            }
        }
        return acc;
    });

    let part2 = part_two();

    println!("Part 1: {}", gamma * epsilon);
    println!("Part 2: {}", part2);
}

fn part_two() -> u32 {
    let mut idx = 0;
    let mut lines: Vec<&str> = include_str!("../input.txt").lines().collect();
    let mut o2 = lines.clone();
    let mut co2 = lines.clone();

    while o2.len() > 1 {
        let num = o2
            .iter()
            .map(|l| l.as_bytes()[idx])
            .fold((0, 0), |mut acc, x| {
                match x {
                    b'0' => acc.0 += 1,
                    b'1' => acc.1 += 1,
                    _ => panic!("invalid input"),
                };
                return acc;
            });
        o2 = match num.0 > num.1 {
            true => o2
                .into_iter()
                .filter(|l| l.as_bytes()[idx] == b'0')
                .collect(),
            false => o2
                .into_iter()
                .filter(|l| l.as_bytes()[idx] == b'1')
                .collect(),
        };
        idx += 1;
    }
    idx = 0;
    while co2.len() > 1 {
        let num = co2
            .iter()
            .map(|l| l.as_bytes()[idx])
            .fold((0, 0), |mut acc, x| {
                match x {
                    b'0' => acc.0 += 1,
                    b'1' => acc.1 += 1,
                    _ => panic!("invalid input"),
                };
                return acc;
            });
        co2 = match num.0 > num.1 {
            true => co2
                .into_iter()
                .filter(|l| l.as_bytes()[idx] == b'1')
                .collect(),
            false => co2
                .into_iter()
                .filter(|l| l.as_bytes()[idx] == b'0')
                .collect(),
        };
        idx += 1;
    }

    let result = u32::from_str_radix(o2[0], 2).unwrap() * u32::from_str_radix(co2[0], 2).unwrap();
    return result;
}
