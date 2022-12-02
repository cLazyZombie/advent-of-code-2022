#[derive(Debug, Copy, Clone)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl<'a> From<&'a str> for RPS {
    fn from(s: &'a str) -> Self {
        match s {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => panic!("Invalid input"),
        }
    }
}

impl RPS {
    pub fn shape_score(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    pub fn beats_score(&self, other: RPS) -> u32 {
        match (*self, other) {
            (RPS::Rock, RPS::Rock) => 3,
            (RPS::Paper, RPS::Paper) => 3,
            (RPS::Scissors, RPS::Scissors) => 3,
            (RPS::Rock, RPS::Scissors) => 6,
            (RPS::Paper, RPS::Rock) => 6,
            (RPS::Scissors, RPS::Paper) => 6,
            _ => 0,
        }
    }
}

fn score(mine: RPS, other: RPS) -> u32 {
    mine.shape_score() + mine.beats_score(other)
}

#[derive(Debug, Copy, Clone)]
enum MatchResult {
    Win,
    Lose,
    Draw,
}

impl<'a> From<&'a str> for MatchResult {
    fn from(s: &'a str) -> Self {
        match s {
            "X" => MatchResult::Lose,
            "Y" => MatchResult::Draw,
            "Z" => MatchResult::Win,
            _ => panic!("Invalid input"),
        }
    }
}

fn expected_rps(other: RPS, result: MatchResult) -> RPS {
    match (other, result) {
        (RPS::Rock, MatchResult::Win) => RPS::Paper,
        (RPS::Rock, MatchResult::Draw) => RPS::Rock,
        (RPS::Rock, MatchResult::Lose) => RPS::Scissors,
        (RPS::Paper, MatchResult::Win) => RPS::Scissors,
        (RPS::Paper, MatchResult::Draw) => RPS::Paper,
        (RPS::Paper, MatchResult::Lose) => RPS::Rock,
        (RPS::Scissors, MatchResult::Win) => RPS::Rock,
        (RPS::Scissors, MatchResult::Draw) => RPS::Scissors,
        (RPS::Scissors, MatchResult::Lose) => RPS::Paper,
    }
}

pub fn total_score_part1(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let mut it = line.trim().split_ascii_whitespace();
        if it.clone().count() != 2 {
            continue;
        }

        let opponent: RPS = it.next().unwrap().into();
        let mine: RPS = it.next().unwrap().into();
        let score = score(mine, opponent);
        total += score
    }
    total
}

pub fn total_score_part2(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let mut it = line.trim().split_ascii_whitespace();
        if it.clone().count() != 2 {
            continue;
        }

        let opponent: RPS = it.next().unwrap().into();
        let guide: MatchResult = it.next().unwrap().into();
        let mine = expected_rps(opponent, guide);
        let score = score(mine, opponent);
        total += score
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_score_by_guide() {
        let input = r#"
        A Y
        B X
        C Z"#;

        let score = total_score_part1(input);
        assert_eq!(score, 15);
    }

    #[test]
    fn part1() {
        let input = include_str!("../input/day_02.txt");
        let score = total_score_part1(input);
        assert_eq!(score, 13484);
    }

    #[test]
    fn test_part2() {
        let input = r#"
        A Y
        B X
        C Z"#;

        let score = total_score_part2(input);
        assert_eq!(score, 12);
    }

    #[test]
    fn part2() {
        let input = include_str!("../input/day_02.txt");
        let score = total_score_part2(input);
        assert_eq!(score, 13433);
    }
}
