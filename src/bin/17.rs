fn main() {
    println!("Part 1: {}", run(|_, _, _, falls, top| (falls == 2022).then(|| top)));

    let mut cache = rustc_hash::FxHashMap::default();

    let p2 = run(|chamber, rock, jet, falls, top| cache
        .insert((chamber[top.saturating_sub(20)..=top].to_vec(), rock, jet), (falls, top))
        .map(|(falls0, top0)| (1000000000000 - falls0, falls - falls0, top0))
        .filter(|(offset, step, _)| (offset % step == 0))
        .map(|(offset, step, top0)| offset / step * (top - top0) + top0)
    );

    println!("Part 2: {}", p2);
}

fn run(mut f: impl FnMut(&[u8], u32, u8, usize, usize) -> Option<usize>) -> usize {
    let mut jets = include_bytes!("../../input/17.txt").iter().cycle();
    let mut chamber = vec![u8::MAX];

    for (falls, mut rock) in [0x1eu32, 0x81c08, 0x4041c, 0x10101010, 0x1818].into_iter().cycle().enumerate() {
        let top = chamber.iter().rposition(|&row| row != 0).unwrap();
        chamber.resize(top + 8, 0);

        for i in (0..top + 5).rev() {
            let jet = *jets.next().unwrap();
            let surface = u32::from_le_bytes(chamber[i..i + 4].try_into().unwrap());
            let collisions = surface | 0x80808080;

            if jet == b'<' {
                rock <<= (collisions & rock.rotate_left(1) == 0) as u8;
            } else {
                rock >>= (collisions & rock.rotate_right(1) == 0) as u8;
            }
            
            if rock & u32::from_le_bytes(chamber[i - 1..i + 3].try_into().unwrap()) > 0 {
                if let Some(n) = f(&chamber, rock, jet, falls, top) {
                    return n;
                } else {
                    chamber[i..i + 4].copy_from_slice(&(rock | surface).to_le_bytes());
                    break;
                }
            }
        }
    }

    unreachable!()
}
