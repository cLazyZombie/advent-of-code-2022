use std::{collections::VecDeque, str::Lines};

pub fn solve_part1(input: &str) -> String {
    let mut lines = input.lines();
    let mut cargo_stack = load_cargo_stack(&mut lines);
    let commands = parse_command(&mut lines);

    for command in commands {
        let Command::Move(count, from, to) = command;

        for _ in 0..count {
            let cargo = cargo_stack[from - 1].pop_back().unwrap();
            cargo_stack[to - 1].push_back(cargo);
        }
    }

    cargo_stack
        .iter()
        .map(|stack| *stack.back().unwrap())
        .collect()
}

pub fn solve_part2(input: &str) -> String {
    let mut lines = input.lines();
    let mut cargo_stack = load_cargo_stack(&mut lines);
    let commands = parse_command(&mut lines);

    for command in commands {
        let Command::Move(count, from, to) = command;

        let mut temp = Vec::new();
        for _ in 0..count {
            let cargo = cargo_stack[from - 1].pop_back().unwrap();
            temp.push(cargo);
        }

        for cargo in temp.into_iter().rev() {
            cargo_stack[to - 1].push_back(cargo);
        }
    }

    cargo_stack
        .iter()
        .map(|stack| *stack.back().unwrap())
        .collect()
}

fn get_cargo_stack_count(lines: &Lines) -> usize {
    let char_count = lines.clone().next().unwrap().chars().count();
    (char_count + 1) / 4
}

fn load_cargo_stack(lines: &mut Lines) -> VecDeque<VecDeque<char>> {
    let stack_count = get_cargo_stack_count(lines);

    let mut result = VecDeque::new();
    result.resize(stack_count, VecDeque::new());

    loop {
        let Some(line) = lines.next() else {
            break;
        };

        if line.chars().any(|c| c.is_numeric()) {
            break;
        }

        // add existing crates
        for (idx, c) in line.chars().enumerate() {
            if c.is_alphabetic() {
                let stack_idx = idx / 4;
                result[stack_idx].push_front(c);
            }
        }
    }

    result
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Command {
    Move(usize, usize, usize),
}

fn parse_command(lines: &mut Lines) -> Vec<Command> {
    let mut commands = Vec::new();

    for line in lines {
        let mut split = line.trim().split_ascii_whitespace();
        if split.clone().count() != 6 {
            continue;
        }

        let _mv = split.next().unwrap();
        let count = split.next().unwrap().parse::<usize>().unwrap();
        let _from = split.next().unwrap();
        let from = split.next().unwrap().parse::<usize>().unwrap();
        let _to = split.next().unwrap();
        let to = split.next().unwrap().parse::<usize>().unwrap();

        commands.push(Command::Move(count, from, to));
    }

    commands
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_cargo_stack() {
        let input = r#"[A]     [B]
[C] [D] [E]
 1   2   3"#;
        let mut lines = input.lines();
        let cargo_stack = load_cargo_stack(&mut lines);
        assert_eq!(cargo_stack.len(), 3);
        assert_eq!(cargo_stack[0], ['C', 'A']);
        assert_eq!(cargo_stack[1], ['D']);
        assert_eq!(cargo_stack[2], ['E', 'B']);
    }

    #[test]
    fn test_parse_command() {
        let input = r#"move 1 from 2 to 1
        move 3 from 1 to 3"#;

        let mut lines = input.lines();
        let commands = parse_command(&mut lines);
        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0], Command::Move(1, 2, 1));
        assert_eq!(commands[1], Command::Move(3, 1, 3));
    }

    #[test]
    fn test_part1_sample() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 
        
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
        let answer = solve_part1(input);
        assert_eq!(answer, "CMZ");
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day_05.txt");
        let answer = solve_part1(input);
        assert_eq!(answer, "BSDMQFLSP");
    }

    #[test]
    fn test_part2_sample() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 
        
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
        let answer = solve_part2(input);
        assert_eq!(answer, "MCD");
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/day_05.txt");
        let answer = solve_part2(input);
        assert_eq!(answer, "PGSQBFLDP");
    }
}
