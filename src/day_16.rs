use std::collections::{BTreeMap, HashMap};

pub fn solve_part1(input: &str) -> i32 {
    let valves = parse_input(input);
    let mut cache = HashMap::new();
    let remain = 30;

    let init_snapshot = Snapshot {
        pos: 0, // AA -> 0
        opened: 0,
        remain,
    };

    max_pressure(init_snapshot, &valves, &mut cache)
}

fn max_pressure(snapshot: Snapshot, valves: &[Valve], cache: &mut HashMap<Snapshot, i32>) -> i32 {
    // consume time and increase pressure
    if snapshot.remain == 0 {
        return 0;
    }

    if let Some(pressure) = cache.get(&snapshot) {
        return *pressure;
    }

    let mut flow_rate = 0;
    for idx in 0..valves.len() {
        if is_set(snapshot.opened, idx as i32) {
            flow_rate += valves[idx].flow_rate;
        }
    }

    // next
    // a. open current valve if not opened
    let mut next_pressure = 0;
    if !is_set(snapshot.opened, snapshot.pos) && valves[snapshot.pos as usize].flow_rate > 0 {
        let mut new_snapshot = snapshot.clone();
        set(&mut new_snapshot.opened, snapshot.pos);
        new_snapshot.remain -= 1;

        let pressure = max_pressure(new_snapshot, valves, cache);
        next_pressure = pressure.max(next_pressure);
    }

    // b. move to next valve
    for next in &valves[snapshot.pos as usize].tunnels {
        let mut new_snapshot = snapshot.clone();
        new_snapshot.pos = *next;
        new_snapshot.remain -= 1;

        let pressure = max_pressure(new_snapshot, valves, cache);
        next_pressure = pressure.max(next_pressure);
    }

    let pressure = flow_rate + next_pressure;
    cache.insert(snapshot, pressure);
    pressure
}

pub fn solve_part2(input: &str) -> i32 {
    let valves = parse_input(input);
    let mut cache = BTreeMap::new();
    let remain = 26;

    let init_snapshot = Snapshot2 {
        pos: 0,
        e_pos: 0,
        remain,
        opened: 0,
    };

    let mut cur_max = 0;
    let max_pressure_per_min = valves.iter().map(|v| v.flow_rate).sum::<i32>();

    max_pressure2(
        0,
        init_snapshot,
        &valves,
        &mut cache,
        &mut cur_max,
        max_pressure_per_min,
    )
}

fn max_pressure2(
    prev_pressure: i32,
    snapshot: Snapshot2,
    valves: &[Valve],
    cache: &mut BTreeMap<Snapshot2, i32>,
    cur_max: &mut i32,
    max_pressure_per_min: i32,
) -> i32 {
    // consume time and increase pressure
    if snapshot.remain == 0 {
        if prev_pressure > *cur_max {
            *cur_max = prev_pressure;
        }
        return 0;
    }

    if let Some(pressure) = cache.get(&snapshot) {
        return *pressure;
    }

    let mut flow_rate = 0;
    for idx in 0..valves.len() {
        if is_set(snapshot.opened, idx as i32) {
            flow_rate += valves[idx].flow_rate;
        }
    }

    // early return
    if prev_pressure + flow_rate + max_pressure_per_min * (snapshot.remain - 1) < *cur_max {
        return 0;
    }

    // next
    // a. open current valve if not opened
    let mut next_pressure = 0;
    if !is_set(snapshot.opened, snapshot.pos) && valves[snapshot.pos as usize].flow_rate > 0 {
        let mut snapshot = snapshot.clone();
        set(&mut snapshot.opened, snapshot.pos);
        snapshot.remain -= 1;

        // open elephant valve
        if !is_set(snapshot.opened, snapshot.e_pos) && valves[snapshot.e_pos as usize].flow_rate > 0
        {
            let mut snapshot = snapshot.clone();
            set(&mut snapshot.opened, snapshot.e_pos);

            next_pressure = max_pressure2(
                prev_pressure + flow_rate,
                snapshot,
                valves,
                cache,
                cur_max,
                max_pressure_per_min,
            );
        }

        // move elephant
        for next in &valves[snapshot.e_pos as usize].tunnels {
            let mut snapshot = snapshot.clone();
            snapshot.e_pos = *next;

            let pressure = max_pressure2(
                prev_pressure + flow_rate,
                snapshot,
                valves,
                cache,
                cur_max,
                max_pressure_per_min,
            );
            next_pressure = next_pressure.max(pressure);
        }
    }

    // b. move to next valve
    for next in &valves[snapshot.pos as usize].tunnels {
        let mut snapshot = snapshot.clone();
        snapshot.pos = *next;
        snapshot.remain -= 1;

        // open elephant valve
        if !is_set(snapshot.opened, snapshot.e_pos) && valves[snapshot.e_pos as usize].flow_rate > 0
        {
            let mut snapshot = snapshot.clone();
            set(&mut snapshot.opened, snapshot.e_pos);

            let pressure = max_pressure2(
                prev_pressure + flow_rate,
                snapshot,
                valves,
                cache,
                cur_max,
                max_pressure_per_min,
            );
            next_pressure = next_pressure.max(pressure);
        }

        // move elephant
        for next in &valves[snapshot.e_pos as usize].tunnels {
            let mut snapshot = snapshot.clone();
            snapshot.e_pos = *next;

            let pressure = max_pressure2(
                prev_pressure + flow_rate,
                snapshot,
                valves,
                cache,
                cur_max,
                max_pressure_per_min,
            );
            next_pressure = next_pressure.max(pressure);
        }
    }

    let pressure = flow_rate + next_pressure;
    cache.insert(snapshot, pressure);
    pressure
}

