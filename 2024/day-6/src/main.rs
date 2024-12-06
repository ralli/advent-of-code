use anyhow::anyhow;
use nom::character::complete::{line_ending, one_of};
use nom::multi::{many1, separated_list1};
use nom::IResult;
use rayon::prelude::*;
use std::collections::BTreeSet;
use std::{fmt, fs};

fn main() -> anyhow::Result<()> {
    let content = fs::read_to_string("day-6/input.txt")?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut grid = parse_grid(input)?;

    while grid.is_on_grid(grid.pos.row, grid.pos.col) {
        grid.step();
    }

    Ok(grid.positions.len())
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let grid = parse_grid(input)?;

    let mut copy = grid.clone();
    while copy.is_on_grid(copy.pos.row, copy.pos.col) {
        copy.step();
    }
    let positions = copy.positions;
    let count = positions
        .par_iter()
        .filter(|p| {
            !(grid.is_occupied(p.row, p.col) || (p.row == grid.pos.row && p.col == grid.pos.col))
        })
        .filter(|p| {
            let row_idx = p.row;
            let col_idx = p.col;
            let mut copy = grid.clone();
            copy.grid[row_idx as usize][col_idx as usize] = State::Occupied;
            while copy.is_on_grid(copy.pos.row, copy.pos.col) {
                if copy.step2() {
                    return true;
                }
            }
            false
        })
        .count();
    Ok(count)
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord)]
struct Point {
    row: isize,
    col: isize,
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Ord)]
enum State {
    Empty,
    Occupied,
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Ord)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotated_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (-1, 0),
            Direction::Left => (0, -1),
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Vec<State>>,
    width: isize,
    height: isize,
    pos: Point,
    direction: Direction,
    positions: BTreeSet<Point>,
    pos_directions: BTreeSet<(Point, Direction)>,
}

impl Grid {
    fn is_occupied(&self, row: isize, col: isize) -> bool {
        self.is_on_grid(row, col) && self.grid[row as usize][col as usize] == State::Occupied
    }

    fn is_on_grid(&self, row: isize, col: isize) -> bool {
        0 <= row && row < self.height && 0 <= col && col < self.width
    }

    fn move_by(&mut self, dr: isize, dc: isize) {
        self.pos.row += dr;
        self.pos.col += dc;
        if self.is_on_grid(self.pos.row, self.pos.col) {
            self.positions.insert(self.pos);
            self.pos_directions.insert((self.pos, self.direction));
        }
    }

    fn rotate_right(&mut self) {
        self.direction = self.direction.rotated_right();
    }

    fn step(&mut self) -> bool {
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

    fn step2(&mut self) -> bool {
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

fn parse_grid(input: &str) -> anyhow::Result<Grid> {
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
        positions: BTreeSet::from([start]),
        pos_directions: BTreeSet::from([(start, Direction::Up)]),
    })
}

fn parse_row(input: &str) -> IResult<&str, Vec<char>> {
    many1(one_of("#.^"))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> anyhow::Result<()> {
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
        let result = part1(input)?;
        assert_eq!(result, 41);
        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
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
        let result = part2(input)?;
        assert_eq!(result, 6);
        Ok(())
    }
}
