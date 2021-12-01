fn main() {
    let lines: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|v| v.parse().unwrap())
        .collect();
    let mut increase = 0;
    for i in 0..(lines.len() - 1) {
        if lines[i] < lines[i + 1] {
            increase += 1;
        }
    }
    println!("Part 1: {}", increase);

    let mut increase2 = 0;
    for i in 0..(lines.len() - 3) {
        let current_win = lines[i] + lines[i + 1] + lines[i + 2];
        let next_win = current_win + lines[i + 3] - lines[i];
        if next_win > current_win {
            increase2 += 1;
        }
    }
    println!("Part 2: {}", increase2);
}
