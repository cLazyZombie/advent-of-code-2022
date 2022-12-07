use std::str::Lines;

pub fn solve_part1(input: &str) -> u32 {
    let mut lines = input.lines();
    lines.next(); // skip dir

    let mut sum_atmost = 0;
    let _sum = get_directory_size(&mut lines, &mut sum_atmost, &mut 0, 0);
    sum_atmost
}

pub fn solve_part2(input: &str) -> u32 {
    let mut lines = input.lines();
    lines.next(); // skip dir

    // pass 1 for sum total space
    let mut _sum_atmost = 0;
    let sum = get_directory_size(&mut lines, &mut _sum_atmost, &mut 0, 0);
    let cur_space = 70000000 - sum;
    let need_space = 30000000 - cur_space;

    // pass 2 for smallest space that can fit needed space
    let mut lines = input.lines();
    lines.next(); // skip dir
    let mut smallest = std::u32::MAX;

    let _sum = get_directory_size(&mut lines, &mut _sum_atmost, &mut smallest, need_space);

    smallest
}

fn get_directory_size<'a>(
    lines: &'a mut Lines,
    sum_atmost: &mut u32,
    smallest: &mut u32,
    need_space: u32,
) -> u32 {
    let mut sum = 0;
    while let Some(line) = lines.next() {
        let (cmd1, cmd2, cmd3) = parse_line(&line);
        match (cmd1, cmd2, cmd3) {
            ("$", "ls", _) => {}
            ("$", "cd", Some("..")) => {
                break;
            }
            ("$", "cd", Some(_dir)) => {
                let sub_sum = get_directory_size(lines, sum_atmost, smallest, need_space);
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

    if sum >= need_space && sum < *smallest {
        *smallest = sum;
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

    #[test]
    fn test_part2_sample() {
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
        let answer = solve_part2(input);
        assert_eq!(answer, 24933642);
    }
    #[test]
    fn test_part2() {
        let input = include_str!("../input/day_07.txt");
        let answer = solve_part2(input);
        assert_eq!(answer, 8319096);
    }
}
