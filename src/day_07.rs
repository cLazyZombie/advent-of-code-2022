use std::str::Lines;

pub fn solve_part1(input: &str) -> u32 {
    let mut lines = input.lines();
    lines.next(); // skip dir

    let mut sum_atmost = 0;
    let _sum = get_directory_size(&mut lines, &mut sum_atmost);
    sum_atmost
}

fn get_directory_size<'a>(lines: &'a mut Lines, sum_atmost: &mut u32) -> u32 {
    let mut sum = 0;
    while let Some(line) = lines.next() {
        let (cmd1, cmd2, cmd3) = parse_line(&line);
        match (cmd1, cmd2, cmd3) {
            ("$", "ls", _) => {}
            ("$", "cd", Some("..")) => {
                break;
            }
            ("$", "cd", Some(_dir)) => {
                let sub_sum = get_directory_size(lines, sum_atmost);
                sum += sub_sum;
            }
            ("dir", _dir, _) => {
                continue;
            }
            (size, _name, None) => {
                let size: u32 = size.parse().unwrap();
                sum += size;
            }
            _ => {
                panic!("Unknown command: {}", line);
            }
        }
    }

    if sum <= 100_000 {
        *sum_atmost += sum;
    }

    sum
}

fn parse_line(line: &str) -> (&str, &str, Option<&str>) {
    let mut iter = line.trim().split_whitespace();
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();
    let third = iter.next();
    (first, second, third)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;
        let answer = solve_part1(input);
        assert_eq!(answer, 95437);
    }

    #[test]
    fn test_part1_edge_case() {
        let input = r#"$ cd /
$ ls
dir a
1 root.txt
$ cd a
$ ls
2 a1.txt
dir b
3 a2.txt
dir c
$ cd b
$ ls
4 b.txt 
$ cd ..
$ cd c
$ ls
5 c.txt
$ cd ..
$ cd .."#;
        let answer = solve_part1(input);
        assert_eq!(answer, 38);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day_07.txt");
        let answer = solve_part1(input);
        assert_eq!(answer, 1743217);
    }
}
