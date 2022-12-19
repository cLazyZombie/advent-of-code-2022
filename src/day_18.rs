pub fn solve_part1(input: &str) -> u32 {
    let input = parse_input(input);
    let (max_x, max_y, max_z) = input
        .iter()
        .fold((0, 0, 0), |(max_x, max_y, max_z), (x, y, z)| {
            (max_x.max(*x), max_y.max(*y), max_z.max(*z))
        });

    let mut grid = vec![vec![vec![false; max_z + 1]; max_y + 1]; max_x + 1];

    for (x, y, z) in input {
        grid[x as usize][y as usize][z as usize] = true;
    }

    let mut count = 0;
    for x in 0..=max_x {
        for y in 0..=max_y {
            for z in 0..=max_z {
                if !grid[x][y][z] {
                    continue;
                }

                // check -x
                if x == 0 || !grid[x - 1][y][z] {
                    count += 1;
                }
                // check +x
                if x == max_x || !grid[x + 1][y][z] {
                    count += 1;
                }
                // check -y
                if y == 0 || !grid[x][y - 1][z] {
                    count += 1;
                }
                // check +y
                if y == max_y || !grid[x][y + 1][z] {
                    count += 1;
                }
                // check -z
                if z == 0 || !grid[x][y][z - 1] {
                    count += 1;
                }
                // check +z
                if z == max_z || !grid[x][y][z + 1] {
                    count += 1;
                }
            }
        }
    }
    count
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cube {
    Outside,
    Inside,
    Solid,
}

fn map_inside(grid: Vec<Vec<Vec<bool>>>) -> Vec<Vec<Vec<Cube>>> {
    let mut visited = vec![vec![vec![false; grid[0][0].len()]; grid[0].len()]; grid.len()];
    let mut result = vec![vec![vec![Cube::Inside; grid[0][0].len()]; grid[0].len()]; grid.len()];

    let mut stack = Vec::new();
    for y in 0..grid[0].len() {
        for z in 0..grid[0][0].len() {
            stack.push((0, y, z));
            stack.push((grid.len() - 1, y, z));
        }
    }

    for x in 0..grid.len() {
        for z in 0..grid[0][0].len() {
            stack.push((x, 0, z));
            stack.push((x, grid[0].len() - 1, z));
        }
    }

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            stack.push((x, y, 0));
            stack.push((x, y, grid[0][0].len() - 1));
        }
    }

    while !stack.is_empty() {
        let (x, y, z) = stack.pop().unwrap();

        if visited[x][y][z] {
            continue;
        }
        visited[x][y][z] = true;

        if grid[x][y][z] {
            result[x][y][z] = Cube::Solid;
            continue;
        }

        result[x][y][z] = Cube::Outside;

        // check -x
        if x > 0 {
            stack.push((x - 1, y, z));
        }
        // check +x
        if x < grid.len() - 1 {
            stack.push((x + 1, y, z));
        }
        // check -y
        if y > 0 {
            stack.push((x, y - 1, z));
        }
        // check +y
        if y < grid[0].len() - 1 {
            stack.push((x, y + 1, z));
        }
        // check -z
        if z > 0 {
            stack.push((x, y, z - 1));
        }
        // check +z
        if z < grid[0][0].len() - 1 {
            stack.push((x, y, z + 1));
        }
    }

    result
}

pub fn solve_part2(input: &str) -> u32 {
    let input = parse_input(input);
    let (max_x, max_y, max_z) = input
        .iter()
        .fold((0, 0, 0), |(max_x, max_y, max_z), (x, y, z)| {
            (max_x.max(*x), max_y.max(*y), max_z.max(*z))
        });

    let mut grid = vec![vec![vec![false; max_z + 3]; max_y + 3]; max_x + 3];

    for (x, y, z) in input {
        grid[x + 1][y + 1][z + 1] = true;
    }

    let grid = map_inside(grid);

    let mut count = 0;
    for x in 1..=(max_x + 1) {
        for y in 1..=(max_y + 1) {
            for z in 1..=(max_z + 1) {
                if grid[x][y][z] != Cube::Solid {
                    continue;
                }

                // check -x
                if grid[x - 1][y][z] == Cube::Outside {
                    count += 1;
                }
                // check +x
                if grid[x + 1][y][z] == Cube::Outside {
                    count += 1;
                }
                // check -y
                if grid[x][y - 1][z] == Cube::Outside {
                    count += 1;
                }
                // check +y
                if grid[x][y + 1][z] == Cube::Outside {
                    count += 1;
                }
                // check -z
                if grid[x][y][z - 1] == Cube::Outside {
                    count += 1;
                }
                // check +z
                if grid[x][y][z + 1] == Cube::Outside {
                    count += 1;
                }
            }
        }
    }
    count
}

fn parse_input(input: &str) -> Vec<(usize, usize, usize)> {
    let mut result = Vec::new();
    for line in input.lines() {
        let mut xyz = line.split(',').map(|s| s.trim().parse().unwrap());
        let (x, y, z) = (
            xyz.next().unwrap(),
            xyz.next().unwrap(),
            xyz.next().unwrap(),
        );
        result.push((x, y, z));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;

    #[test]
    fn test_parse_sample() {
        let valves = parse_input(SAMPLE_INPUT);
        assert_eq!(valves.len(), 13);
        assert_eq!(valves[0], (2, 2, 2));
        assert_eq!(valves[0], (1, 2, 2));
    }

    #[test]
    fn test_part1_sample() {
        let answer = solve_part1(SAMPLE_INPUT);
        assert_eq!(answer, 64);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day_18.txt");
        let answer = solve_part1(input);
        assert_eq!(answer, 4460);
    }

    #[test]
    fn test_part2_sample() {
        let answer = solve_part2(SAMPLE_INPUT);
        assert_eq!(answer, 58);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/day_18.txt");
        let answer = solve_part2(input);
        assert_eq!(answer, 2498);
    }
}
