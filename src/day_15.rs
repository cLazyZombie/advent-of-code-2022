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
    let mut init = ConvexHull::new();
    init.points = vec![(0, 0), (size, 0), (size, size), (0, size)];

    let mut world = vec![init];

    for (sx, sy, bx, by) in parsed {
        let len = (bx - sx).abs() + (by - sy).abs();

        let planes = [
            Plane::from_point(Dir::UpLeft, (sx - len, sy)),
            Plane::from_point(Dir::UpRight, (sx + len, sy)),
            Plane::from_point(Dir::DownRight, (sx + len, sy)),
            Plane::from_point(Dir::DownLeft, (sx - len, sy)),
        ];

        // culling
        let mut remaining = Vec::new();
        for plane in &planes {
            let mut candidates = Vec::new();

            for convex in &mut world {
                let (front, back) = convex.divide(plane);
                if let Some(front) = front {
                    remaining.push(front);
                }

                if let Some(back) = back {
                    candidates.push(back);
                }
            }
            world = candidates;
        }
        world = remaining;
    }
    println!("world: {:?}", world);
    0
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

#[derive(Debug, PartialEq, Eq, Clone)]
struct ConvexHull {
    points: Vec<(i32, i32)>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    DownRight,
    DownLeft,
    UpLeft,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Plane {
    normal: Dir,
    distance: i32,
}

impl Plane {
    fn new(normal: Dir, distance: i32) -> Plane {
        Plane { normal, distance }
    }

    fn from_point(normal: Dir, p: (i32, i32)) -> Plane {
        let distance = match normal {
            Dir::Up => -p.1,
            Dir::Down => p.1,
            Dir::Left => -p.0,
            Dir::Right => p.0,
            Dir::UpRight => -p.0 + p.1,
            Dir::DownRight => p.0 + p.1,
            Dir::DownLeft => p.0 - p.1,
            Dir::UpLeft => -p.0 - p.1,
        } + 1;

        Plane { normal, distance }
    }

    fn distance(&self, p: (i32, i32)) -> i32 {
        match self.normal {
            Dir::Up => -p.1 - self.distance,
            Dir::Down => p.1 - self.distance,
            Dir::Left => -p.0 - self.distance,
            Dir::Right => p.0 - self.distance,
            Dir::UpRight => p.0 - p.1 - self.distance,
            Dir::DownRight => p.0 + p.1 - self.distance,
            Dir::DownLeft => -p.0 + p.1 - self.distance,
            Dir::UpLeft => -p.0 - p.1 - self.distance,
        }
    }

    fn opposite(&self) -> Plane {
        Plane {
            normal: match self.normal {
                Dir::Up => Dir::Down,
                Dir::Down => Dir::Up,
                Dir::Left => Dir::Right,
                Dir::Right => Dir::Left,
                Dir::UpRight => Dir::DownLeft,
                Dir::DownRight => Dir::UpLeft,
                Dir::DownLeft => Dir::UpRight,
                Dir::UpLeft => Dir::DownRight,
            },
            distance: -self.distance + 1,
        }
    }
}

impl ConvexHull {
    fn new() -> ConvexHull {
        ConvexHull { points: Vec::new() }
    }

    fn divide(&self, plane: &Plane) -> (Option<ConvexHull>, Option<ConvexHull>) {
        if self.points.len() == 1 {
            if plane.distance(self.points[0]) >= 0 {
                return (Some(self.clone()), None);
            } else {
                return (None, Some(self.clone()));
            }
        }

        let front = self.cull(plane);
        let back = self.cull(&plane.opposite());
        (front, back)
    }

    fn cull(&self, plane: &Plane) -> Option<ConvexHull> {
        if self.points.len() == 1 {
            if plane.distance(self.points[0]) >= 0 {
                return Some(self.clone());
            } else {
                return None;
            }
        }

        let mut front = ConvexHull::new();

        let lines = self.points[0..self.points.len()]
            .iter()
            .zip(self.points[1..].iter().chain(self.points[0..1].iter()));

        for line in lines {
            let d1 = plane.distance(*line.0);
            let d2 = plane.distance(*line.1);

            if d1 >= 0 {
                front.points.push(*line.0);
            }

            if d1 * d2 < 0 {
                let (x1, y1) = line.0;
                let (x2, y2) = line.1;

                let x = (x1 * d2 - x2 * d1) / (d2 - d1);
                let y = (y1 * d2 - y2 * d1) / (d2 - d1);

                front.points.push((x, y));
            }
        }

        if front.points.is_empty() {
            None
        } else {
            Some(front)
        }
    }
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
        let answer = solve_part2(SAMPLE_INPUT, 20);
        assert_eq!(answer, 93);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/day_15.txt");
        let answer = solve_part2(input, 20);
        assert_eq!(answer, 28145);
    }

    #[test]
    fn test_opposite_plane() {
        let plane = Plane::new(Dir::Up, -10);
        assert_eq!(plane.opposite(), Plane::new(Dir::Down, 11));
    }

    #[test]
    fn test_divide_convex_hull() {
        let mut cv = ConvexHull::new();
        cv.points.push((0, 0));
        cv.points.push((10, 0));
        cv.points.push((10, 10));
        cv.points.push((0, 10));

        let plane = Plane::new(Dir::Up, -5);
        let (front, back) = cv.divide(&plane);
        assert_eq!(
            front.unwrap().points,
            vec![(0, 0), (10, 0), (10, 5), (0, 5)]
        );
        assert_eq!(
            back.unwrap().points,
            vec![(10, 6), (10, 10), (0, 10), (0, 6)]
        );

        let plane = Plane::new(Dir::UpLeft, -1);
        let (front, back) = cv.divide(&plane);
        assert_eq!(front.unwrap().points, vec![(0, 0), (1, 0), (0, 1)]);
        assert_eq!(
            back.unwrap().points,
            vec![(2, 0), (10, 0), (10, 10), (0, 10), (0, 2)]
        );

        let plane = Plane::new(Dir::UpLeft, 0);
        let (front, back) = cv.divide(&plane);
        assert_eq!(front.unwrap().points, vec![(0, 0)]);
        assert_eq!(
            back.unwrap().points,
            vec![(1, 0), (10, 0), (10, 10), (0, 10), (0, 1)]
        );

        let plane = Plane::new(Dir::UpRight, 11);
        let (front, back) = cv.divide(&plane);
        assert_eq!(front, None);
        assert_eq!(back, Some(cv));
    }
}
