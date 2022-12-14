fn main() {
    let mut map: Vec<Vec<_>> = include_str!("../../input/12.txt").lines().map(|s| s.bytes().collect()).collect();
    let (start, end) = (locate(&map, b'S').next().unwrap(), locate(&map, b'E').next().unwrap());

    map[start.1][start.0] = b'a';
    map[end.1][end.0] = b'z';

    println!("Part 1: {}", steps(&map, vec![start], end));
    println!("Part 2: {}", steps(&map, locate(&map, b'a').collect(), end));
}

fn locate<'a>(map: &'a Vec<Vec<u8>>, b: u8) -> impl Iterator<Item = (usize, usize)> + 'a {
    itertools::iproduct!(0..map[0].len(), 0..map.len()).filter(move |&(x, y)| map[y][x] == b)
}

fn steps(map: &Vec<Vec<u8>>, mut start: Vec<(usize, usize)>, end: (usize, usize)) -> usize {
    let mut seen = std::collections::HashSet::new();

    for step in 0.. {
        let mut next = vec![];

        for (x, y) in start {
            if seen.insert((x, y)) {
                if (x, y) == end {
                    return step;
                }         

                for (u, v) in [(x.wrapping_sub(1), y), (x + 1, y), (x, y.wrapping_sub(1)), (x, y + 1)] {
                    if u < map[0].len() && v < map.len() && map[v][u] <= map[y][x] + 1 {
                        next.push((u, v));
                    }
                }
            }
        }

        start = next;
    }

    unreachable!()
}
