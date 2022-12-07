struct Dir { size: usize, entries: Vec<Dir> }

impl Dir {
    fn new<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Dir {
        let mut dir = Dir { size: 0, entries: vec![] };

        while let Some(line) = lines.next().filter(|&line| line != "$ cd ..") {
            if !["$ cd /", "dir", "$ ls"].iter().any(|s| line.starts_with(s)) {
                if let Ok(size) = line.split(' ').next().unwrap().parse::<usize>() {
                    dir.size += size;
                } else {
                    dir.entries.push(Self::new(lines));
                    dir.size += dir.entries.last().unwrap().size;
                }
            }
        }

        dir
    }

    fn recurse(&self) -> Box<dyn Iterator<Item = &Self> + '_> {
        Box::new(self.entries.iter().flat_map(Self::recurse).chain([self]))
    }
}

fn main() {
    let root = Dir::new(&mut include_str!("../../input/07.txt").lines());
    let sizes: Vec<_> = root.recurse().map(|d| d.size).collect();
    let required = 30000000 - (70000000 - root.size);

    println!("Part 1: {}", sizes.iter().filter(|&&size| size <= 100000).sum::<usize>());
    println!("Part 2: {}", sizes.iter().filter(|&&size| size >= required).min().unwrap());
}
