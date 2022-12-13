use std::cmp::Ordering;

pub fn solve_part1(input: &str) -> i32 {
    let mut result = 0;
    let input = parse_input(input);
    for (idx, (left, right)) in pair_lists(&input).into_iter().enumerate() {
        let ordering = left.cmp(right);
        if ordering == Ordering::Less || ordering == Ordering::Equal {
            result += idx as i32 + 1;
        }
    }
    result
}

pub fn solve_part2(input: &str) -> i32 {
    let mut input: Vec<List> = parse_input(input);
    let div1 = parse_line("[[2]]");
    let div2 = parse_line("[[6]]");
    input.push(div1);
    input.push(div2);
    input.sort();

    let mut result = 1;

    for (idx, list) in input.iter().enumerate() {
        match list.to_string().as_str() {
            "[[2]]" | "[[6]]" => result *= idx as i32 + 1,
            _ => (),
        }
    }
    result
}

#[derive(Debug, PartialEq, Eq)]
struct List {
    values: Vec<Value>,
}

impl List {
    fn from_value(value: i32) -> Self {
        List {
            values: vec![Value::Value(value)],
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut left_it = self.values.iter();
        let mut right_it = other.values.iter();
        loop {
            let (left_value, right_value) = match (left_it.next(), right_it.next()) {
                (None, None) => return Ordering::Equal,
                (None, Some(_)) => return Ordering::Less,
                (Some(_), None) => return Ordering::Greater,
                (Some(left_value), Some(right_value)) => (left_value, right_value),
            };

            match (left_value, right_value) {
                (Value::Value(left_value), Value::Value(right_value)) => {
                    match left_value.cmp(right_value) {
                        Ordering::Equal => {}
                        ord => return ord,
                    }
                }
                (Value::Value(left_value), Value::List(right_list)) => {
                    let left_list = List::from_value(*left_value);
                    match left_list.cmp(right_list) {
                        Ordering::Equal => {}
                        ord => return ord,
                    }
                }
                (Value::List(left_list), Value::Value(right_value)) => {
                    let right_list = List::from_value(*right_value);
                    match left_list.cmp(&right_list) {
                        Ordering::Equal => {}
                        ord => return ord,
                    }
                }
                (Value::List(left_list), Value::List(right_list)) => {
                    match left_list.cmp(right_list) {
                        Ordering::Equal => {}
                        ord => return ord,
                    }
                }
            }
        }
    }
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

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
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

fn pair_lists(list_array: &[List]) -> Vec<(&List, &List)> {
    let mut result = Vec::new();
    for idx in 0..(list_array.len() / 2) {
        let left = &list_array[idx * 2];
        let right = &list_array[idx * 2 + 1];
        result.push((left, right));
    }
    result
}

fn parse_input(input: &str) -> Vec<List> {
    let mut result = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let list = parse_line(line);
        result.push(list);
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
        if tok.starts_with('[') {
            let (left, right) = tok.split_at(1);
            result.push(left);
            result.extend(tokenize(right));
        } else if tok.ends_with(']') {
            let (left, right) = tok.split_at(tok.len() - 1);
            result.extend(tokenize(left));
            result.push(right);
        } else {
            result.push(tok);
        }
    }

    result.into_iter()
}

fn parse_list<'a>(it: &mut impl Iterator<Item = &'a str>) -> List {
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
        assert_eq!(left.cmp(&right), Ordering::Equal);
        assert!(!left.lt(&right));

        let left = parse_line("[[1], [2, 3, 4]]");
        let right = parse_line("[[1], 4]");
        assert_eq!(left.cmp(&right), Ordering::Less);
        assert!(left.lt(&right));

        let left = parse_line("[[[[3, 5], [8, 2, 9, 7], [4, 5], [2]]], [10, []], [], [], []]");
        let right = parse_line("[[[[0, 9, 3, 7], 2, 1, [], [6]]]]");
        assert_eq!(left.cmp(&right), Ordering::Greater);
        assert!(!left.lt(&right));

        let left = parse_line("[[[1]], 2]");
        let right = parse_line("[[1], 2]");
        assert_eq!(left.cmp(&right), Ordering::Equal);
        assert_eq!(left.partial_cmp(&right), Some(Ordering::Equal));
        assert!(!left.lt(&right));
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
