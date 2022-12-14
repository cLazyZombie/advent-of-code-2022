pub fn solve_part1(input: &str) -> i32 {
    let lines = parse_input(input);
    let mut points = Vec::new();
    let max_y = find_max_y(&lines);
    let mut endless = None;
    'outer: for idx in 1.. {
        let mut p = (500, 0);
        println!("{}: {:?}", idx, p);
        loop {
            if p.1 >= max_y {
                endless = Some(idx);
                break 'outer;
            }

            // go down
            let next_p = (p.0, p.1 + 1);
            if is_empty(next_p, &lines, &points) {
                p = next_p;
                continue;
            }

            // go down left
            let next_p = (p.0 - 1, p.1 + 1);
            if is_empty(next_p, &lines, &points) {
                p = next_p;
                continue;
            }

            // go down right
            let next_p = (p.0 + 1, p.1 + 1);
            if is_empty(next_p, &lines, &points) {
                p = next_p;
                continue;
            }

            // stuck
            points.push(p);
            break;
        }
    }

    endless.unwrap() - 1
}

pub fn solve_part2(input: &str) -> i32 {
    let mut lines = parse_input(input);

    // floor
    let max_y = find_max_y(&lines) + 2;
    let floor = Line::new((i32::MIN, max_y), (i32::MAX, max_y));
    lines.push(floor);

    let mut points = Vec::new();

    let mut full = None;
    'outer: for idx in 1.. {
        let mut p = (500, 0);
        loop {
            // go down
            let next_p = (p.0, p.1 + 1);
            if is_empty(next_p, &lines, &points) {
                p = next_p;
                continue;
            }

            // go down left
            let next_p = (p.0 - 1, p.1 + 1);
            if is_empty(next_p, &lines, &points) {
                p = next_p;
                continue;
            }

            // go down right
            let next_p = (p.0 + 1, p.1 + 1);
            if is_empty(next_p, &lines, &points) {
                p = next_p;
                continue;
            }

            // stuck
            // println!("{}: {:?}", idx, p);
            if p == (500, 0) {
                full = Some(idx);
                break 'outer;
            }
            points.push(p);
            break;
        }
    }

    full.unwrap()
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .flat_map(|line| parse_lines(line).into_iter())
        .collect()
}

fn is_empty(p: (i32, i32), lines: &[Line], points: &[(i32, i32)]) -> bool {
    lines.iter().all(|line| !line.has_point(p)) && points.iter().all(|point| !point.has_point(p))
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

trait HasPoint {
    fn has_point(&self, p: (i32, i32)) -> bool;
}

impl HasPoint for (i32, i32) {
    fn has_point(&self, p: (i32, i32)) -> bool {
        self == &p
    }
}

impl HasPoint for Line {
    fn has_point(&self, p: (i32, i32)) -> bool {
        self.min_x() <= p.0 && p.0 <= self.max_x() && self.min_y() <= p.1 && p.1 <= self.max_y()
    }
}

impl Line {
    fn new(start: (i32, i32), end: (i32, i32)) -> Self {
        Line { start, end }
    }

    fn min_x(&self) -> i32 {
        self.start.0.min(self.end.0)
    }

    fn max_x(&self) -> i32 {
        self.start.0.max(self.end.0)
    }

    fn min_y(&self) -> i32 {
        self.start.1.min(self.end.1)
    }

    fn max_y(&self) -> i32 {
        self.start.1.max(self.end.1)
    }
}

fn find_max_y(lines: &[Line]) -> i32 {
    lines.iter().map(|line| line.max_y()).max().unwrap()
}

fn parse_lines(s: &str) -> Vec<Line> {
    let points: Vec<_> = s
        .split(" -> ")
        .map(|point| {
            let mut it = point.split(',').map(|s| s.trim().parse::<i32>().unwrap());
            let x = it.next().unwrap();
            let y = it.next().unwrap();
            (x, y)
        })
        .collect();

    let mut lines = Vec::new();
    for (start, end) in points.iter().zip(points.iter().skip(1)) {
        let line = Line::new(*start, *end);
        lines.push(line);
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"498,4 -> 498,6 -> 496,6
    503,4 -> 502,4 -> 502,9 -> 494,9"#;

    #[test]
    fn test_parse_sample() {
        let lines = parse_input(SAMPLE_INPUT);
        assert_eq!(lines.len(), 5);
        assert_eq!(lines[0], Line::new((498, 4), (498, 6)));
        assert_eq!(lines[1], Line::new((498, 6), (496, 6)));
        assert_eq!(lines[2], Line::new((503, 4), (502, 4)));
        assert_eq!(lines[3], Line::new((502, 4), (502, 9)));
        assert_eq!(lines[4], Line::new((502, 9), (494, 9)));
    }

    #[test]
    fn test_part1_sample() {
        let answer = solve_part1(SAMPLE_INPUT);
        assert_eq!(answer, 24);
    }

    #[test]
    #[ignore]
    fn test_part1() {
        let input = include_str!("../input/day_14.txt");
        let answer = solve_part1(input);
        assert_eq!(answer, 737);
    }

    #[test]
    fn test_part2_sample() {
        let answer = solve_part2(SAMPLE_INPUT);
        assert_eq!(answer, 93);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/day_14.txt");
        let answer = solve_part2(input);
        assert_eq!(answer, 0);
    }
}
