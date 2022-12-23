use std::cmp::Ordering;
use Packet::*;

#[derive(Eq, PartialEq)]
enum Packet {
    L(Vec<Packet>),
    N(isize),
}


impl Packet {
    fn parse(s: &str) -> Self {
        Self::_parse(&mut regex::Regex::new(r"\[|]|\d+").unwrap().find_iter(s).map(|s| s.as_str()).peekable())
    }

    fn _parse<'a, T: Iterator<Item = &'a str>>(tokens: &mut std::iter::Peekable<T>) -> Self {
        match tokens.next() {
            Some("[") => {
                let mut packets = vec![];

                loop {
                    match tokens.peek() {
                        Some(&"]") => { tokens.next(); return Self::L(packets) },
                        Some(_) => packets.push(Self::_parse(tokens)),
                        None => unreachable!()
                    }
                }
            },
            Some(s) => Self::N(s.parse().unwrap()),
            None => unreachable!()
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (N(a), N(b)) => a.cmp(b),
            (N(a), b @ L(_)) => Self::L(vec![N(*a)]).cmp(b),
            (a @ L(_), N(b)) => a.cmp(&Self::L(vec![N(*b)])),
            (L(a), L(b)) => a
                .iter()
                .zip(b)
                .map(|(a, b)| a.cmp(b))
                .find(|&c| c != Ordering::Equal)
                .unwrap_or(a.len().cmp(&b.len()))
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut packets: Vec<_> = include_str!("../../input/13.txt")
        .lines()
        .filter_map(|line| (!line.is_empty()).then(|| Packet::parse(line)))
        .collect();

    let p1: usize = (1..)
        .zip(packets.chunks(2))
        .filter_map(|(i, c)| (c[0].cmp(&c[1]) != Ordering::Greater).then(|| i))
        .sum();

    println!("Part 1: {}", p1);

    packets.extend(["[[2]]", "[[6]]"].into_iter().map(Packet::parse));
    packets.sort();

    let p2: usize = (1..)
        .zip(packets)
        .filter_map(|(i, p)| [2, 6].iter().any(|&n| N(n).cmp(&p) == Ordering::Equal).then(|| i))
        .product();

    println!("Part 2: {}", p2);
}
