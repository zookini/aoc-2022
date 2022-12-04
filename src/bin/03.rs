fn main() {
    let rucksacks: Vec<&str> = include_str!("../../input/03.txt").lines().collect();

    let p1: u32 = rucksacks
        .iter()
        .map(|s| s.split_at(s.len() / 2))
        .map(|(a, b)| priority(set(a) & set(b)))
        .sum();

    let p2: u32 = rucksacks
        .chunks(3)
        .map(|rs| priority(set(rs[0]) & set(rs[1]) & set(rs[2])))
        .sum();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn set(s: &str) -> u64 {
    s.bytes().fold(0, |set, b| set | 1 << (b - if b.is_ascii_lowercase() { b'a' } else { b'A' - 26 }))
}

fn priority(set: u64) -> u32 {
    set.trailing_zeros() + 1
}
