fn main() {
    println!("Part 1: {}", run(20, 3));
    println!("Part 2: {}", run(10000, 1));
}

fn run(rounds: usize, div: usize) -> usize {
    let mut ms: Vec<_> = regex::Regex::new(r"(?s)s: (\d+(?:, \d+)*).*?old (.) (\w+).*?(\d+).*?(\d+).*?(\d+)")
        .unwrap()
        .captures_iter(include_str!("../../input/11.txt"))
        .map(|cp| (
            cp[1].split(", ").flat_map(|n| n.parse()).collect::<std::collections::VecDeque<_>>(),
            cp[2].chars().next().unwrap(),
            (&cp[3] != "old").then(|| cp[3].parse::<usize>().unwrap()),
            cp[4].parse::<usize>().unwrap(),
            cp[5].parse::<usize>().unwrap(),
            cp[6].parse::<usize>().unwrap(),
            0
        ))
        .collect();

    let md: usize = ms.iter().map(|m| m.3).product();

    for _ in 0..rounds {
        for i in 0..ms.len() {
            ms[i].6 += ms[i].0.len();

            while let Some(item) = ms[i].0.pop_front() {
                let other = ms[i].2.unwrap_or(item);
                let worry = if ms[i].1 == '*' { item * other } else { item + other } % md / div;
                let recipient = if worry % ms[i].3 == 0 { ms[i].4 } else { ms[i].5 };

                ms[recipient].0.push_back(worry);
            }
        }
    }

    ms.sort_by(|a, b| b.6.cmp(&a.6));
    ms[0].6 * ms[1].6
}
