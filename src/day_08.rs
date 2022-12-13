pub fn solve_part1(input: &str) -> u32 {
    let grid = load_grid(input);

    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if is_visible(&grid, x, y) {
                sum += 1;
            }
        }
    }

    sum
}

pub fn solve_part2(input: &str) -> u32 {
    let grid = load_grid(input);

    dbg!(get_score(&grid, 2, 3));

    let mut highest = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let score = get_score(&grid, x, y);
            if score > highest {
                highest = score;
            }
        }
    }

    highest
}

fn load_grid(input: &str) -> Vec<Vec<u8>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            if let Ok(h) = c.to_string().parse::<u8>() {
                row.push(h);
            }
        }
        grid.push(row);
    }
    grid
}

fn is_visible(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    if x == 0 || y == 0 || x == grid[0].len() - 1 || y == grid.len() - 1 {
        return true;
    }

    let cur = grid[y][x];

    // check left
    let mut visible = true;
    for i in 0..x {
        if grid[y][i] >= cur {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }

    // check right
    let mut visible = true;
    for i in x + 1..grid[0].len() {
        if grid[y][i] >= cur {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }

    // check up
    let mut visible = true;
    for i in 0..y {
        if grid[i][x] >= cur {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }

    // check down
    let mut visible = true;
    // for i in y + 1..grid.len() {
    for row in grid.iter().skip(y + 1) {
        if row[x] >= cur {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }

    false
}

fn get_score(grid: &[Vec<u8>], x: usize, y: usize) -> u32 {
    let cur = grid[y][x];

    // check left
    let mut left_score = 0;
    for i in (0..x).rev() {
        left_score += 1;

        let h = grid[y][i];
        if h >= cur {
            break;
        }
    }

    // check right
    let mut right_score = 0;
    for i in x + 1..grid[0].len() {
        right_score += 1;

        let h = grid[y][i];
        if h >= cur {
            break;
        }
    }

    // check up
    let mut up_score = 0;

    for i in (0..y).rev() {
        up_score += 1;

        let h = grid[i][x];
        if h >= cur {
            break;
        }
    }

    // check down
    let mut down_score = 0;

    // for i in y + 1..grid.len() {
    for row in grid.iter().skip(y + 1) {
        down_score += 1;

        let h = row[x];
        if h >= cur {
            break;
        }
    }

    left_score * right_score * up_score * down_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = r#"30373
        25512
        65332
        33549
        35390"#;
        let answer = solve_part1(input);
        assert_eq!(answer, 21);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day_08.txt");
        let answer = solve_part1(input);
        assert_eq!(answer, 1681);
    }

    #[test]
    fn test_part2_sample() {
        let input = r#"30373
        25512
        65332
        33549
        35390"#;
        let answer = solve_part2(input);
        assert_eq!(answer, 8);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/day_08.txt");
        let answer = solve_part2(input);
        assert_eq!(answer, 201684);
    }
}
