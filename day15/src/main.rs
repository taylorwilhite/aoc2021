use std::collections::VecDeque;
#[derive(Copy, Clone)]
struct Node {
    visited: bool,
    danger: usize,
    value: usize,
    active: bool,
}

impl std::str::FromStr for Node {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let danger: usize = s.parse().unwrap();
        Ok(Self {
            visited: false,
            danger: danger,
            value: usize::MAX,
            active: false,
        })
    }
}
fn main() {
    let mut graph: Vec<Vec<Node>> = include_str!("../input.txt")
        .lines()
        .map(|l| {
            l.split("")
                .filter(|&s| s != "")
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<Node>>>();
    let width = graph[0].len();
    let height = graph.len();
    let mut nodes = graph.clone();
    // let mut expanded: Vec<Vec<Node>> = (0..(5 * height))
    //     .map(|y| {
    //         (0..(5 * width))
    //             .map(|x| {
    //                 let mut danger = nodes[y % height][x % width].danger
    //                     + (x / width) as usize
    //                     + (y / height) as usize;
    //                 if danger >= 10 {
    //                     danger -= 9
    //                 };
    //                 Node {
    //                     active: false,
    //                     visited: false,
    //                     value: usize::MAX,
    //                     danger: danger,
    //                 }
    //             })
    //             .collect()
    //     })
    //     .collect();
    for i in 0..5 {
        let nheight = height * i;
        let mut final_rows: Vec<Vec<Node>> = Vec::new();
        for _ in 0..height {
            final_rows.push(Vec::new());
        }

        for j in 0..5 {
            if i + j == 0 {
                continue;
            }
            let nwidth = width * j;
            let rows: Vec<Vec<Node>> = graph
                .iter()
                .map(|r| {
                    r.iter()
                        .map(|n| {
                            let mut dangervalue = n.danger + (i + j);
                            if dangervalue >= 10 {
                                dangervalue = dangervalue - 9;
                            }
                            Node {
                                visited: false,
                                danger: dangervalue,
                                value: usize::MAX,
                                active: false,
                            }
                        })
                        .collect()
                })
                .collect();
            for k in 0..rows.len() {
                for node in &rows[k] {
                    final_rows[k].push(*node);
                }
            }
        }

        for y in 0..final_rows.len() {
            if i == 0 {
                for node in &final_rows[y] {
                    nodes[y].push(*node)
                }
            } else {
                nodes.push(final_rows[y].clone())
            }
        }
    }
    // Use Djikstra's algorithm
    let part1 = djikstra(&mut graph, width, height);
    println!("Part 1: {}", part1);
    let part2 = djikstra(&mut nodes, width * 5, height * 5);
    println!("Part 2: {}", part2);
}

fn djikstra(nodes: &mut Vec<Vec<Node>>, width: usize, height: usize) -> usize {
    let last = (nodes[0].len() - 1, nodes.len() - 1);
    nodes[0][0].active = true;
    nodes[0][0].value = 0;
    let initial_neighbors = get_neighbors((0, 0), width, height);
    let stack_init = [vec![(0, 0)], initial_neighbors].concat();
    let mut stack: VecDeque<(usize, usize)> = VecDeque::from(stack_init);
    while !nodes[last.1][last.0].visited {
        // Get current visited node and set to visited
        let (x, y) = stack.pop_front().unwrap();
        if (x, y) == last {
            break;
        }
        nodes[y][x].visited = true;
        let value = nodes[y][x].value;
        // activate surrounding nodes and push to stack
        let neighbors = get_neighbors((x, y), width, height);
        for (nx, ny) in neighbors {
            if nodes[ny][nx].visited {
                continue;
            }
            if !nodes[ny][nx].active {
                nodes[ny][nx].active = true;
                stack.push_back((nx, ny));
                nodes[ny][nx].value = value + nodes[ny][nx].danger;
                continue;
            }
            // set sum value for surrounding nodes
            if nodes[ny][nx].value > value + nodes[ny][nx].danger {
                nodes[ny][nx].value = value + nodes[ny][nx].danger;
            }
        }
    }
    println!("left: {}", nodes[last.1][last.0 - 1].value);
    println!("top: {}", nodes[last.1 - 1][last.0].value);
    println!("last: {}", nodes[last.1][last.0].danger);
    nodes[last.1][last.0].value
}

fn get_neighbors(point: (usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
    let (x, y) = point;
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    if x != 0 {
        neighbors.push((x - 1, y));
    }
    if y != 0 {
        neighbors.push((x, y - 1));
    }
    if x < width - 1 {
        neighbors.push((x + 1, y));
    }
    if y < height - 1 {
        neighbors.push((x, y + 1));
    }

    neighbors
}
