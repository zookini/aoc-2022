fn main() {
    println!("Part 1: {}", run(2));
    println!("Part 2: {}", run(10));
}

fn run(size: usize) -> usize {
    let mut rope = vec![(0isize, 0isize); size];
    let mut visited = std::collections::HashSet::from([rope[rope.len() - 1]]);

    for line in include_str!("../../input/09.txt").lines() {
        let (dir, n) = line.split_once(' ').unwrap();

        for _ in 0..n.parse().unwrap() {
            match dir {
                "R" => rope[0].0 += 1,
                "U" => rope[0].1 -= 1,
                "L" => rope[0].0 -= 1,
                "D" => rope[0].1 += 1,
                _ => unreachable!()
            }

            for i in 0..rope.len() - 1 {
                if (rope[i + 1].0 - rope[i].0).abs() > 1 || (rope[i + 1].1 - rope[i].1).abs() > 1 {
                    rope[i + 1].0 += (rope[i].0 - rope[i + 1].0).signum();
                    rope[i + 1].1 += (rope[i].1 - rope[i + 1].1).signum();
                }
            }

            visited.insert(rope[rope.len() - 1]);
        }
    }

    visited.len()
}
