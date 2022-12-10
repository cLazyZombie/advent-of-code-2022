use std::str::FromStr;

pub fn solve_part1(input: &str) -> i32 {
    let instructions = parse_instructoins(input);
    let mut ops = instructions.into_iter();
    let mut x = 1;
    let mut cur_instruction: Option<(Instruction, u32)> = None;
    let mut sum = 0;

    for cycle in 1..=220 {
        run_instruction(&mut cur_instruction, &mut x, &mut ops, cycle, &mut sum);
    }
    sum
}

pub fn solve_part2(input: &str) -> String {
    let instructions = parse_instructoins(input);
    let mut ops = instructions.into_iter();
    let mut x = 1;
    let mut cur_instruction: Option<(Instruction, u32)> = None;
    let mut sum = 0;
    let mut result = String::new();

    for cycle in 1..=240 {
        apply_prev_instruction(&mut cur_instruction, &mut x);
        // run_instruction(&mut cur_instruction, &mut x, &mut ops, cycle, &mut sum);

        let pixel = (cycle - 1) % 40;
        let sprite = x;

        if cycle != 1 && (cycle - 1) % 40 == 0 {
            result += "\n";
        }

        if pixel >= sprite - 1 && pixel <= sprite + 1 {
            result += "#";
        } else {
            result += ".";
        }

        run_cur_instruction(&mut cur_instruction, &mut x, &mut ops, cycle, &mut sum);
    }

    println!("{result}");

    result
}

fn apply_prev_instruction(cur_instruction: &mut Option<(Instruction, u32)>, x: &mut i32) {
    if let Some((op, mut count)) = *cur_instruction {
        count -= 1;
        if count == 0 {
            op.apply(x);
            *cur_instruction = None;
        } else {
            *cur_instruction = Some((op, count));
        }
    }
}

fn run_cur_instruction(
    cur_instruction: &mut Option<(Instruction, u32)>,
    x: &mut i32,
    ops: &mut std::vec::IntoIter<Instruction>,
    cycle: i32,
    sum: &mut i32,
) {
    if cur_instruction.is_none() {
        let op = ops.next().unwrap();
        *cur_instruction = Some((op, op.cycle()));
    }
    if cycle == 20 || (cycle + 20) % 40 == 0 {
        *sum += *x * cycle;
    }
}

fn run_instruction(
    cur_instruction: &mut Option<(Instruction, u32)>,
    x: &mut i32,
    ops: &mut std::vec::IntoIter<Instruction>,
    cycle: i32,
    sum: &mut i32,
) {
    apply_prev_instruction(cur_instruction, x);
    run_cur_instruction(cur_instruction, x, ops, cycle, sum);
    // if let Some((op, mut count)) = *cur_instruction {
    //     count -= 1;
    //     if count == 0 {
    //         op.apply(x);
    //         *cur_instruction = None;
    //     } else {
    //         *cur_instruction = Some((op, count));
    //     }
    // }
    // if cur_instruction.is_none() {
    //     let op = ops.next().unwrap();
    //     *cur_instruction = Some((op, op.cycle()));
    // }
    // if cycle == 20 || (cycle + 20) % 40 == 0 {
    //     *sum += *x * cycle;
    // }
}

fn parse_instructoins(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    AddX(i32),
    NoOp,
}

impl Instruction {
    pub fn cycle(&self) -> u32 {
        match self {
            Instruction::AddX(_) => 2,
            Instruction::NoOp => 1,
        }
    }

    pub fn apply(&self, x: &mut i32) {
        match self {
            Instruction::AddX(arg) => *x += arg,
            Instruction::NoOp => (),
        }
    }
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let op = iter.next().unwrap();
        match op {
            "addx" => {
                let arg = iter.next().unwrap().parse::<i32>().unwrap();
                Ok(Instruction::AddX(arg))
            }
            "noop" => Ok(Instruction::NoOp),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;
        let answer = solve_part1(input);
        assert_eq!(answer, 13140);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day_10.txt");
        let answer = solve_part1(input);
        assert_eq!(answer, 11720);
    }

    #[test]
    fn test_part2_sample() {
        let input = r#"addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop"#;
        let answer = solve_part2(input);
        assert_eq!(
            answer,
            r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#
        );
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/day_10.txt");
        let answer = solve_part2(input);
        assert_eq!(answer, "####.###...##..###..####.###...##....##.\n#....#..#.#..#.#..#.#....#..#.#..#....#.\n###..#..#.#....#..#.###..#..#.#.......#.\n#....###..#....###..#....###..#.......#.\n#....#.#..#..#.#.#..#....#....#..#.#..#.\n####.#..#..##..#..#.####.#.....##...##..");
    }
}
