pub fn main() {
    let mut lines = include_str!("../input.txt")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let part1 = part_one(&mut lines);
    println!("Part 1: {}", part1);
    let part2 = part_two(&mut lines);
    println!("Part 2: {}", part2);
}

fn part_one(mut lines: &Vec<Vec<u32>>) -> usize {
    let mut sum = 0;
    let mut board = lines.clone();
    for _ in 0..100 {
        let ticks = tick_bursts(&mut board);
        sum += ticks;
    }

    return sum;
}

fn part_two(lines: &Vec<Vec<u32>>) -> usize {
    let mut result = 0;
    let mut board = lines.clone();
    for i in 1.. {
        let ticks = tick_bursts(&mut board);
        if ticks == 100 {
            result = i;
            break;
        }
    }
    return result;
}

fn tick_bursts(board: &mut Vec<Vec<u32>>) -> usize {
    let mut bursts = 0;
    let height = board.len();
    let width = board[0].len();
    for i in 0..height {
        board[i] = board[i].iter().map(|c| c + 1).collect();
    }
    for i in 0..height {
        for j in 0..width {
            if board[i][j] > 9 {
                bursts += burst_nine(j, i, width, height, board);
            }
        }
    }
    return bursts;
}

fn burst_nine(x: usize, y: usize, width: usize, height: usize, board: &mut Vec<Vec<u32>>) -> usize {
    board[y][x] = 0;
    let mut sum = 1;
    let neighbors = get_neighbors(x, y, width, height);
    neighbors.iter().for_each(|&(nx, ny)| {
        if board[ny][nx] == 0 {
            ()
        } else {
            board[ny][nx] += 1;
            if board[ny][nx] > 9 {
                sum += burst_nine(nx, ny, width, height, board);
            }
        }
    });
    return sum;
}

fn get_neighbors(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = Vec::new();
    if y != 0 {
        points.push((x, y - 1));
    }
    if y != 0 && x != 0 {
        points.push((x - 1, y - 1));
    }
    if y != 0 && x < width - 1 {
        points.push((x + 1, y - 1));
    }
    if y < height - 1 {
        points.push((x, y + 1));
    }
    if y < height - 1 && x != 0 {
        points.push((x - 1, y + 1));
    }
    if y < height - 1 && x < width - 1 {
        points.push((x + 1, y + 1));
    }
    if x != 0 {
        points.push((x - 1, y));
    }
    if x < width - 1 {
        points.push((x + 1, y));
    }
    return points;
}
