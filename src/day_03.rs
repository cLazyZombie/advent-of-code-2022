use std::collections::{hash_map::RandomState, HashSet};

pub fn solve_part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let line = line.trim();
        let (left, right) = divide_line(line);
        let in_both = find_both_compartments(left, right).unwrap();
        let priority = aplha_to_priority(in_both);
        sum += priority;
    }
    sum
}

fn divide_line(line: &str) -> (&str, &str) {
    let len = line.len();
    line.split_at(len / 2)
}

fn aplha_to_priority(c: u8) -> u32 {
    match c {
        b'a'..=b'z' => (c - b'a') as u32 + 1,
        b'A'..=b'Z' => (c - b'A') as u32 + 27,
        _ => panic!("Invalid input"),
    }
}

fn find_both_compartments(left: &str, right: &str) -> Option<u8> {
    let (mut i, mut j) = (0, 0);
    let (mut left, mut right): (Vec<_>, Vec<_>) = (
        left.as_bytes().iter().collect(),
        right.as_bytes().iter().collect(),
    );
    left.sort();
    right.sort();

    while i < left.len() && j < right.len() {
        if left[i] == right[j] {
            return Some(*left[i]);
        }
        if left[i] < right[j] {
            i += 1;
        } else {
            j += 1;
        }
    }

    None
}

pub fn solve_part2(input: &str) -> u32 {
    let mut sum = 0;
    let mut lines = input.lines();
    loop {
        let Some(line_a) = lines.next() else {
            break;
        };
        let line_b = lines.next().unwrap();
        let line_c = lines.next().unwrap();

        let badge = find_badge(line_a, line_b, line_c).unwrap();
        sum += aplha_to_priority(badge);
    }

    sum
}

fn find_badge(line_a: &str, line_b: &str, line_c: &str) -> Option<u8> {
    let line_a: HashSet<u8, RandomState> = HashSet::from_iter(line_a.trim().bytes());
    let line_b: HashSet<u8, RandomState> = HashSet::from_iter(line_b.trim().bytes());
    let line_c: HashSet<u8, RandomState> = HashSet::from_iter(line_c.trim().bytes());

    let line_ab = line_a
        .intersection(&line_b)
        .into_iter()
        .copied()
        .collect::<HashSet<u8, RandomState>>();

    let mut line_abc = line_ab.intersection(&line_c).into_iter().copied();
    line_abc.next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_line() {
        assert_eq!(divide_line("abcdef"), ("abc", "def"));
    }

    #[test]
    fn test_alpha_to_priority() {
        assert_eq!(aplha_to_priority(b'a'), 1);
        assert_eq!(aplha_to_priority(b'z'), 26);
        assert_eq!(aplha_to_priority(b'A'), 27);
        assert_eq!(aplha_to_priority(b'Z'), 52);
    }

    #[test]
    fn test_find_both_compartments() {
        assert_eq!(find_both_compartments("abc", "def"), None);
        assert_eq!(find_both_compartments("abc", "dea"), Some(b'a'));
        assert_eq!(find_both_compartments("ABC", "ICO"), Some(b'C'));
    }

    #[test]
    fn test_part1_sample() {
        let sample = r#"vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw"#;

        let answer = solve_part1(sample);
        assert_eq!(answer, 157);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day_03.txt");
        let answer = solve_part1(input);
        assert_eq!(answer, 7817);
    }

    #[test]
    fn test_find_badge() {
        let line_a = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let line_b = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let line_c = "PmmdzqPrVvPwwTWBwg";
        assert_eq!(Some(b'r'), find_badge(line_a, line_b, line_c));

        let line_a = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
        let line_b = "ttgJtRGJQctTZtZT";
        let line_c = "CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(Some(b'Z'), find_badge(line_a, line_b, line_c));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/day_03.txt");
        let answer = solve_part2(input);
        assert_eq!(answer, 2444);
    }
}
