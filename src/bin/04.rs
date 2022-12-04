use itertools::Itertools;

fn main() {
    println!("Part 1: {}", count(|(a1, a2, b1, b2)| a1 <= b1 && b2 <= a2 || b1 <= a1 && a2 <= b2));
    println!("Part 2: {}", count(|(a1, a2, b1, b2)| a1 <= b2 && b1 <= a2));
}

fn count(p: fn(&(u32, u32, u32, u32)) -> bool) -> usize {
    include_str!("../../input/04.txt")
        .lines()
        .filter_map(|s| s.split(|c: char| !c.is_numeric()).map(|n| n.parse().unwrap()).next_tuple())
        .filter(p)
        .count()
}
