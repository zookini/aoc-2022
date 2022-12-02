fn main() {
    println!("Part 1: {}", score(|_, b| b));
    println!("Part 2: {}", score(|a, b| (a + b - 1).rem_euclid(3)));
}

fn score(f: fn(i32, i32) -> i32) -> i32 {
    include_str!("../../input/02.txt")
        .lines()
        .map(|s| ((s.as_bytes()[0] - b'A') as i32, (s.as_bytes()[2] - b'X') as i32))
        .map(|(a, b)| (a, f(a, b)))
        .map(|(a, b)| b + 1 + (b - a + 1).rem_euclid(3) * 3)
        .sum()
}
