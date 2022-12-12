const INF: u32 = 1000000;
const MAX_DIST: u32 = 100000000;

pub fn solve_part1(input: &str) -> u32 {
    let (start, _start_candidates, end, mut map) = load_map(input);
    shortest_path(start, end, &mut map)
}

pub fn solve_part2(input: &str) -> u32 {
    let (_, start_candidates, end, mut map) = load_map(input);

    let mut shortest = None;
    for start in start_candidates {
        let cur = shortest_path(start, end, &mut map);
        if cur < shortest.unwrap_or(MAX_DIST) {
            shortest = Some(cur);
        }
    }
    shortest.unwrap()
}

fn next_node(visited: &Vec<bool>, distance: &Vec<u32>) -> Option<usize> {
    let mut min_dist = MAX_DIST;
    let mut min_idx = None;

    for i in 0..distance.len() {
        if visited[i] {
            continue;
        }

        if distance[i] < min_dist {
            min_dist = distance[i];
            min_idx = Some(i);
        }
    }

    min_idx
}

fn shortest_path(start: usize, end: usize, map: &mut Vec<Vec<u32>>) -> u32 {
    let mut visited = vec![false; map.len()];
    let mut distance = map[start].clone();

    visited[start] = true;

    for _ in 0..(distance.len() - 1) {
        let i = next_node(&visited, &distance).unwrap();
        visited[i] = true;

        for j in 0..distance.len() {
            if visited[j] {
                continue;
            }

            // if distance[i] == u32::MAX || map[i][j] == u32::MAX {
            //     continue;
            // }

            let new_dist = distance[i] + map[i][j];
            if distance[j] > new_dist {
                distance[j] = new_dist;
            }
        }
    }

    distance[end]
}

fn is_neighbor(src: usize, dst: usize, width: usize) -> bool {
    let src_xy = (src % width, src / width);
    let dst_xy = (dst % width, dst / width);
    let diff_x = i64::abs(src_xy.0 as i64 - dst_xy.0 as i64);
    let diff_y = i64::abs(src_xy.1 as i64 - dst_xy.1 as i64);
    diff_x + diff_y <= 1
}

fn can_go(src: usize, dst: usize, data: &Vec<Vec<u8>>) -> bool {
    if !is_neighbor(src, dst, data[0].len()) {
        return false;
    }

    let src_xy = (src % data[0].len(), src / data[0].len());
    let dst_xy = (dst % data[0].len(), dst / data[0].len());
    get_height(data[dst_xy.1][dst_xy.0]) as i64 - get_height(data[src_xy.1][src_xy.0]) as i64 <= 1
}

fn get_height(h: u8) -> u8 {
    match h {
        b'S' => b'a',
        b'E' => b'z',
        _ => h,
    }
}

fn load_data(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.as_bytes().iter().copied().collect())
        .collect()
}

fn load_map(input: &str) -> (usize, Vec<usize>, usize, Vec<Vec<u32>>) {
    let data = load_data(input);

    let mut start: Option<(usize, usize)> = None;
    'outer: for (y, line) in data.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == b'S' {
                start = Some((x, y));
                break 'outer;
            }
        }
    }
    let start = start.unwrap();
    let start = start.0 + start.1 * data[0].len();

    let mut start_candidates = vec![start];
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if data[i][j] == b'a' {
                start_candidates.push(j + i * data[0].len());
            }
        }
    }

    let mut end: Option<(usize, usize)> = None;
    'outer: for (y, line) in data.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == b'E' {
                end = Some((x, y));
                break 'outer;
            }
        }
    }
    let end = end.unwrap();
    let end = end.0 + end.1 * data[0].len();

    let mut map = vec![vec![INF; data.len() * data[0].len()]; data.len() * data[0].len()];
    for src in 0..map.len() {
        for dst in 0..map.len() {
            if src == dst {
                map[src][dst] = 0;
            } else if can_go(src, dst, &data) {
                map[src][dst] = 1;
            }
        }
    }

    (start, start_candidates, end, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

    #[test]
    fn test_load() {
        let (start, start_candidates, end, _map) = load_map(SAMPLE_INPUT);
        assert_eq!(start, 0);
        assert_eq!(start_candidates, vec![0, 1, 8, 16, 24, 32]);
        assert_eq!(end, 21);
    }

    #[test]
    fn test_load_data() {
        let data = load_data(SAMPLE_INPUT);
        assert_eq!(can_go(20, 21, &data), true);
    }

    #[test]
    #[ignore]
    fn test_part1() {
        let input = include_str!("../input/day_12.txt");
        assert_eq!(solve_part1(input), 456);
    }

    #[test]
    fn test_part1_sample() {
        let answer = solve_part1(SAMPLE_INPUT);
        assert_eq!(answer, 31);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let input = include_str!("../input/day_12.txt");
        assert_eq!(solve_part2(input), 0);
    }

    #[test]
    #[ignore]
    fn test_part2_sample() {
        let answer = solve_part2(SAMPLE_INPUT);
        assert_eq!(answer, 29);
    }
}
