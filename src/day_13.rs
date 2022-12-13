use std::cmp::Ordering;

pub fn solve_part1(input: &str) -> i32 {
    let mut result = 0;
    let input = parse_input(input);
    for (idx, (left, right)) in input.iter().enumerate() {
        let order = is_right_order(left, right);
        if order == Order::Right || order == Order::Continue {
            result += idx as i32 + 1;
        }
    }
    result
}

pub fn solve_part2(input: &str) -> i32 {
    let mut input: Vec<List> = parse_input(input)
        .into_iter()
        .map(|(l, r)| [l, r])
        .flatten()
        .collect();

    let div1 = parse_line("[[2]]");
    let div2 = parse_line("[[6]]");
    input.push(div1);
    input.push(div2);

    input.sort_by(|l, r| match is_right_order(l, r) {
        Order::Right => Ordering::Less,
        Order::NotRight => Ordering::Greater,
        Order::Continue => Ordering::Equal,
    });

    let mut result = 1;

    for (idx, list) in input.iter().enumerate() {
        if list.to_string() == "[[2]]" {
            result *= idx as i32 + 1;
        } else if list.to_string() == "[[6]]" {
            result *= idx as i32 + 1;
        }
    }
    result
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Order {
    Right,
    NotRight,
    Continue,
}

fn is_right_order(left: &List, right: &List) -> Order {
    let mut left_it = left.values.iter();
    let mut right_it = right.values.iter();
    loop {
        let left_value = left_it.next();
        let right_value = right_it.next();

        if left_value.is_none() && right_value.is_none() {
            return Order::Continue;
        } else if left_value.is_none() {
            return Order::Right;
        } else if right_value.is_none() {
            return Order::NotRight;
        }

        let left_value = left_value.unwrap();
        let right_value = right_value.unwrap();

        match (left_value, right_value) {
            (Value::Value(left_value), Value::Value(right_value)) => {
                if left_value < right_value {
                    return Order::Right;
                } else if right_value < left_value {
                    return Order::NotRight;
                }
            }
            (Value::Value(left_value), Value::List(right_list)) => {
                match is_right_order(
                    &List {
                        values: vec![Value::Value(*left_value)],
                    },
                    right_list,
                ) {
                    Order::Right => return Order::Right,
                    Order::NotRight => return Order::NotRight,
                    Order::Continue => {}
                }
            }
            (Value::List(left_list), Value::Value(right_value)) => {
                match is_right_order(
                    left_list,
                    &List {
                        values: vec![Value::Value(*right_value)],
                    },
                ) {
                    Order::Right => return Order::Right,
                    Order::NotRight => return Order::NotRight,
                    Order::Continue => {}
                }
            }
            (Value::List(left_list), Value::List(right_list)) => {
                match is_right_order(left_list, right_list) {
                    Order::Right => return Order::Right,
                    Order::NotRight => return Order::NotRight,
                    Order::Continue => {}
                }
            }
        }
    }
}

struct List {
    values: Vec<Value>,
}

impl ToString for List {
    fn to_string(&self) -> String {
        let mut result = String::new();
        result.push('[');
        for (i, value) in self.values.iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            }
            result.push_str(&value.to_string());
        }
        result.push(']');
        result
    }
}

enum Value {
    List(List),
    Value(i32),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::List(list) => list.to_string(),
            Value::Value(value) => value.to_string(),
        }
    }
}

fn parse_input(input: &str) -> Vec<(List, List)> {
    let mut lines = input.lines();
    let mut result = Vec::new();
    loop {
        let Some(line) = lines.next() else {
            break;
        };
        let left_list = parse_line(line);

        let Some(line) = lines.next() else {
            break;
        };
        let right_list = parse_line(line);

        result.push((left_list, right_list));

        if lines.next().is_none() {
            break;
        }
    }

    result
}

fn parse_line(line: &str) -> List {
    let mut it = tokenize(line);
    it.next().unwrap(); // skip [
    parse_list(&mut it)
}

fn tokenize(line: &str) -> impl Iterator<Item = &str> {
    let mut result = Vec::new();
    for tok in line.trim().split(|c: char| c.is_whitespace() || c == ',') {
        if tok.starts_with("[") {
            let (left, right) = tok.split_at(1);
            result.push(left);
            result.extend(tokenize(right));
        } else if tok.ends_with("]") {
            let (left, right) = tok.split_at(tok.len() - 1);
            result.extend(tokenize(left));
            result.push(right);
        } else {
            result.push(tok);
        }
    }

    result.into_iter()
}

fn parse_list<'a>(mut it: &mut impl Iterator<Item = &'a str>) -> List {
    let mut values = Vec::new();
    while let Some(c) = it.next() {
        match c {
            "[" => {
                let list = parse_list(it);
                values.push(Value::List(list));
            }
            "]" => {
                return List { values };
            }
            _ => {
                let Ok(value) = c.trim().parse() else {
                    continue;
                };
                values.push(Value::Value(value));
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"[1,1,3,1,1]
    [1,1,5,1,1]
    
    [[1],[2,3,4]]
    [[1],4]
    
    [9]
    [[8,7,6]]
    
    [[4,4],4,4]
    [[4,4],4,4,4]
    
    [7,7,7,7]
    [7,7,7]
    
    []
    [3]
    
    [[[]]]
    [[]]
    
    [1,[2,[3,[4,[5,6,7]]]],8,9]
    [1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    #[test]
    fn test_parse_line() {
        let line = "[1, 2, 3]";
        let list = parse_line(line);
        assert_eq!(list.to_string(), "[1, 2, 3]");

        let line = "[1, 2, [[3], [4,5]]]";
        let list = parse_line(line);
        assert_eq!(list.to_string(), "[1, 2, [[3], [4, 5]]]");
    }

    #[test]
    fn test_right_order() {
        let left = parse_line("[1, 2, 3]");
        let right = parse_line("[1, 2, 3]");
        assert_eq!(is_right_order(&left, &right), Order::Continue);

        let left = parse_line("[[1], [2, 3, 4]]");
        let right = parse_line("[[1], 4]");
        assert_eq!(is_right_order(&left, &right), Order::Right);

        let left = parse_line("[[[[3, 5], [8, 2, 9, 7], [4, 5], [2]]], [10, []], [], [], []]");
        let right = parse_line("[[[[0, 9, 3, 7], 2, 1, [], [6]]]]");
        assert_eq!(is_right_order(&left, &right), Order::NotRight);

        let left = parse_line("[[[1]], 2]");
        let right = parse_line("[[1], 2]");
        assert_eq!(is_right_order(&left, &right), Order::Continue);
    }

    #[test]
    fn test_part1_sample() {
        let answer = solve_part1(SAMPLE_INPUT);
        assert_eq!(answer, 13);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day_13.txt");
        let answer = solve_part1(input);
        assert_eq!(answer, 5684);
    }

    #[test]
    fn test_part2_sample() {
        let answer = solve_part2(SAMPLE_INPUT);
        assert_eq!(answer, 140);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/day_13.txt");
        let answer = solve_part2(input);
        assert_eq!(answer, 22932);
    }
}
