use std::collections::{BTreeSet, VecDeque};
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let grid: Grid = input.parse()?;
    let initial = Position {
        row: 0,
        col: 0,
        direction: Direction::Right,
    };
    let mut q = VecDeque::from([initial]);
    let mut positions: BTreeSet<(i32, i32)> = BTreeSet::new();
    let mut visited: BTreeSet<Position> = BTreeSet::new();

    while let Some(pos) = q.pop_front() {
        positions.insert((pos.row, pos.col));
        let cell = grid.get(pos.row, pos.col);
        match cell {
            '.' => {
                let (dr, dc) = pos.direction.delta();
                let next_pos = Position {
                    row: pos.row + dr,
                    col: pos.col + dc,
                    direction: pos.direction,
                };
                if grid.is_valid_position(&next_pos) && visited.insert(next_pos) {
                    q.push_back(next_pos);
                }
            }
            '/' => {
                let next_direction = match pos.direction {
                    Direction::Right => Direction::Up,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                };
                let (dr, dc) = next_direction.delta();
                let next_pos = Position {
                    row: pos.row + dr,
                    col: pos.col + dc,
                    direction: next_direction,
                };
                if grid.is_valid_position(&next_pos) && visited.insert(next_pos) {
                    q.push_back(next_pos);
                }
            }
            '\\' => {
                let next_direction = match pos.direction {
                    Direction::Up => Direction::Left,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                };
                let (dr, dc) = next_direction.delta();
                let next_pos = Position {
                    row: pos.row + dr,
                    col: pos.col + dc,
                    direction: next_direction,
                };
                if grid.is_valid_position(&next_pos) && visited.insert(next_pos) {
                    q.push_back(next_pos);
                }
            }
            '-' => {
                if pos.direction == Direction::Left || pos.direction == Direction::Right {
                    let (dr, dc) = pos.direction.delta();
                    let next_pos = Position {
                        row: pos.row + dr,
                        col: pos.col + dc,
                        direction: pos.direction,
                    };
                    if grid.is_valid_position(&next_pos) && visited.insert(next_pos) {
                        q.push_back(next_pos);
                    }
                } else {
                    let next_direction = Direction::Left;
                    let (dr, dc) = next_direction.delta();
                    let next_pos = Position {
                        row: pos.row + dr,
                        col: pos.col + dc,
                        direction: next_direction,
                    };
                    if grid.is_valid_position(&next_pos) && visited.insert(next_pos) {
                        q.push_back(next_pos);
                    }

                    let next_direction = Direction::Right;
                    let (dr, dc) = next_direction.delta();
                    let next_pos = Position {
                        row: pos.row + dr,
                        col: pos.col + dc,
                        direction: next_direction,
                    };
                    if grid.is_valid_position(&next_pos) && visited.insert(next_pos) {
                        q.push_back(next_pos);
                    }
                }
            }
            '|' => {
                if pos.direction == Direction::Up || pos.direction == Direction::Down {
                    let (dr, dc) = pos.direction.delta();
                    let next_pos = Position {
                        row: pos.row + dr,
                        col: pos.col + dc,
                        direction: pos.direction,
                    };
                    if grid.is_valid_position(&next_pos) && visited.insert(next_pos) {
                        q.push_back(next_pos);
                    }
                } else {
                    let next_direction = Direction::Up;
                    let (dr, dc) = next_direction.delta();
                    let next_pos = Position {
                        row: pos.row + dr,
                        col: pos.col + dc,
                        direction: next_direction,
                    };
                    if grid.is_valid_position(&next_pos) && visited.insert(next_pos) {
                        q.push_back(next_pos);
                    }

                    let next_direction = Direction::Down;
                    let (dr, dc) = next_direction.delta();
                    let next_pos = Position {
                        row: pos.row + dr,
                        col: pos.col + dc,
                        direction: next_direction,
                    };
                    if grid.is_valid_position(&next_pos) && visited.insert(next_pos) {
                        q.push_back(next_pos);
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    // println!("{grid}");
    // for row in 0..grid.height {
    //     for col in 0..grid.width {
    //         if positions.contains(&(row, col)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    let result = positions.len();
    Ok(result)
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Position {
    row: i32,
    col: i32,
    direction: Direction,
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<char>>,
    width: i32,
    height: i32,
}

impl Grid {
    fn is_valid_position(&self, pos: &Position) -> bool {
        pos.row >= 0 && pos.row < self.height && pos.col >= 0 && pos.col < self.width
    }

    fn get(&self, row: i32, col: i32) -> char {
        self.cells[row as usize][col as usize]
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let height = cells.len() as i32;
        let width = cells.iter().next().map(|s| s.len()).unwrap_or_default() as i32;
        Ok(Grid {
            cells,
            width,
            height,
        })
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                write!(f, "{}", self.get(row, col))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }

    fn right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 46;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 145;
        assert_eq!(result, expected);
        Ok(())
    }
}
