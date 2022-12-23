use itertools::Itertools;

type Cave = std::collections::HashSet<Point>;
type Point = (i16, i16);

fn main() {
    let mut cave: Cave = include_str!("../../input/14.txt")
        .lines()
        .flat_map(|line| line
            .split(" -> ")
            .map(|s| s.split(',').map(|n| n.parse().unwrap()).collect_tuple().unwrap())
            .tuple_windows()
        )
        .flat_map(|(start, end)| draw(start, end))
        .collect();

    let bottom = *cave.iter().map(|(_, y)| y).max().unwrap();
    println!("Part 1: {}", run(cave.clone(), bottom));

    let bottom = bottom + 2;
    cave.extend(draw((500 - bottom, bottom), (500 + bottom, bottom)));
    println!("Part 2: {}", run(cave, bottom));
}

fn run(mut cave: Cave, bottom: i16) -> usize {
    (0..).find(|_| drop(&mut cave, bottom)).unwrap()
}

fn drop(cave: &mut Cave, bottom: i16) -> bool {
    let (mut x, mut y) = (500, 0);

    loop {
        if y > bottom || cave.contains(&(500, 0)) {
            return true;
        } else if let Some(p) = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)].into_iter().find(|p| !cave.contains(p)) {
            (x, y) = p;
        }
        else {
            cave.insert((x, y));
            return false;    
        }
    }
}

fn draw((x1, y1): Point, (x2, y2): Point) -> impl Iterator<Item = Point> {
    let (dx, dy) = ((x2 - x1).signum(), (y2 - y1).signum());
    std::iter::successors(Some((x1, y1)), move |&(x, y)| (x != x2 || y != y2).then(|| (x + dx, y + dy)))
}
