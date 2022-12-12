pub fn solve_part1(input: &str) -> u64 {
    let mut monkeys = load_monkeys(input);

    for _round in 1..=20 {
        for monkey_idx in 0..monkeys.len() {
            monkeys[monkey_idx].inspected += monkeys[monkey_idx].items.len() as u64;
            for item in monkeys[monkey_idx].items.clone() {
                let changed = monkeys[monkey_idx].operation.run(item) / 3;
                let test_result = monkeys[monkey_idx].test.run(changed);
                let throw_to = if test_result {
                    monkeys[monkey_idx].if_true_throw_to as usize
                } else {
                    monkeys[monkey_idx].if_false_throw_to as usize
                };

                monkeys[throw_to].items.push(changed);
            }
            monkeys[monkey_idx].items.clear();
        }
    }

    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    monkeys[0].inspected * monkeys[1].inspected
}

pub fn solve_part2(input: &str) -> u64 {
    let mut monkeys = load_monkeys(input);

    let mut total_divider = 1;
    for monkey in &monkeys {
        match &monkey.test {
            Test::DivisibleBy(divider) => {
                total_divider *= divider;
            }
        }
    }
    // println!("total_divider: {}", total_divider);

    // for _round in 1..=10000 {
    for _round in 1..=10000 {
        for monkey_idx in 0..monkeys.len() {
            monkeys[monkey_idx].inspected += monkeys[monkey_idx].items.len() as u64;
            for item in monkeys[monkey_idx].items.clone() {
                let changed = monkeys[monkey_idx].operation.run(item) % total_divider;
                let test_result = monkeys[monkey_idx].test.run(changed);
                let throw_to = if test_result {
                    monkeys[monkey_idx].if_true_throw_to as usize
                } else {
                    monkeys[monkey_idx].if_false_throw_to as usize
                };

                monkeys[throw_to].items.push(changed);
            }
            monkeys[monkey_idx].items.clear();
        }

        // println!("== After round {} ==", _round);
        // for (idx, monkey) in monkeys.iter().enumerate() {
        //     println!("Monkey {}: {} times", idx, monkey.inspected);
        // }
    }

    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    monkeys[0].inspected * monkeys[1].inspected
}

fn load_monkeys(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let mut lines = input.lines();
    loop {
        let monkey_line = lines.next().unwrap().trim();

        let _id: u32 = monkey_line
            .trim()
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse()
            .unwrap();

        let starting_items_line = lines.next().unwrap().trim();
        let starting_items: Vec<u64> = starting_items_line
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();

        let operation_line = lines.next().unwrap().trim();
        let operation: Vec<&str> = operation_line
            .strip_prefix("Operation: new = old ")
            .unwrap()
            .split_whitespace()
            .collect();
        let operation = match (operation[0], operation[1]) {
            ("+", "old") => Operation::AddOld,
            ("+", s) => {
                let n: u64 = s.parse().unwrap();
                Operation::Add(n)
            }
            ("*", "old") => Operation::MultiplyOld,
            ("*", s) => {
                let n: u64 = s.parse().unwrap();
                Operation::Multiply(n)
            }
            _ => panic!("Unknown operation"),
        };

        let test_line = lines.next().unwrap().trim();
        let test_cond: u64 = test_line
            .strip_prefix("Test: divisible by ")
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        let if_true: u32 = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        let if_false: u32 = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        let monkey = Monkey {
            items: starting_items,
            operation,
            test: Test::DivisibleBy(test_cond),
            if_true_throw_to: if_true,
            if_false_throw_to: if_false,
            inspected: 0,
        };
        monkeys.push(monkey);

        if lines.next().is_none() {
            break;
        }
    }

    monkeys
}

#[derive(Debug, PartialEq, Eq)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
    if_true_throw_to: u32,
    if_false_throw_to: u32,
    inspected: u64,
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Add(u64),
    AddOld,
    Multiply(u64),
    MultiplyOld,
}

impl Operation {
    fn run(&self, item: u64) -> u64 {
        match self {
            Operation::Add(n) => item + n,
            Operation::AddOld => item + item,
            Operation::Multiply(n) => item * n,
            Operation::MultiplyOld => item * item,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Test {
    DivisibleBy(u64),
}

impl Test {
    fn run(&self, item: u64) -> bool {
        match self {
            Test::DivisibleBy(n) => item % n == 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3
      
      Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
          If true: throw to monkey 2
          If false: throw to monkey 0
      
      Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
          If true: throw to monkey 1
          If false: throw to monkey 3
      
      Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
          If true: throw to monkey 0
          If false: throw to monkey 1"#;

    #[test]
    fn test_load_monkeys() {
        let monkeys = load_monkeys(SAMPLE_INPUT);
        assert_eq!(monkeys.len(), 4);

        let monkey0 = &monkeys[0];
        let expected = Monkey {
            items: vec![79, 98],
            operation: Operation::Multiply(19),
            test: Test::DivisibleBy(23),
            if_true_throw_to: 2,
            if_false_throw_to: 3,
            inspected: 0,
        };
        assert_eq!(monkey0, &expected);
    }

    #[test]
    fn test_part1_sample() {
        let answer = solve_part1(SAMPLE_INPUT);
        assert_eq!(answer, 10605);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day_11.txt");
        let answer = solve_part1(input);
        assert_eq!(answer, 88208);
    }

    #[test]
    fn test_part2_sample() {
        let answer = solve_part2(SAMPLE_INPUT);
        assert_eq!(answer, 2713310158);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/day_11.txt");
        let answer = solve_part2(input);
        assert_eq!(answer, 21115867968);
    }
}
