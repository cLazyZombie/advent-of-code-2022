pub fn solve_part1(input: &str) -> u32 {
    0
}

pub fn solve_part2(input: &str) -> u32 {
    0
}

enum Dir {
    Left,
    Right,
}

fn load_input(input: &str) -> Vec<Dir> {
    input
        .as_bytes()
        .iter()
        .map(|c| match *c {
            b'<' => Dir::Left,
            b'>' => Dir::Right,
            _ => panic!("Invalid input"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"Sabqponm
abdefghi"#;

    #[test]
    fn test_load() {}

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day_17.txt");
        assert_eq!(solve_part1(input), 0);
    }

    #[test]
    fn test_part1_sample() {
        let answer = solve_part1(SAMPLE_INPUT);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/day_17.txt");
        assert_eq!(solve_part2(input), 0);
    }

    #[test]
    #[ignore]
    fn test_part2_sample() {
        let answer = solve_part2(SAMPLE_INPUT);
        assert_eq!(answer, 0);
    }
}
