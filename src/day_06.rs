pub fn solve_part1(input: &str) -> u32 {
    solve::<4>(input)
}

pub fn solve_part2(input: &str) -> u32 {
    solve::<14>(input)
}

fn solve<const LEN: usize>(input: &str) -> u32 {
    let mut last_4 = [0_u8; LEN];
    for (idx, &c) in input.as_bytes().iter().enumerate() {
        last_4[idx % LEN] = c;

        if idx >= LEN - 1 && is_unique(&last_4) {
            return idx as u32 + 1;
        }
    }
    0
}

fn is_unique(slice: &[u8]) -> bool {
    for i in 0..slice.len() {
        for j in i + 1..slice.len() {
            if slice[i] == slice[j] {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        assert_eq!(solve_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(solve_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve_part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day_06.txt");
        let answer = solve_part1(input);
        assert_eq!(answer, 1651);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(solve_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solve_part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solve_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solve_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/day_06.txt");
        let answer = solve_part2(input);
        assert_eq!(answer, 3837);
    }
}