fn parse_input(input: &str) -> Vec<Valve> {
    struct RawValve {
        name: String,
        flow_rate: i32,
        tunnels: Vec<String>,
    }

    let mut raw_valves = HashMap::new();

    for line in input.lines() {
        let line = line.trim();
        let name_start = line.find("Valve ").unwrap() + 6;
        let name_end = line.find(" has ").unwrap();
        let name = line[name_start..name_end].to_string();

        let flow_rate_start = line.find("flow rate=").unwrap() + 10;
        let flow_rate_end = line.find(";").unwrap();
        let flow_rate = line[flow_rate_start..flow_rate_end].parse::<i32>().unwrap();

        let mut tunnels = Vec::new();
        let tunnel_start = if let Some(pos) = line.find("tunnels lead to valves ") {
            pos + 23
        } else {
            line.find("tunnel leads to valve ").unwrap() + 22
        };
        let tunnel_str = line[tunnel_start..].to_string();
        for tunnel in tunnel_str.split(", ") {
            tunnels.push(tunnel.to_string());
        }

        let valve = RawValve {
            name,
            flow_rate,
            tunnels,
        };

        raw_valves.insert(valve.name.clone(), valve);
    }

    // name -> index
    let mut name_to_index = HashMap::new();
    name_to_index.insert("AA".to_string(), 0);

    for (v, _) in &raw_valves {
        if v == "AA" {
            continue;
        }

        let index = name_to_index.len();
        name_to_index.insert(v.clone(), index);
    }

    let mut valves: Vec<Option<Valve>> = vec![None; name_to_index.len()];
    valves.resize(name_to_index.len(), None);

    for (_, raw) in raw_valves {
        let index = name_to_index[&raw.name] as i32;
        let mut tunnels = Vec::new();
        for tunnel in raw.tunnels {
            tunnels.push(name_to_index[&tunnel] as i32);
        }

        let valve = Valve {
            name: index,
            flow_rate: raw.flow_rate,
            tunnels,
        };

        valves[index as usize] = Some(valve);
    }

    valves.into_iter().map(|v| v.unwrap()).collect()
}

#[derive(Clone)]
struct Valve {
    name: i32,
    flow_rate: i32,
    tunnels: Vec<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
struct Snapshot {
    pos: i32,
    remain: i32,
    opened: u64,
}

fn is_set(value: u64, pos: i32) -> bool {
    (value & (1 << pos)) != 0
}

fn set(value: &mut u64, pos: i32) {
    *value = *value | (1 << pos)
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
struct Snapshot2 {
    pos: i32,
    e_pos: i32,
    remain: i32,
    opened: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;

    #[test]
    fn test_parse_sample() {
        let valves = parse_input(SAMPLE_INPUT);
        assert_eq!(valves.len(), 10);
        assert_eq!(valves[0].flow_rate, 0);
    }

    #[test]
    fn test_part1_sample() {
        let answer = solve_part1(SAMPLE_INPUT);
        assert_eq!(answer, 1651);
    }

    #[test]
    #[ignore]
    fn test_part1() {
        let input = include_str!("../input/day_16.txt");
        let answer = solve_part1(input);
        assert_eq!(answer, 1474);
    }

    #[test]
    fn test_part2_sample() {
        let answer = solve_part2(SAMPLE_INPUT);
        assert_eq!(answer, 1707);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let input = include_str!("../input/day_16.txt");
        let answer = solve_part2(input);
        assert_eq!(answer, 2100);
    }
}
