use std::collections::{BTreeMap, HashMap, HashSet};

pub fn solve_part1(input: &str) -> i32 {
    let valves = parse_input(input);
    let mut cache = BTreeMap::new();
    let remain = 30;

    let init_snapshot = Snapshot {
        pos: "AA".to_string(),
        opened: HashSet::new(),
        remain,
    };

    max_pressure(init_snapshot, &valves, &mut cache)
}

fn max_pressure(
    snapshot: Snapshot,
    valves: &HashMap<String, Valve>,
    cache: &mut BTreeMap<Snapshot, i32>,
) -> i32 {
    // consume time and increase pressure
    if snapshot.remain == 0 {
        return 0;
    }

    if let Some(pressure) = cache.get(&snapshot) {
        return *pressure;
    }

    let mut flow_rate = 0;
    for open in &snapshot.opened {
        flow_rate += valves[open].flow_rate;
    }

    // next
    // a. open current valve if not opened
    let mut next_pressure = 0;
    if !snapshot.opened.contains(&snapshot.pos) && valves[&snapshot.pos].flow_rate > 0 {
        let mut new_snapshot = snapshot.clone();
        new_snapshot.opened.insert(snapshot.pos.clone());
        new_snapshot.remain -= 1;

        let pressure = max_pressure(new_snapshot, valves, cache);
        next_pressure = pressure.max(next_pressure);
    }

    // b. move to next valve
    for next in &valves[&snapshot.pos].tunnels {
        let mut new_snapshot = snapshot.clone();
        new_snapshot.pos = next.clone();
        new_snapshot.remain -= 1;

        let pressure = max_pressure(new_snapshot, valves, cache);
        next_pressure = pressure.max(next_pressure);
    }

    let pressure = flow_rate + next_pressure;
    cache.insert(snapshot, pressure);
    pressure
}

pub fn solve_part2(input: &str) -> i64 {
    0
}

fn parse_input(input: &str) -> HashMap<String, Valve> {
    let mut valves = HashMap::new();

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

        let valve = Valve {
            name,
            flow_rate,
            tunnels,
        };

        valves.insert(valve.name.clone(), valve);
    }

    valves
}

struct Valve {
    name: String,
    flow_rate: i32,
    tunnels: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Snapshot {
    pos: String,
    opened: HashSet<String>,
    remain: i32,
}

impl Ord for Snapshot {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord1 = self.pos.cmp(&other.pos);
        if ord1 == std::cmp::Ordering::Equal {
            let ord2 = self.remain.cmp(&other.remain);
            if ord2 == std::cmp::Ordering::Equal {
                self.opened.iter().cmp(other.opened.iter())
            } else {
                ord2
            }
        } else {
            ord1
        }
    }
}

impl PartialOrd for Snapshot {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
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
        assert_eq!(valves["AA"].flow_rate, 0);
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
        assert_eq!(answer, 56000011);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let input = include_str!("../input/day_16.txt");
        let answer = solve_part2(input);
        assert_eq!(answer, 28145);
    }
}
