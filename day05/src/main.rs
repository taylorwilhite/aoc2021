#[derive(Clone)]
struct Coordinates {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}
struct Board {
    points: Vec<Vec<usize>>,
}

type Point = (usize, usize);

fn get_min_max(num1: Point, num2: Point) -> (Point, Point) {
    if num1.0 > num2.0 {
        return (num2, num1);
    } else {
        return (num1, num2);
    }
}
impl Board {
    fn new(coordinates: Vec<Coordinates>, diagonal: bool) -> Self {
        // Get maximum X and Y coords to determine vec length
        let max_x = coordinates.iter().fold(0, |acc, cur| {
            if acc > cur.x1 && acc > cur.x2 {
                return acc;
            } else if cur.x1 > acc && cur.x1 > cur.x2 {
                return cur.x1 + 1;
            }
            return cur.x2 + 1;
        });
        let max_y = coordinates.iter().fold(0, |acc, cur| {
            if acc > cur.y1 && acc > cur.y2 {
                return acc;
            } else if cur.y1 > acc && cur.y1 > cur.y2 {
                return cur.y1 + 1;
            }
            return cur.y2 + 1;
        });
        // initialize container
        let mut points = vec![vec![0; max_x]; max_y];

        for coord in coordinates {
            if coord.x1 != coord.x2 && coord.y1 != coord.y2 {
                if !diagonal {
                    continue;
                }
                let (left, right) = get_min_max((coord.x1, coord.y1), (coord.x2, coord.y2));
                let mut y_offset = 0;
                for i in left.0..(right.0 + 1) {
                    if left.1 < right.1 {
                        points[left.1 + y_offset][i] += 1;
                    } else {
                        points[left.1 - y_offset][i] += 1;
                    }
                    y_offset += 1;
                }
                continue;
            }
            if coord.x1 < coord.x2 {
                for i in coord.x1..(coord.x2 + 1) {
                    points[coord.y1][i] += 1;
                }
            } else if coord.x2 < coord.x1 {
                for i in coord.x2..(coord.x1 + 1) {
                    points[coord.y1][i] += 1;
                }
            } else if coord.y1 < coord.y2 {
                for i in coord.y1..(coord.y2 + 1) {
                    points[i][coord.x1] += 1;
                }
            } else if coord.y2 < coord.y1 {
                for i in coord.y2..(coord.y1 + 1) {
                    points[i][coord.x1] += 1;
                }
            }
        }

        Self { points: points }
    }

    fn get_crossover(&self) -> usize {
        let result = self.points.iter().fold(0, |acc, cur| {
            let sum = cur.iter().fold(0, |acc, x| {
                if x > &1 {
                    return acc + 1;
                }
                return acc;
            });
            acc + sum
        });
        return result;
    }
}
fn main() {
    let instructions: Vec<Coordinates> = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let (first, second) = l.split_once(" -> ").unwrap();
            let (firstx, firsty) = first.split_once(",").unwrap();
            let (secondx, secondy) = second.split_once(",").unwrap();
            Coordinates {
                x1: firstx.parse().unwrap(),
                y1: firsty.parse().unwrap(),
                x2: secondx.parse().unwrap(),
                y2: secondy.parse().unwrap(),
            }
        })
        .collect();
    let board = Board::new(instructions.clone(), false);
    let part1 = board.get_crossover();
    println!("Part 1: {}", part1);
    let board2 = Board::new(instructions, true);
    let part2 = board2.get_crossover();
    println!("Part 2: {}", part2)
}
