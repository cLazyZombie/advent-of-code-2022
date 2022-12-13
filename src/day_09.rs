use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
enum Dir {
    R,
    L,
    U,
    D,
}

impl FromStr for Dir {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Dir::R),
            "L" => Ok(Dir::L),
            "U" => Ok(Dir::U),
            "D" => Ok(Dir::D),
            _ => Err(()),
        }
    }
}

pub fn solve_part1(input: &str) -> u32 {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut h = (0, 0);
    let mut t = (0, 0);

    visited.insert(t);

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let dir = iter.next().unwrap().parse::<Dir>().unwrap();
        let count = iter.next().unwrap().parse::<u32>().unwrap();

        for _ in 0..count {
            match dir {
                Dir::R => h = (h.0 + 1, h.1),
                Dir::L => h = (h.0 - 1, h.1),
                Dir::U => h = (h.0, h.1 + 1),
                Dir::D => h = (h.0, h.1 - 1),
            }

            // follow tail
            t = follow(h, t);
            visited.insert(t);
        }
    }

    visited.len() as u32
}

fn is_far(l: (i32, i32), r: (i32, i32)) -> bool {
    (l.0 - r.0).abs() > 1 || (l.1 - r.1).abs() > 1
}

fn follow(h: (i32, i32), t: (i32, i32)) -> (i32, i32) {
    if is_far(h, t) {
        let next = match ((h.0 - t.0).signum(), (h.1 - t.1).signum()) {
            (1, 0) => (t.0 + 1, t.1),
            (-1, 0) => (t.0 - 1, t.1),
            (0, 1) => (t.0, t.1 + 1),
            (0, -1) => (t.0, t.1 - 1),
            (1, 1) => (t.0 + 1, t.1 + 1),
            (1, -1) => (t.0 + 1, t.1 - 1),
            (-1, 1) => (t.0 - 1, t.1 + 1),
            (-1, -1) => (t.0 - 1, t.1 - 1),
            _ => (t.0, t.1),
        };
        return next;
    }

    t
}

pub fn solve_part2(input: &str) -> u32 {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut snakes = [(0, 0); 10];

    visited.insert(*snakes.last().unwrap());

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let dir = iter.next().unwrap().parse::<Dir>().unwrap();
        let count = iter.next().unwrap().parse::<u32>().unwrap();

        for _ in 0..count {
            let mut h = *snakes.first().unwrap();
            match dir {
                Dir::R => h = (h.0 + 1, h.1),
                Dir::L => h = (h.0 - 1, h.1),
                Dir::U => h = (h.0, h.1 + 1),
                Dir::D => h = (h.0, h.1 - 1),
            }
            snakes[0] = h;

            for i in 1..snakes.len() {
                snakes[i] = follow(snakes[i - 1], snakes[i]);
            }
            visited.insert(*snakes.last().unwrap());
        }
    }

    visited.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = r#" R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2 "#;
        let answer = solve_part1(input);
        assert_eq!(answer, 13);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day_09.txt");
        let answer = solve_part1(input);
        assert_eq!(answer, 6406);
    }

    #[test]
    fn test_part2_sample() {
        let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20 "#;
        let answer = solve_part2(input);
        assert_eq!(answer, 36);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/day_09.txt");
        let answer = solve_part2(input);
        assert_eq!(answer, 2643);
    }
}
