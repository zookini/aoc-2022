fn main() {
    let (mut x, mut cycle, mut xs) = (1, 1isize, vec![]);
    let mut pending;

    for line in include_str!("../../input/10.txt").lines() {
        pending = if line == "noop" { (cycle + 1, 0) } else { (cycle + 2, line[5..].parse::<isize>().unwrap()) };

        while cycle < pending.0 {
            xs.push(x);
            cycle += 1;
        }

        x += pending.1;
    }

    println!("Part 1: {}", (1..).zip(&xs).skip(19).step_by(40).take(6).map(|(a, b)| a * b).sum::<isize>());
    println!("Part 2:");

    for xx in xs.chunks(40) {
        let row = (1..).zip(xx).map(|(i, x)| if (0..3).contains(&(i - x)) { '#' } else { ' ' });
        println!("{}", row.collect::<String>());
    }

}
