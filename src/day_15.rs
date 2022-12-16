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

pub fn solve_part2(input: &str, size: i32) -> i64 {
    let parsed = parse_input(input);
    find_empty_pos(&parsed, size)
}

fn find_empty_pos(input: &[(i32, i32, i32, i32)], size: i32) -> i64 {
    for &(sx, sy, bx, by) in input {
        let points = get_cover_points(sx, sy, bx, by);
        for (x, y) in points {
            if x < 0 || x > size || y < 0 || y > size {
                continue;
            }

            if is_empty_at_pos(input, x, y) {
                return (x as i64) * 4000000 + (y as i64);
            }
        }
    }
    0
}

fn is_empty_at_pos(input: &[(i32, i32, i32, i32)], x: i32, y: i32) -> bool {
    for &(sx, sy, bx, by) in input {
        if is_in_area(sx, sy, bx, by, x, y) {
            return false;
        }
    }

    true
}

fn is_in_area(sx: i32, sy: i32, bx: i32, by: i32, px: i32, py: i32) -> bool {
    let len = (bx - sx).abs() + (by - sy).abs();
    let dist = (px - sx).abs() + (py - sy).abs();

    dist <= len
}

fn get_cover_points(sx: i32, sy: i32, bx: i32, by: i32) -> Vec<(i32, i32)> {
    let mut points = Vec::new();

    let len = (bx - sx).abs() + (by - sy).abs() + 1;

    for i in -len..=len {
        let x = sx + i;
        let y1 = sy + (len - i.abs());
        let y2 = sy - (len - i.abs());
        if y1 == y2 {
            points.push((x, y1));
        } else {
            points.push((x, y1));
            points.push((x, y2));
        }
    }

    points
}

fn impossible_beacon_count_at_row(input: &[(i32, i32, i32, i32)], row: i32) -> i32 {
    let mut impossibles = HashSet::<i32>::new();

    for (sx, sy, bx, by) in input {
        if *sy == row {
            impossibles.insert(*sx);
        }

        if *by == row {
            impossibles.insert(*bx);
        }
    }

    let prev_count = impossibles.len();

    for (sx, sy, bx, by) in input {
        let len = (bx - sx).abs() + (by - sy).abs();

        if (*sy - row).abs() > len {
            continue;
        }

        let count = len - (*sy - row).abs();
        for i in -count..=count {
            impossibles.insert(*sx + i);
        }
    }

    for x in 0..=20 {
        if impossibles.contains(&x) {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!("");

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
    #[ignore]
    fn test_part1() {
        let input = include_str!("../input/day_15.txt");
        let answer = solve_part1(input, 2000000);
        assert_eq!(answer, 5073496);
    }

    #[test]
    fn test_part2_sample() {
        let answer = solve_part2(SAMPLE_INPUT, 20);
        assert_eq!(answer, 56000011);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let input = include_str!("../input/day_15.txt");
        let answer = solve_part2(input, 4000000);
        assert_eq!(answer, 28145);
    }

    #[test]
    fn test_is_in_area() {
        let (sx, sy, bx, by) = (10, 10, 20, 10);
        assert_eq!(is_in_area(sx, sy, bx, by, 10, 10), true);
        assert_eq!(is_in_area(sx, sy, bx, by, 20, 10), true);
        assert_eq!(is_in_area(sx, sy, bx, by, 21, 10), false);
        assert_eq!(is_in_area(sx, sy, bx, by, 0, 10), true);
        assert_eq!(is_in_area(sx, sy, bx, by, -1, 10), false);
    }

    #[test]
    fn test_cover_points() {
        let (sx, sy, bx, by) = (10, 10, 12, 10);
        let cover_points = get_cover_points(sx, sy, bx, by);

        assert_eq!(cover_points.len(), 12);
        assert_eq!(
            cover_points,
            [
                (7, 10),
                (8, 11),
                (8, 9),
                (9, 12),
                (9, 8),
                (10, 13),
                (10, 7),
                (11, 12),
                (11, 8),
                (12, 11),
                (12, 9),
                (13, 10)
            ]
        )
    }
}
