use std::collections::HashSet;

pub fn parse_input(input: &str) -> Vec<(i32, i32, i32, i32)> {
    let mut parsed = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        let x_start = line.find("x=").unwrap() + 2;
        let x_end = line.find(",").unwrap();
        let sx = line[x_start..x_end].parse::<i32>().unwrap();

        let y_start = line.find("y=").unwrap() + 2;
        let y_end = line.find(":").unwrap();
        let sy = line[y_start..y_end].parse::<i32>().unwrap();

        let x_start = line.rfind("x=").unwrap() + 2;
        let x_end = line.rfind(",").unwrap();
        let bx = line[x_start..x_end].parse::<i32>().unwrap();

        let y_start = line.rfind("y=").unwrap() + 2;
        let by = line[y_start..].parse::<i32>().unwrap();

        parsed.push((sx, sy, bx, by));
    }

    parsed
}

pub fn solve_part1(input: &str, row: i32) -> i32 {
    let parsed = parse_input(input);
    impossible_beacon_count_at_row(&parsed, row)
}

pub fn solve_part2(input: &str) -> i32 {
    let parsed = parse_input(input);
    0
}

fn impossible_beacon_count_at_row(input: &[(i32, i32, i32, i32)], row: i32) -> i32 {
    let mut impossibles = HashSet::<(i32, i32)>::new();

    for (sx, sy, bx, by) in input {
        impossibles.insert((*sx, *sy));
        impossibles.insert((*bx, *by));
    }

    let prev_count = impossibles.len();

    for (sx, sy, bx, by) in input {
        let len = (bx - sx).abs() + (by - sy).abs();

        if (*sy - row).abs() > len {
            continue;
        }

        let count = len - (*sy - row).abs();
        for i in -count..=count {
            impossibles.insert((sx + i, row));
        }
    }

    (impossibles.len() - prev_count) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3 "#;

    #[test]
    fn test_parse_sample() {
        let parsed = parse_input(SAMPLE_INPUT);
        assert_eq!(parsed.len(), 14);

        assert_eq!(parsed[0], (2, 18, -2, 15));
        assert_eq!(parsed[13], (20, 1, 15, 3));
    }

    #[test]
    fn test_part1_sample() {
        let answer = solve_part1(SAMPLE_INPUT, 10);
        assert_eq!(answer, 26);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day_15.txt");
        let answer = solve_part1(input, 2000000);
        assert_eq!(answer, 5073496);
    }

    #[test]
    fn test_part2_sample() {
        let answer = solve_part2(SAMPLE_INPUT);
        assert_eq!(answer, 93);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/day_15.txt");
        let answer = solve_part2(input);
        assert_eq!(answer, 28145);
    }
}
