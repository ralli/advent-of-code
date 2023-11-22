use anyhow::Context;
use std::fmt::Formatter;
use std::{fmt, fs};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::{many1, separated_list0};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let filename = "day-11.txt";
    let input =
        fs::read_to_string(filename).with_context(|| format!("cannot load file {filename}"))?;
    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut layout = parse_input(input)?;
    loop {
        let (num_changes, next) = layout.step();
        if num_changes == 0 {
            return Ok(next.num_occupied());
        }
        layout = next;
    }
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut layout = parse_input(input)?;
    loop {
        let (num_changes, next) = layout.step2();
        if num_changes == 0 {
            return Ok(next.num_occupied());
        }
        layout = next;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Position {
    Floor,
    Empty,
    Occupied,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            Position::Floor => '.',
            Position::Empty => 'L',
            Position::Occupied => '#',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Layout {
    positions: Vec<Vec<Position>>,
}

impl Layout {
    fn step(&self) -> (i32, Layout) {
        let mut n = self.clone();
        let mut num_changes = 0;
        for row in 0..self.positions.len() {
            for col in 0..self.positions[0].len() {
                let pos = self.positions[row][col];
                if pos == Position::Occupied || pos == Position::Empty {
                    let occupied_count =
                        self.adj_count(row as isize, col as isize, Position::Occupied);
                    if self.positions[row][col] == Position::Empty && occupied_count == 0 {
                        n.positions[row][col] = Position::Occupied;
                        num_changes += 1;
                    }
                    if self.positions[row][col] == Position::Occupied && occupied_count >= 4 {
                        n.positions[row][col] = Position::Empty;
                        num_changes += 1;
                    }
                }
            }
        }
        (num_changes, n)
    }

    fn step2(&self) -> (i32, Layout) {
        let mut n = self.clone();
        let mut num_changes = 0;
        for row in 0..self.positions.len() {
            for col in 0..self.positions[0].len() {
                let pos = self.positions[row][col];
                if pos == Position::Occupied || pos == Position::Empty {
                    let occupied_count = self.adj_count2(row as isize, col as isize);
                    if self.positions[row][col] == Position::Empty && occupied_count == 0 {
                        n.positions[row][col] = Position::Occupied;
                        num_changes += 1;
                    }
                    if self.positions[row][col] == Position::Occupied && occupied_count >= 5 {
                        n.positions[row][col] = Position::Empty;
                        num_changes += 1;
                    }
                }
            }
        }
        (num_changes, n)
    }

    fn adj_count2(&self, row: isize, col: isize) -> i32 {
        let directions = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let mut result = 0;
        for (dx, dy) in directions {
            if self.occupied_2(row, col, dx, dy) {
                result += 1;
            }
        }
        result
    }

    fn occupied_2(&self, row: isize, col: isize, dx: isize, dy: isize) -> bool {
        let mut r = row + dy;
        let mut c = col + dx;

        while r >= 0
            && r < self.positions.len() as isize
            && c >= 0
            && c < self.positions[0].len() as isize
        {
            if self.positions[r as usize][c as usize] == Position::Empty {
                return false;
            }
            if self.positions[r as usize][c as usize] == Position::Occupied {
                return true;
            }
            c += dx;
            r += dy;
        }
        return false;
    }

    fn adj_count(&self, row: isize, col: isize, value: Position) -> i32 {
        let mut result = 0;
        for r in row - 1..=row + 1 {
            for c in col - 1..=col + 1 {
                if (r != row || c != col)
                    && r >= 0
                    && r < self.positions.len() as isize
                    && c >= 0
                    && c < self.positions[0].len() as isize
                    && self.positions[r as usize][c as usize] == value
                {
                    result += 1;
                }
            }
        }
        result
    }

    fn num_occupied(&self) -> usize {
        self.positions
            .iter()
            .map(|r| r.iter().filter(|&p| *p == Position::Occupied).count())
            .sum()
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for r in self.positions.iter() {
            for v in r.iter() {
                write!(f, "{v}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> anyhow::Result<Layout> {
    let (_, layout) = layout(input).map_err(|e| anyhow::Error::msg(e.to_string()))?;
    Ok(layout)
}

fn layout(input: &str) -> IResult<&str, Layout> {
    let (input, positions) = separated_list0(line_ending, row)(input)?;
    Ok((input, Layout { positions }))
}

fn row(input: &str) -> IResult<&str, Vec<Position>> {
    many1(position)(input)
}

fn position(input: &str) -> IResult<&str, Position> {
    let floor = map(tag("."), |_| Position::Floor);
    let empty = map(tag("L"), |_| Position::Empty);
    let occupied = map(tag("#"), |_| Position::Occupied);
    alt((floor, empty, occupied))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 37;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 26;
        assert_eq!(result, expected);
        Ok(())
    }
}
