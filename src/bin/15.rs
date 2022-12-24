use itertools::Itertools;

fn main() {
    let zone: Vec<_> = include_str!("../../input/15.txt")
        .split(|c: char| c != '-' && !c.is_numeric())
        .filter_map(|s| s.parse::<i64>().ok())
        .tuples()
        .map(|(sx, sy, bx, by)| ((sx, sy), (bx, by)))
        .collect();

    let ranges = |row| zone.iter().filter_map(move |&(s, b)| range(s, b, row));

    let row = 2_000_000;
    let (start, end) = merge(ranges(row)).unwrap();
    let beacons = zone.iter().filter_map(|&(_, (bx, by))| (by == row).then(|| bx)).unique();

    println!("Part 1: {}", 1 + end - start - beacons.filter(|b| (start..=end).contains(b)).count() as i64);

    let end = 4_000_000;

    let p2 = (0..end)
        .find_map(|y| merge(ranges(y).map(|(s, e)| (s.max(0), e.min(end)))).err().map(|t| (t + 1) * end + y))
        .unwrap();

    println!("Part 2: {:?}", p2);
}

fn merge(ranges: impl Iterator<Item = (i64, i64)>) -> Result<(i64, i64), i64> {
    let ranges: Vec<_> = ranges.sorted().collect();
    ranges[1..].iter().try_fold(ranges[0], |(ms, me), &(rs, re)| if me < rs { Err(me) } else { Ok((ms, me.max(re))) })
}

fn range((sx, sy): (i64, i64), (bx, by): (i64, i64), row: i64) -> Option<(i64, i64)> {
    let distance = (sx - bx).abs() + (sy - by).abs() - (sy - row).abs();
    (distance >= 0).then(|| (sx - distance, sx + distance))
}
