use itertools::Itertools;

fn main() {
    println!("Part 1: {}", run(|src, dst, n| dst.extend(src.drain(src.len() - n..).rev())));
    println!("Part 2: {}", run(|src, dst, n| dst.extend(src.drain(src.len() - n..))));
}

fn run(transfer: fn(&mut Vec<char>, &mut Vec<char>, usize)) -> String {
    let (diagram, commands) = include_str!("../../input/05.txt").split_once("\r\n\r\n").unwrap();
    let mut diagram = diagram.lines().peekable();
    let mut stacks = vec![vec![]; diagram.peek().unwrap().len() / 3];
    let mut src = vec![];

    for row in diagram.take_while(|s| !s.starts_with(" 1")) {
        for (i, c) in row.chars().skip(1).step_by(4).enumerate().filter(|&(_, c)| c != ' ') {
            stacks[i].insert(0, c);
        }
    }

    for command in commands.lines() {
        let (amount, from, to) = command
            .split(|c: char| !c.is_numeric())
            .filter_map(|n| n.parse().ok())
            .next_tuple()
            .unwrap();

        std::mem::swap(&mut src, &mut stacks[from - 1]);
        transfer(&mut src, &mut stacks[to - 1], amount);
        std::mem::swap(&mut src, &mut stacks[from - 1]);
    }

    stacks.iter().filter_map(|stack| stack.last()).collect()
}
