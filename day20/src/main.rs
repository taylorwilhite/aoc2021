use std::str;
fn main() {
    let (algorithm, image) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let pixels: Vec<Vec<u8>> = image
        .lines()
        .map(|l| {
            l.bytes()
                .map(|b| match b {
                    b'#' => b'1',
                    b'.' => b'0',
                    _ => unreachable!(),
                })
                .collect::<Vec<u8>>()
        })
        .collect();
    let binary_alg: Vec<u8> = algorithm
        .bytes()
        .map(|b| match b {
            b'#' => b'1',
            b'.' => b'0',
            _ => unreachable!(),
        })
        .collect();
    let part1 = part_one(&binary_alg, &pixels);
    println!("Part 1: {}", part1);
    let part2 = part_two(&binary_alg, &pixels);
    println!("Part 2: {}", part2);
}

fn part_one(algorithm: &Vec<u8>, pixels: &Vec<Vec<u8>>) -> usize {
    let mut new_image = pixels.clone();
    let mut fill = b'0';
    for _ in 0..2 {
        let (ticked_image, new_fill) = tick_image(&new_image, algorithm, fill);
        new_image = ticked_image;
        fill = new_fill
    }

    return new_image
        .iter()
        .map(|l| l.iter().filter(|&b| *b == b'1').count())
        .fold(0, |acc, l| acc + l);
}

fn part_two(algorithm: &Vec<u8>, pixels: &Vec<Vec<u8>>) -> usize {
    let mut new_image = pixels.clone();
    let mut fill = b'0';
    for _ in 0..50 {
        let (ticked_image, new_fill) = tick_image(&new_image, algorithm, fill);
        new_image = ticked_image;
        fill = new_fill
    }

    return new_image
        .iter()
        .map(|l| l.iter().filter(|&b| *b == b'1').count())
        .fold(0, |acc, l| acc + l);
}

fn tick_image(pixels: &Vec<Vec<u8>>, algorithm: &Vec<u8>, fill: u8) -> (Vec<Vec<u8>>, u8) {
    let new_fill = match fill {
        b'0' => algorithm[0],
        b'1' => algorithm[0b111111111],
        _ => unreachable!(),
    };
    let padded_image = pad_image(pixels, fill);
    let mut new_image: Vec<Vec<u8>> = Vec::new();
    let height = padded_image.len() - 1;
    let width = padded_image[0].len() - 1;
    for y in 0..padded_image.len() {
        let mut new_slice = Vec::new();
        for x in 0..padded_image[y].len() {
            let toprow = match y {
                0 => vec![fill; 3],
                _ => {
                    if x == 0 {
                        vec![fill, padded_image[y - 1][x], padded_image[y - 1][x + 1]]
                    } else if x == width {
                        vec![padded_image[y - 1][x - 1], padded_image[y - 1][x], fill]
                    } else {
                        vec![
                            padded_image[y - 1][x - 1],
                            padded_image[y - 1][x],
                            padded_image[y - 1][x + 1],
                        ]
                    }
                }
            };
            let midrow = match x {
                0 => vec![fill, padded_image[y][x], padded_image[y][x + 1]],
                _ if x == width => vec![padded_image[y][x - 1], padded_image[y][x], fill],
                _ => vec![
                    padded_image[y][x - 1],
                    padded_image[y][x],
                    padded_image[y][x + 1],
                ],
            };
            let botrow = match y {
                _ if y == height => vec![fill; 3],
                _ => {
                    if x == 0 {
                        vec![fill, padded_image[y + 1][x], padded_image[y + 1][x + 1]]
                    } else if x == width {
                        vec![padded_image[y + 1][x - 1], padded_image[y + 1][x], fill]
                    } else {
                        vec![
                            padded_image[y + 1][x - 1],
                            padded_image[y + 1][x],
                            padded_image[y + 1][x + 1],
                        ]
                    }
                }
            };

            let number = [toprow, midrow, botrow].concat();

            let index = usize::from_str_radix(str::from_utf8(&number).unwrap(), 2).unwrap();
            new_slice.push(algorithm[index]);
        }
        new_image.push(new_slice);
    }
    return (new_image, new_fill);
}

fn pad_image(image: &Vec<Vec<u8>>, fill: u8) -> Vec<Vec<u8>> {
    let mut new_image = Vec::new();
    new_image.push(vec![fill; image[0].len() + 2]);
    for y in 0..image.len() {
        let mut new_row = vec![fill];
        for x in 0..image[y].len() {
            new_row.push(image[y][x]);
        }
        new_row.push(fill);
        new_image.push(new_row);
    }
    new_image.push(vec![fill; image[0].len() + 2]);
    new_image
}

#[test]
fn example_p1() {
    let (algorithm, image) = include_str!("../test.txt").split_once("\n\n").unwrap();
    let pixels: Vec<Vec<u8>> = image
        .lines()
        .map(|l| {
            l.bytes()
                .map(|b| match b {
                    b'#' => b'1',
                    b'.' => b'0',
                    _ => unreachable!(),
                })
                .collect::<Vec<u8>>()
        })
        .collect();
    let binary_alg: Vec<u8> = algorithm
        .bytes()
        .map(|b| match b {
            b'#' => b'1',
            b'.' => b'0',
            _ => unreachable!(),
        })
        .collect();
    let result = part_one(&binary_alg, &pixels);
    assert_eq!(result, 35)
}
