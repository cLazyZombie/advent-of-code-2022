pub fn most_calories(input: &str) -> u32 {
    let mut max_calories = 0;
    let mut sum_calories = 0;

    for line in input.lines() {
        let Ok(calories) = line.parse::<u32>() else {
            if sum_calories > max_calories {
                max_calories = sum_calories;
            }
            sum_calories = 0;
            continue;
        };

        sum_calories += calories;
    }

    max_calories
}

pub fn top_three_calories(input: &str) -> u32 {
    let mut top_three: [u32; 4] = [0, 0, 0, 0]; // 1st element is new input
    let mut sum_calories = 0;

    for line in input.lines() {
        let Ok(calories) = line.parse::<u32>() else {
            top_three[0] = sum_calories;
            top_three.sort();
            sum_calories = 0;
            continue;
        };

        sum_calories += calories;
    }

    top_three.iter().skip(1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input/day_01.txt");
        let max_calories = most_calories(input);
        assert_eq!(max_calories, 74394);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input/day_01.txt");
        let top_three = top_three_calories(input);
        assert_eq!(top_three, 212836);
    }
}
