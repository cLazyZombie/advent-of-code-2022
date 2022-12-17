#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub fn solve_part1(input: &str) -> u64 {
    simulate(input, 2021)
}

pub fn solve_part2(input: &str) -> u32 {
    0
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
        // chamber.print(Some(&rock));

        while let Some(&dir) = jets.next() {
            chamber.move_rock(&mut rock, dir);
            // chamber.print(Some(&rock));
            if !chamber.move_down_rock(&mut rock) {
                chamber.land_rock(rock);
                break;
            }
            // chamber.print(Some(&rock));
        }
        // chamber.print(None);

        if idx == count as usize {
            return chamber.height();
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
}

impl Chamber {
    pub fn new() -> Self {
        Self { spaces: Vec::new() }
    }

    fn create_rock(&self, rock_type: RockType) -> Rock {
        let height = rock_type.height();
        let x = 2;
        let y = self.height() + 3;
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
        if rock.y == 0 {
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
        if self.spaces.len() < height as usize {
            self.spaces
                .resize(height as usize, [false; CHAMBER_WIDTH as usize]);
        }

        for (x, y) in rock.positions() {
            self.spaces[y as usize][x as usize] = true;
        }
    }

    fn height(&self) -> u64 {
        self.spaces.len() as u64
    }

    fn is_empty(&self, rock: &Rock) -> bool {
        for (x, y) in rock.positions() {
            if let Some(row) = self.spaces.get(y as usize) {
                if row[x as usize] {
                    return false;
                }
            }
        }
        true
    }

    fn print(&self, rock: Option<&Rock>) {
        let max_y = if let Some(rock) = rock {
            self.height().max((rock.y + rock.rock_type.height()) as u64)
        } else {
            self.height()
        };

        for y in (0..max_y).rev() {
            print!("|");
            for x in 0..CHAMBER_WIDTH {
                if let Some(rock) = rock {
                    if rock.positions().contains(&(x, y)) {
                        print!("@");
                        continue;
                    }
                }
                if let Some(row) = self.spaces.get(y as usize) {
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
        println!("+-------+");
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
    fn test_part2() {
        let input = include_str!("../input/day_17.txt");
        assert_eq!(solve_part2(input), 0);
    }

    #[test]
    #[ignore]
    fn test_part2_sample() {
        let answer = solve_part2(SAMPLE_INPUT);
        assert_eq!(answer, 0);
    }
}
