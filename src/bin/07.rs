fn main() {
    let sizes = walk(&mut include_str!("../../input/07.txt").lines());
    let required = 30000000 - (70000000 - sizes.last().unwrap());

    println!("Part 1: {}", sizes.iter().filter(|&&size| size <= 100000).sum::<usize>());
    println!("Part 2: {}", sizes.iter().filter(|&&size| size >= required).min().unwrap());
}

fn walk<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Vec<usize> {
    let (mut total, mut dirs) = (0, vec![]);

    while let Some(line) = lines.next().filter(|&line| line != "$ cd ..") {
        if !["$ cd /", "dir", "$ ls"].iter().any(|s| line.starts_with(s)) {
            if let Ok(size) = line.split(' ').next().unwrap().parse::<usize>() {
                total += size
            } else {
                dirs.extend(walk(lines));
                total += dirs.last().unwrap();
            }
        }
    }

    dirs.push(total);
    dirs
}
