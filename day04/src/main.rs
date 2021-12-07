#[derive(Debug)]
struct Board {
    rows: Vec<Vec<BingoSpace>>,
    completed: bool,
    finished_at: usize,
}

impl Board {
    fn is_bingo(&self) -> bool {
        if self.completed {
            return true;
        }
        // Check rows
        if let Some(b) = self.rows.iter().find(|row| row.iter().all(|v| v.called)) {
            // self.completed = b.iter().fold(0, |acc, x| acc + x.value);
            return true;
        }
        // Check columns
        for i in 0..5 {
            if self.rows.iter().all(|row| row[i].called) {
                let mut sum = 0;
                for j in 0..self.rows.len() {
                    sum += self.rows[j][i].value
                }
                // self.completed = sum;
                return true;
            }
        }
        return false;
    }
    fn get_unmarked(&self) -> u32 {
        let unmarked_sum = self
            .rows
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|v| !v.called)
                    .fold(0, |acc, x| acc + x.value)
            })
            .fold(0, |acc, x| acc + x);
        return unmarked_sum;
    }
    fn update_spaces(&mut self, num: u32, idx: usize) {
        for i in 0..self.rows.len() {
            for j in 0..self.rows.len() {
                if self.rows[i][j].value == num {
                    self.rows[i][j].called = true;
                }
            }
        }
        if !self.completed && self.is_bingo() {
            self.completed = true;
            self.finished_at = idx;
        }
    }
}

impl std::clone::Clone for Board {
    fn clone(&self) -> Self {
        Self {
            rows: self.rows.clone(),
            completed: self.completed,
            finished_at: self.finished_at,
        }
    }
}
#[derive(Debug)]
struct BingoSpace {
    value: u32,
    called: bool,
}
impl std::clone::Clone for BingoSpace {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            called: self.called,
        }
    }
}
fn main() {
    let (numstring, input) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let nums: Vec<u32> = numstring.split(",").map(|s| s.parse().unwrap()).collect();
    let boards: Vec<Board> = input
        .split("\n\n")
        .map(|b| {
            let lines = b
                .lines()
                .map(|l| {
                    l.split_whitespace()
                        .map(|s| {
                            return BingoSpace {
                                value: s.parse().unwrap(),
                                called: false,
                            };
                        })
                        .collect()
                })
                .collect();
            return Board {
                rows: lines,
                completed: false,
                finished_at: 0,
            };
        })
        .collect();

    let boards2 = boards.clone();
    let nums2 = nums.clone();
    let part1 = part_one(nums, boards);
    println!("Part 1: {}", part1);
    let part2 = part_two(nums2, boards2);
    println!("Part 2: {}", part2);
}

fn part_one(nums: Vec<u32>, mut boards: Vec<Board>) -> u32 {
    let mut idx = 0;
    loop {
        // read nums and check for bingo here
        let num = nums[idx];
        for i in 0..boards.len() {
            boards[i].update_spaces(num, idx)
        }

        if boards.iter().any(|board| board.is_bingo()) {
            break;
        }

        idx += 1;
    }
    // Grab first bingo board
    let winner = boards.iter().find(|board| board.is_bingo()).unwrap();
    let sum = winner.get_unmarked();
    return sum * nums[idx];
}

fn part_two(nums: Vec<u32>, mut boards: Vec<Board>) -> u32 {
    let mut idx = 0;
    loop {
        // read nums and check for bingo here
        let num = nums[idx];
        for i in 0..boards.len() {
            boards[i].update_spaces(num, idx)
        }

        if !boards.iter().any(|board| !board.is_bingo()) {
            break;
        }

        idx += 1;
    }
    // Grab first bingo board
    let loser = boards.iter().fold(&boards[0], |acc, board| {
        if board.finished_at > acc.finished_at {
            board
        } else {
            acc
        }
    });
    let sum = loser.get_unmarked();
    return sum * nums[idx];
}
