#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub fn solve_part1(input: &str) -> u64 {
    simulate(input, 2022)
}

pub fn solve_part2(input: &str) -> u64 {
    simulate2(input, 1000000000000_usize)
}

fn simulate(input: &str, count: u64) -> u64 {
    let jets = load_input(input);
    let rocks = [
        RockType::Horizontal,
        RockType::Cross,
        RockType::LMirror,
        RockType::I,
        RockType::Square,
    ];
    let mut jets = jets.iter().cycle();
    let mut chamber = Chamber::new();
    for (idx, &rock_type) in rocks.iter().cycle().enumerate() {
        let mut rock = chamber.create_rock(rock_type);

        while let Some(&dir) = jets.next() {
            chamber.move_rock(&mut rock, dir);
            // chamber.print(Some(&rock));
            if !chamber.move_down_rock(&mut rock) {
                chamber.land_rock(rock);
                break;
            }
        }

        if idx + 1 == count as usize {
            return chamber.height() as u64;
        }
    }

    unreachable!()
}

#[derive(Debug)]
struct Pattern {
    rock_idx: usize,
    jet_idx: usize,
    spaces: Vec<[bool; CHAMBER_WIDTH as usize]>,
}

fn simulate2(input: &str, count: usize) -> u64 {
    let jets = load_input(input);
    let rocks = [
        RockType::Horizontal,
        RockType::Cross,
        RockType::LMirror,
        RockType::I,
        RockType::Square,
    ];
    let cycle = num::integer::lcm(rocks.len(), jets.len());
    // let cycle = rocks.len() * jets.len();
    let mut pattern: Vec<Vec<(Pattern, usize, usize)>> = Vec::new();
    pattern.resize_with(cycle, Vec::new);

    let mut jets_it = jets.iter().cycle();
    let mut chamber = Chamber::new();
    let mut idx = 0;
    let mut jet_idx = 0;
    let mut pattern_found = false;
    for &rock_type in rocks.iter().cycle() {
        idx += 1;

        let mut rock = chamber.create_rock(rock_type);

        while let Some(&dir) = jets_it.next() {
            jet_idx += 1;

            chamber.move_rock(&mut rock, dir);
            if !chamber.move_down_rock(&mut rock) {
                chamber.land_rock(rock);
                chamber.remove_after_blocking();
                break;
            }
        }

        if !pattern_found && (idx - 1) % cycle == 0 {
            let cur_pattern = Pattern {
                rock_idx: idx,
                jet_idx: jet_idx,
                spaces: chamber.spaces.clone(),
            };

            let target_patterns = &mut pattern[(idx - 1) % cycle];

            for (p, base, height) in target_patterns.iter() {
                if p.rock_idx % rocks.len() == cur_pattern.rock_idx % rocks.len()
                    && p.jet_idx % jets.len() == cur_pattern.jet_idx % jets.len()
                    && p.spaces == cur_pattern.spaces
                {
                    let r = (count - p.rock_idx) % (idx - p.rock_idx);
                    let c = (count - p.rock_idx) / (idx - p.rock_idx);
                    println!(
                        "pattern found at {}, prev: {}. cycle: {}",
                        idx, p.rock_idx, cycle
                    );
                    println!(
                        "prev base: {}, cur base: {}, c: {}, r: {}",
                        base, chamber.base, c, r
                    );
                    println!("prev height: {}, cur height: {}", height, chamber.height());
                    chamber.base = *base + c * (chamber.base - *base);

                    idx = count - r;
                    pattern_found = true;
                    break;
                }
            }
            if !pattern_found {
                target_patterns.push((cur_pattern, chamber.base, chamber.height()));
            }
        }

        if idx == count {
            return chamber.height() as u64;
        }
    }

    unreachable!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RockType {
    Horizontal,
    Cross,
    LMirror,
    I,
    Square,
}

impl RockType {
    fn height(&self) -> u64 {
        match *self {
            RockType::Horizontal => 1,
            RockType::Cross => 3,
            RockType::LMirror => 3,
            RockType::I => 4,
            RockType::Square => 2,
        }
    }

    fn width(&self) -> u64 {
        match *self {
            RockType::Horizontal => 4,
            RockType::Cross => 3,
            RockType::LMirror => 3,
            RockType::I => 1,
            RockType::Square => 2,
        }
    }

    fn positions(&self) -> Vec<(u64, u64)> {
        match *self {
            RockType::Horizontal => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            RockType::Cross => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            RockType::LMirror => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            RockType::I => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            RockType::Square => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rock {
    rock_type: RockType,
    x: u64,
    y: u64,
}

impl Rock {
    fn positions(&self) -> Vec<(u64, u64)> {
        self.rock_type
            .positions()
            .into_iter()
            .map(|p| (p.0 + self.x, p.1 + self.y))
            .collect()
    }
}

const CHAMBER_WIDTH: u64 = 7;
#[derive(Debug)]
struct Chamber {
    spaces: Vec<[bool; CHAMBER_WIDTH as usize]>,
    base: usize,
}

impl Chamber {
    pub fn new() -> Self {
        Self {
            spaces: Vec::new(),
            base: 0,
        }
    }

    fn create_rock(&self, rock_type: RockType) -> Rock {
        let height = rock_type.height();
        let x = 2;
        let y = (self.height() + 3) as u64;
        Rock { rock_type, x, y }
    }

    fn move_rock(&self, rock: &mut Rock, dir: Dir) -> bool {
        if dir == Dir::Left && rock.x == 0 {
            return false;
        }

        if dir == Dir::Right && rock.x + rock.rock_type.width() >= CHAMBER_WIDTH {
            return false;
        }

        let mut moved = *rock;
        match dir {
            Dir::Left => moved.x -= 1,
            Dir::Right => moved.x += 1,
        }

        if self.is_empty(&moved) {
            *rock = moved;
            return true;
        }

        false
    }

    fn move_down_rock(&self, rock: &mut Rock) -> bool {
        if rock.y as usize == self.base {
            return false;
        }

        let mut moved = *rock;
        moved.y -= 1;
        if self.is_empty(&moved) {
            *rock = moved;
            return true;
        }

        false
    }

    fn land_rock(&mut self, rock: Rock) {
        // check enough height
        let height = rock.y + rock.rock_type.height();
        if (self.height() as u64) < height {
            let need = height as usize - self.height() as usize;
            let total = self.spaces.len() + need;
            self.spaces.resize(total, [false; CHAMBER_WIDTH as usize]);
        }

        for (x, y) in rock.positions() {
            if (y as usize) < self.base {
                assert!(y as usize >= self.base, "y: {}, base: {}", y, self.base);
            }
            self.spaces[y as usize - self.base][x as usize] = true;
        }
    }

    fn height(&self) -> usize {
        self.spaces.len() + self.base
    }

    fn is_empty(&self, rock: &Rock) -> bool {
        for (x, y) in rock.positions() {
            if (y as usize) < self.base {
                continue;
            }

            if let Some(row) = self.spaces.get(y as usize - self.base) {
                if row[x as usize] {
                    return false;
                }
            }
        }
        true
    }

    fn remove_after_blocking(&mut self) {
        let mut spaces = self.spaces.clone();
        spaces.push([false; CHAMBER_WIDTH as usize]);

        let y = spaces.len() - 1;
        let mut stack = Vec::new();
        stack.push((0, y));
        spaces[y][0] = true;

        let mut smallest_y = y;

        while let Some((x, y)) = stack.pop() {
            smallest_y = smallest_y.min(y);

            // left
            if x > 0 && !spaces[y][x - 1] {
                spaces[y][x - 1] = true;
                stack.push((x - 1, y));
            }
            // right
            if x < CHAMBER_WIDTH as usize - 1 && !spaces[y][x + 1] {
                spaces[y][x + 1] = true;
                stack.push((x + 1, y));
            }
            // down
            if y > 0 && !spaces[y - 1][x] {
                spaces[y - 1][x] = true;
                stack.push((x, y - 1));
            }
            // up
            if y < spaces.len() - 1 && !spaces[y + 1][x] {
                spaces[y + 1][x] = true;
                stack.push((x, y + 1));
            }
        }

        let base = smallest_y.max(0);
        self.spaces = self.spaces.split_off(base);
        self.base += base;
    }

    fn print(&self, rock: Option<&Rock>) {
        let max_y = if let Some(rock) = rock {
            self.height()
                .max((rock.y + rock.rock_type.height()) as usize)
        } else {
            self.height()
        };

        for y in (self.base..max_y as usize).rev() {
            print!("[{:5}] |", y);
            for x in 0..CHAMBER_WIDTH {
                if let Some(rock) = rock {
                    if rock.positions().contains(&(x, y as u64)) {
                        print!("@");
                        continue;
                    }
                }
                if let Some(row) = self.spaces.get(y as usize - self.base) {
                    if row[x as usize] {
                        print!("#");
                    } else {
                        print!(".");
                    }
                } else {
                    print!(".");
                }
            }
            println!("|");
        }
        println!("[ base] +-------+");
        println!("");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
}

fn load_input(input: &str) -> Vec<Dir> {
    input
        .as_bytes()
        .iter()
        .map(|c| match *c {
            b'<' => Dir::Left,
            b'>' => Dir::Right,
            _ => panic!("Invalid input"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    // const SAMPLE_INPUT: &str = ">>><<><>><<<>><>>><<<>><<<<<<<<<<><<<><<<>><>><<>>>><<><<<";

    #[test]
    fn test_load() {
        let input = "<><";
        let dirs = load_input(input);
        assert_eq!(dirs, vec![Dir::Left, Dir::Right, Dir::Left]);
    }

    #[test]
    fn test_create_rock() {
        let mut chamber = Chamber::new();
        let rock = chamber.create_rock(RockType::Square);
        assert_eq!(rock.x, 2);
        assert_eq!(rock.y, 3);
        assert_eq!(rock.rock_type, RockType::Square);
        assert_eq!(rock.positions(), vec![(2, 3), (3, 3), (2, 4), (3, 4)]);
    }

    #[test]
    fn test_rock_positions() {
        let mut chamber = Chamber::new();
        let mut rock = chamber.create_rock(RockType::Square);
        assert_eq!(rock.positions(), vec![(2, 3), (3, 3), (2, 4), (3, 4)]);

        chamber.move_down_rock(&mut rock);
        assert_eq!(rock.positions(), vec![(2, 2), (3, 2), (2, 3), (3, 3)]);
    }

    #[test]
    fn test_land_rock() {
        let mut chamber = Chamber::new();
        let rock = chamber.create_rock(RockType::Square);
        chamber.land_rock(rock);
        assert_eq!(chamber.height(), 5);
    }

    #[test]
    fn test_move_rock() {
        let mut chamber = Chamber::new();
        let mut rock = chamber.create_rock(RockType::Square);

        for _ in 0..3 {
            assert_eq!(chamber.move_rock(&mut rock, Dir::Right), true);
        }
        assert_eq!(chamber.move_rock(&mut rock, Dir::Right), false);

        for _ in 0..3 {
            assert_eq!(chamber.move_down_rock(&mut rock), true);
        }
        assert_eq!(chamber.move_down_rock(&mut rock), false);

        chamber.land_rock(rock);
        assert_eq!(chamber.height(), 2);

        // create cross rock and move right most as possible
        let mut rock = chamber.create_rock(RockType::Cross);

        for _ in 0..2 {
            assert_eq!(chamber.move_rock(&mut rock, Dir::Left), true);
        }
        assert_eq!(chamber.move_rock(&mut rock, Dir::Left), false);
    }

    #[test]
    fn test_remove_after_blocking() {
        let mut chamber = Chamber::new();
        chamber.remove_after_blocking();
        assert_eq!(chamber.base, 0);
        assert_eq!(chamber.height(), 0);

        let mut rock = chamber.create_rock(RockType::Horizontal);
        chamber.land_rock(rock);
        assert_eq!(chamber.height(), 4);

        for x in 0..CHAMBER_WIDTH {
            chamber.spaces[1][x as usize] = true;
        }
        chamber.spaces[2][0] = true;
        // chamber.print(None);

        chamber.remove_after_blocking();
        assert_eq!(chamber.base, 2);
        assert_eq!(chamber.height(), 4);
        // chamber.print(None);

        let mut rock = chamber.create_rock(RockType::Horizontal);
        // chamber.print(Some(&rock));
        assert_eq!((rock.x, rock.y), (2, 7));
        for _ in 0..3 {
            assert_eq!(chamber.move_down_rock(&mut rock), true);
        }
        assert_eq!(chamber.move_down_rock(&mut rock), false);
        // chamber.print(Some(&rock));
        chamber.land_rock(rock);
        assert_eq!(chamber.height(), 5);
        // chamber.print(None);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day_17.txt");
        assert_eq!(solve_part1(input), 3191);
    }

    #[test]
    fn test_part1_sample() {
        let answer = solve_part1(SAMPLE_INPUT);
        assert_eq!(answer, 3068);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let input = include_str!("../input/day_17.txt");
        assert_eq!(solve_part2(input), 1572093023267);
    }

    #[test]
    fn test_part2_sample() {
        let answer = solve_part2(SAMPLE_INPUT);
        assert_eq!(answer, 1514285714288);
    }
}
