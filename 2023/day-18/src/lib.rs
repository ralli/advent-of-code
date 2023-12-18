use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use anyhow::anyhow;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete;
use nom::character::complete::{line_ending, one_of, space1};
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::separated_list1;
use nom::Parser;
use nom::sequence::{delimited, tuple};

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let mut state = parse_input(input)?;
    let commands = state.commands.clone();
    for cmd in commands.iter() {
        state.process_command(cmd);
    }
    println!("{}", state.grid);
    state.grid.floodfill();
    println!("{}", state.grid);
    let result = state.grid.count_filled();
    Ok(result)
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[derive(Debug)]
struct State {
    commands: Vec<Command>,
    pos: Position,
    grid: Grid,
}

impl State {
    fn process_command(&mut self, command: &Command) {
        let (dr, dc) = command.direction.delta();
        for _ in 0..command.distance {
            self.pos.row += dr;
            self.pos.col += dc;
            self.grid.put(self.pos.row, self.pos.col, command.color);
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Command {
    direction: Direction,
    distance: u32,
    color: Color,
}

#[derive(Debug, Copy, Clone)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1)
        }
    }
}

#[derive(Debug)]
struct Grid {
    cubes: BTreeMap<Position, Color>,
}

impl Grid {
    fn new() -> Self {
        Self { cubes: BTreeMap::new() }
    }

    fn put(&mut self, row: i32, col: i32, color: Color) {
        self.cubes.insert(Position::new(row, col), color);
    }

    fn get(&self, row: i32, col: i32) -> Option<Color> {
        self.cubes.get(&Position::new(row, col)).copied()
    }

    fn is_set(&self, row: i32, col: i32) -> bool {
        self.cubes.contains_key(&Position::new(row, col))
    }

    fn get_min_row(&self) -> i32 {
        self.cubes.keys().map(|c| c.row).min().unwrap_or_default()
    }

    fn get_max_row(&self) -> i32 {
        self.cubes.keys().map(|c| c.row).max().unwrap_or_default()
    }

    fn get_min_col(&self) -> i32 {
        self.cubes.keys().map(|c| c.col).min().unwrap_or_default()
    }

    fn get_max_col(&self) -> i32 {
        self.cubes.keys().map(|c| c.col).max().unwrap_or_default()
    }

    fn count_filled(&self) -> usize {
        let min_row = self.get_min_row();
        let max_row = self.get_max_row();
        let min_col = self.get_min_col();
        let max_col = self.get_max_col();
        let mut result = 0;
        for row in min_row..=max_row {
            for col in min_col..=max_col {
                if let Some(Color { red: 255, green: 255, blue: 255 }) = self.get(row, col) {} else {
                    result += 1;
                }
            }
        }
        result
    }
    fn floodfill(&mut self) {
        let min_row = self.get_min_row() - 1;
        let max_row = self.get_max_row() + 1;
        let min_col = self.get_min_col() - 1;
        let max_col = self.get_max_col() + 1;
        let start = Position::new(min_row, min_col);
        let mut q = VecDeque::from([start]);

        while let Some(current) = q.pop_front() {
            if (self.is_set(current.row, current.col)) {
                continue;
            }
            self.put(current.row, current.col, Color { red: 255, green: 255, blue: 255 });

            if current.row > min_row {
                let next_pos = Position::new(current.row - 1, current.col);
                q.push_back(next_pos);
            }

            if current.row < max_row {
                let next_pos = Position::new(current.row + 1, current.col);
                q.push_back(next_pos);
            }

            if current.col > min_col {
                let next_pos = Position::new(current.row, current.col - 1);
                q.push_back(next_pos);
            }

            if current.col < max_col {
                let next_pos = Position::new(current.row, current.col + 1);
                q.push_back(next_pos);
            }
        }
    }

    fn fill(&mut self) {
        let min_row = self.get_min_row();
        let max_row = self.get_max_row();
        let min_col = self.get_min_col();
        let max_col = self.get_max_col();

        for row in min_row..=max_row {
            let mut col = min_col;
            while col <= max_col {
                while !self.is_set(row, col) && col <= max_col {
                    col += 1;
                }
                while self.is_set(row, col) && col <= max_col {
                    col += 1;
                }
                while !self.is_set(row, col) && col <= max_col {
                    self.put(row, col, Color { red: 0, green: 0, blue: 0 });
                    col += 1;
                }
                if self.is_set(row, col) && col <= max_col {
                    col += 1;
                }
            }
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let min_row = self.get_min_row();
        let max_row = self.get_max_row();
        let min_col = self.get_min_col();
        let max_col = self.get_max_col();

        for row in min_row..=max_row {
            for col in min_col..=max_col {
                let ch = if self.is_set(row, col) { '#' } else { '.' };
                write!(f, "{ch}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}


fn parse_input(input: &str) -> anyhow::Result<State> {
    let (_, commands) = parse_commands(input).map_err(|e| anyhow!(e.to_string()))?;
    let pos = Position::new(0, 0);
    let mut grid = Grid::new();
    grid.put(pos.row, pos.col, Color { red: 0, green: 0, blue: 0 });
    Ok(State { commands, pos, grid })
}

fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(line_ending, parse_command)(input)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, direction) = parse_direction(input)?;
    let (input, _) = space1(input)?;
    let (input, distance) = complete::u32(input)?;
    let (input, _) = space1(input)?;
    let (input, color) = delimited(tag("("), hex_color, tag(")"))(input)?;
    Ok((input, Command { direction, distance, color }))
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(2, 2, is_hex_digit),
        from_hex,
    )(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, c) = one_of("UDLR")(input)?;
    let dir = match c {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => unreachable!("no color"),
    };
    Ok((input, dir))
}


#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 62;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 94;
        assert_eq!(result, expected);
        Ok(())
    }
}
