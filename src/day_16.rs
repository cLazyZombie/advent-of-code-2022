pub fn solve_part1(input: &str) -> i32 {
    0
}

pub fn solve_part2(input: &str) -> i64 {
    0
}

fn parse_input(input: &str) -> () {}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"
"#;

    #[test]
    fn test_parse_sample() {
        let parsed = parse_input(SAMPLE_INPUT);
    }

    #[test]
    fn test_part1_sample() {
        let answer = solve_part1(SAMPLE_INPUT);
        assert_eq!(answer, 26);
    }

    #[test]
    #[ignore]
    fn test_part1() {
        let input = include_str!("../input/day_16.txt");
        let answer = solve_part1(input);
        assert_eq!(answer, 5073496);
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
