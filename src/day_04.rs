pub fn solve_part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let (left, right) = parse_line(line);
        if is_fully_contains(left, right) || is_fully_contains(right, left) {
            sum += 1;
        }
    }
    sum
}

fn parse_line(line: &str) -> ((u32, u32), (u32, u32)) {
    let mut iter = line.trim().split(',');
    let range = iter.next().unwrap();
    let range_a = parse_range(range);

    let range = iter.next().unwrap();
    let range_b = parse_range(range);

    (range_a, range_b)
}

fn parse_range(range: &str) -> (u32, u32) {
    let mut iter = range.split('-');
    let start = iter.next().unwrap().parse().unwrap();
    let end = iter.next().unwrap().parse().unwrap();
    (start, end)
}

fn is_fully_contains(range_a: (u32, u32), range_b: (u32, u32)) -> bool {
    range_a.0 <= range_b.0 && range_a.1 >= range_b.1
}

pub fn solve_part2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let (left, right) = parse_line(line);
        if is_overlap(left, right) {
            sum += 1;
        }
    }
    sum
}

fn is_overlap(range_a: (u32, u32), range_b: (u32, u32)) -> bool {
    u32::min(range_a.1, range_b.1) >= u32::max(range_a.0, range_b.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = r#"2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8"#;

        let answer = solve_part1(input);
        assert_eq!(answer, 2);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day_04.txt");
        let answer = solve_part1(input);
        assert_eq!(answer, 582);
    }

    #[test]
    fn test_parse_range() {
        let range = "1-100";
        assert_eq!(parse_range(range), (1, 100));
    }

    #[test]
    fn test_parse_line() {
        let line = "1-100,200-300";
        assert_eq!(parse_line(line), ((1, 100), (200, 300)));
    }

    #[test]
    fn test_part2_sample() {
        let input = r#"2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8"#;
        let answer = solve_part2(input);
        assert_eq!(answer, 4);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/day_04.txt");
        let answer = solve_part2(input);
        assert_eq!(answer, 893);
    }

    #[test]
    fn test_is_overlap() {
        assert_eq!(is_overlap((1, 100), (50, 200)), true);
        assert_eq!(is_overlap((1, 100), (101, 200)), false);
        assert_eq!(is_overlap((1, 100), (0, 50)), true);
        assert_eq!(is_overlap((1, 100), (100, 100)), true);
        assert_eq!(is_overlap((1, 100), (100, 101)), true);
        assert_eq!(is_overlap((1, 100), (0, 0)), false);
    }
}
