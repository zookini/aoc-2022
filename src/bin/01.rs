use itertools::Itertools;

fn main() {
    let food: Vec<u32> = include_str!("../../input/01.txt")
        .split("\r\n\r\n")
        .map(|elf| elf.lines().map(|line| line.parse::<u32>().unwrap()).sum::<u32>())
        .sorted_by(|a, b| b.cmp(a))
        .collect();

    println!("Part 1: {}", food[0]);
    println!("Part 2: {}", food.iter().take(3).sum::<u32>());
}
