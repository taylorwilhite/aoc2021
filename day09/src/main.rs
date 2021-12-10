use std::collections::HashSet;
fn main() {
    let rows: Vec<Vec<char>> = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let (part1, lows) = part_one(&rows);
    println!("Part 1: {}", part1);
    let part2 = part_two(&rows, lows);
    println!("Part 2: {}", part2);
}

fn part_one(rows: &Vec<Vec<char>>) -> (u32, Vec<Lowpoint>) {
    let mut sum = 0;
    let mut lows = Vec::new();
    for i in 0..rows.len() {
        for j in 0..rows[i].len() {
            let point = rows[i][j];
            if i != 0 && point >= rows[i - 1][j] {
                continue;
            }
            if j != 0 && point >= rows[i][j - 1] {
                continue;
            }
            if j + 1 != rows[i].len() && point >= rows[i][j + 1] {
                continue;
            }
            if i + 1 != rows.len() && point >= rows[i + 1][j] {
                continue;
            }
            sum += point.to_digit(10).unwrap() + 1;
            lows.push(Lowpoint { x: j, y: i });
        }
    }
    return (sum, lows);
}

struct Lowpoint {
    x: usize,
    y: usize,
}

fn part_two(rows: &Vec<Vec<char>>, lows: Vec<Lowpoint>) -> usize {
    let mut basins: Vec<usize> = Vec::new();
    let height = rows.len();
    let width = rows[0].len();
    for low in lows {
        let mut visited = HashSet::new();
        let mut stack = vec![(low.x, low.y)];
        while let Some((x, y)) = stack.pop() {
            if !visited.insert((x, y)) {
                continue;
            }
            get_neighbors(x, y, width, height)
                .iter()
                .filter(|(nx, ny)| rows[*ny][*nx] != '9')
                .for_each(|&point| stack.push(point));
        }
        basins.push(visited.len())
    }
    basins.sort();
    basins.reverse();
    return basins[0] * basins[1] * basins[2];
}

fn get_neighbors(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = Vec::new();
    if y != 0 {
        points.push((x, y - 1));
    }
    if y < height - 1 {
        points.push((x, y + 1));
    }
    if x != 0 {
        points.push((x - 1, y));
    }
    if x < width - 1 {
        points.push((x + 1, y));
    }
    return points;
}
