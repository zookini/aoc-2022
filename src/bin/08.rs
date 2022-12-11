use std::iter::successors;
use take_until::TakeUntilExt;

fn main() {
    let grid: Vec<_> = include_str!("../../input/08.txt").lines().map(str::as_bytes).collect();

    let p1 = itertools::iproduct!(0..grid[0].len(), 0..grid.len())
        .filter(|&(x, y)| steps((x, y), &grid).any(|mut d| d.all(|(u, v)| grid[v][u] < grid[y][x])))
        .count();

    println!("Part 1: {}", p1);

    let p2: usize = itertools::iproduct!(0..grid[0].len(), 0..grid.len())
        .map(|(x, y)| steps((x, y), &grid).map(|d| d.take_until(|&(u, v)| grid[v][u] >= grid[y][x]).count()).product())
        .max()
        .unwrap();

    println!("Part 2: {}", p2);
}

fn steps<'a>((x, y): (usize, usize), grid: &'a Vec<&'a [u8]>) -> impl Iterator<Item = impl Iterator<Item = (usize, usize)> + 'a> {
    [(usize::MAX, 0), (1, 0), (0, usize::MAX), (0, 1)]
        .iter()
        .map(move |&(dx, dy)| successors(Some((x, y)), move |(x, y)| Some((x.wrapping_add(dx), y.wrapping_add(dy))))
            .skip(1)
            .take_while(move |&(x, y)| x < grid[0].len() && y < grid.len())
        )
}
