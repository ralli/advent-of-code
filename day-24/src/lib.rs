use std::fmt;
use std::fmt::Formatter;

use nom::character::complete::{line_ending, one_of};
use nom::multi::{many1, separated_list1};
use nom::IResult;

#[derive(Debug)]
pub struct Board {
    cells: Vec<Cell>,
    pub width: i32,
    pub height: i32,
}

impl Board {
    pub fn at_minute(&self, minute: i32) -> Self {
        let next_cells: Vec<Cell> = self
            .cells
            .iter()
            .map(|cell| cell.at_minute(minute, self.width, self.height))
            .collect();
        Board {
            cells: next_cells,
            width: self.width,
            height: self.height,
        }
    }

    pub fn get(&self, row: i32, col: i32) -> Option<CellType> {
        self.cells
            .iter()
            .find(|cell| cell.position.row == row && cell.position.col == col)
            .map(|cell| cell.value)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                match self.get(row, col) {
                    None => write!(f, ".")?,
                    Some(value) => write!(f, "{}", value)?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CellType {
    Wall,
    Blizzard(Direction),
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CellType::Wall => write!(f, "#"),
            CellType::Blizzard(direction) => write!(f, "{}", direction),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Direction::*;
        let c = match self {
            Up => '^',
            Down => 'v',
            Left => '<',
            Right => '>',
        };
        write!(f, "{}", c)
    }
}

impl Direction {
    pub fn delta(&self) -> (i32, i32) {
        use Direction::*;
        match self {
            Up => (-1, 0),
            Down => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
        }
    }

    pub fn delta_at(&self, minute: i32) -> (i32, i32) {
        let (row, col) = self.delta();
        (row * minute, col * minute)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub row: i32,
    pub col: i32,
}

impl Position {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

impl Position {
    pub fn plus(&self, delta: (i32, i32)) -> Position {
        Position {
            row: self.row + delta.0,
            col: self.col + delta.1,
        }
    }

    pub fn wrapped(&self, width: i32, height: i32) -> Position {
        Position {
            row: (self.row - 1).rem_euclid(height - 2) + 1,
            col: (self.col - 1).rem_euclid(width - 2) + 1,
        }
    }

    pub fn at_minute(
        &self,
        minute: i32,
        direction: Direction,
        width: i32,
        height: i32,
    ) -> Position {
        let delta = direction.delta_at(minute);
        self.plus(delta).wrapped(width, height)
    }
}

#[derive(Debug, Copy, Clone)]
struct Cell {
    position: Position,
    value: CellType,
}

impl Cell {
    pub fn at_minute(&self, minute: i32, width: i32, height: i32) -> Cell {
        match self.value {
            CellType::Wall => *self,
            CellType::Blizzard(direction) => Cell {
                position: self.position.at_minute(minute, direction, width, height),
                value: self.value,
            },
        }
    }
}

pub fn board(input: &str) -> IResult<&str, Board> {
    let (input, lines) = separated_list1(line_ending, many1(one_of(".#<>^v")))(input)?;
    let mut result = Vec::new();
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    for (row, line) in lines.into_iter().enumerate() {
        for (col, c) in line.into_iter().enumerate() {
            let position = Position::new(row as i32, col as i32);
            match c {
                '.' => {}
                '#' => result.push(Cell {
                    position,
                    value: CellType::Wall,
                }),
                '^' => result.push(Cell {
                    position,
                    value: CellType::Blizzard(Direction::Up),
                }),
                'v' => result.push(Cell {
                    position,
                    value: CellType::Blizzard(Direction::Down),
                }),
                '<' => result.push(Cell {
                    position,
                    value: CellType::Blizzard(Direction::Left),
                }),
                '>' => result.push(Cell {
                    position,
                    value: CellType::Blizzard(Direction::Right),
                }),
                _ => unreachable!("{}", c),
            }
        }
    }

    Ok((
        input,
        Board {
            cells: result,
            width,
            height,
        },
    ))
}
