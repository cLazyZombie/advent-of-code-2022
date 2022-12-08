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

fn load_grid(input: &str) -> Vec<Vec<u8>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            if let Ok(h) = (&c.to_string()).parse::<u8>() {
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
    for i in y + 1..grid.len() {
        if grid[i][x] >= cur {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }

    false
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
        assert_eq!(answer, 1681);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day_08.txt");
        let answer = solve_part1(input);
        assert_eq!(answer, 256);
    }
}
