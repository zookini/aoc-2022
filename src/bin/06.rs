fn main() {
    let scan = |size| size + include_bytes!("../../input/06.txt")
        .windows(size)
        .position(|w| w.iter().fold(0u32, |c, b| c | 1 << b - b'a').count_ones() as usize == size)
        .unwrap();

    println!("Part 1: {}", scan(4));
    println!("Part 2: {}", scan(14));
}
