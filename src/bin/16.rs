use rustc_hash::FxHashMap;
use itertools::Itertools;

type Tunnels = FxHashMap<u64, (u32, Vec<u64>)>;

fn main() {
    let tunnels: Vec<_> = regex::Regex::new(r"Valve (\w+).*?rate=(\d+).*?valves? ([^\r\n]+)")
        .unwrap()
        .captures_iter(include_str!("../../input/16.txt"))
        .flat_map(|cp| Some((cp.get(1)?.as_str(), (cp[2].parse().ok()?, cp.get(3)?.as_str().split(", ")))))
        .collect();
    
    let map: FxHashMap<_, _> = tunnels.iter().enumerate().map(|(i, &(k, _))| (k, 1 << i)).collect();

    let tunnels = tunnels
        .into_iter()
        .map(|(tunnel, (rate, leads))| (map[tunnel], (rate, leads.map(|lead| map[lead]).collect())))
        .collect();

    let mut cache = FxHashMap::default();
    visit(&mut cache, &tunnels, map["AA"], 0, 0, 30);

    println!("Part 1: {}", cache.values().max().unwrap());

    cache.clear();
    visit(&mut cache, &tunnels, map["AA"], 0, 0, 26);

    let p2 = cache
        .iter()
        .map(|(&(_, opened, _), &flow)| (opened, flow))
        .into_grouping_map()
        .max()
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| (a.0 & b.0 == 0).then(|| a.1 + b.1))
        .max()
        .unwrap();

    println!("Part 2: {}", p2);
}

fn visit(cache: &mut FxHashMap<(u64, u64, u8), u32>, tunnels: &Tunnels, tunnel: u64, opened: u64, flow: u32, time: u8) {
    if time > 0 && cache.get(&(tunnel, opened, time)).filter(|&&hit| flow <= hit).is_none() {
        cache.insert((tunnel, opened, time), flow);

        let (rate, leads) = &tunnels[&tunnel];

        let mut visit_leads = |opened, flow, time: u8| for &lead in leads {
            visit(cache, tunnels, lead, opened, flow, time.saturating_sub(1))
        };
        
        visit_leads(opened, flow, time);

        if *rate > 0 && opened & tunnel == 0 {
            visit_leads(opened | tunnel, flow + *rate * (time as u32 - 1), time - 1);
        }
    }
}
