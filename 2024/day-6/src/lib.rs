use anyhow::anyhow;
use nom::character::complete::{line_ending, one_of};
use nom::multi::{many1, separated_list1};
use nom::IResult;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fmt;

pub fn count_guard_positions(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    while grid.is_on_grid(grid.pos.row, grid.pos.col) {
        grid.step();
    }
    grid.positions.len()
}

pub fn count_obstructions(grid: &Grid) -> usize {
    let mut copy = grid.clone();
    while copy.is_on_grid(copy.pos.row, copy.pos.col) {
        copy.step();
    }
    let positions = copy.positions;
    let count = positions
        .par_iter()
        .filter(|p| p.row != grid.pos.row || p.col != grid.pos.col)
        .filter(|p| {
            let mut copy = grid.clone();
            copy.grid[p.row as usize][p.col as usize] = State::Occupied;
            while copy.is_on_grid(copy.pos.row, copy.pos.col) {
                if copy.step2() {
                    // true means, step2 detected a loop
                    return true;
                }
            }
            false
        })
        .count();
    count
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct Point {
    pub row: isize,
    pub col: isize,
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Ord)]
pub enum State {
    Empty,
    Occupied,
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn rotated_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (-1, 0),
            Direction::Left => (0, -1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub grid: Vec<Vec<State>>,
    pub width: isize,
    pub height: isize,
    pub pos: Point,
    pub direction: Direction,
    pub positions: HashSet<Point>,
    pub pos_directions: HashSet<(Point, Direction)>,
}

impl Grid {
    pub fn is_occupied(&self, row: isize, col: isize) -> bool {
        self.is_on_grid(row, col) && self.grid[row as usize][col as usize] == State::Occupied
    }

    pub fn is_on_grid(&self, row: isize, col: isize) -> bool {
        0 <= row && row < self.height && 0 <= col && col < self.width
    }

    pub fn move_by(&mut self, dr: isize, dc: isize) {
        self.pos.row += dr;
        self.pos.col += dc;
        if self.is_on_grid(self.pos.row, self.pos.col) {
            self.positions.insert(self.pos);
            self.pos_directions.insert((self.pos, self.direction));
        }
    }

    pub fn rotate_right(&mut self) {
        self.direction = self.direction.rotated_right();
    }

    pub fn step(&mut self) -> bool {
        let (dr, dc) = self.direction.delta();
        let next_row = self.pos.row + dr;
        let next_col = self.pos.col + dc;

        if self.is_occupied(next_row, next_col) {
            self.rotate_right()
        } else {
            self.move_by(dr, dc);
        }

        false
    }

    pub fn step2(&mut self) -> bool {
        let (dr, dc) = self.direction.delta();
        let next_row = self.pos.row + dr;
        let next_col = self.pos.col + dc;

        if self.pos_directions.contains(&(
            Point {
                row: next_row,
                col: next_col,
            },
            self.direction,
        )) {
            return true;
        }

        if self.is_occupied(next_row, next_col) {
            self.rotate_right()
        } else {
            self.move_by(dr, dc);
        }

        false
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (row_idx, row) in self.grid.iter().enumerate().rev() {
            for (col_idx, state) in row.iter().enumerate() {
                let c = match state {
                    State::Empty
                        if row_idx as isize == self.pos.row && col_idx as isize == self.pos.col =>
                    {
                        match self.direction {
                            Direction::Up => '^',
                            Direction::Left => '>',
                            Direction::Down => 'v',
                            Direction::Right => '<',
                        }
                    }
                    State::Empty => '.',
                    State::Occupied => '#',
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn parse_grid(input: &str) -> anyhow::Result<Grid> {
    let (_, mut grid_chars) =
        separated_list1(line_ending, parse_row)(input).map_err(|e| anyhow!("{e}"))?;
    grid_chars.reverse();
    let (row_idx, row) = grid_chars
        .iter()
        .enumerate()
        .find(|(_, row)| row.contains(&'^'))
        .ok_or_else(|| anyhow::anyhow!("No start found in grid"))?;
    let col_idx = row.iter().position(|&c| c == '^').unwrap();
    let grid: Vec<_> = grid_chars
        .iter()
        .map(|row| {
            let r: Vec<_> = row
                .iter()
                .map(|c| match c {
                    '#' => State::Occupied,
                    _ => State::Empty,
                })
                .collect();
            r
        })
        .collect();
    let start = Point {
        row: row_idx as isize,
        col: col_idx as isize,
    };
    Ok(Grid {
        width: grid[0].len() as isize,
        height: grid.len() as isize,
        grid: grid,
        pos: start,
        direction: Direction::Up,
        positions: HashSet::from([start]),
        pos_directions: HashSet::from([(start, Direction::Up)]),
    })
}

fn parse_row(input: &str) -> IResult<&str, Vec<char>> {
    many1(one_of("#.^"))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_guard_positions() -> anyhow::Result<()> {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        let grid = parse_grid(input).map_err(|e| anyhow!("{e}"))?;
        let result = count_guard_positions(&grid);
        assert_eq!(result, 41);
        Ok(())
    }

    #[test]
    fn test_count_obstructions() -> anyhow::Result<()> {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        let grid = parse_grid(input).map_err(|e| anyhow!("{e}"))?;
        let result = count_obstructions(&grid);
        assert_eq!(result, 6);
        Ok(())
    }
}
