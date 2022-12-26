use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, one_of};
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::IResult;
use std::collections::HashMap;
use std::hash::Hash;

fn main() -> anyhow::Result<()> {
    let input = read_file("./day-22/input.txt")?;

    let result = part1(&input);
    println!("{}", result);

    let result = part2(&input);
    println!("{}", result);

    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Open,
    Wall,
}

#[derive(Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
        }
    }
}

impl From<Direction> for i32 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Move(i32),
    TurnLeft,
    TurnRight,
}

fn part1(input: &str) -> i32 {
    let (_, mut state) = state(input).unwrap();
    let commands = state.commands.to_vec();
    for command in commands.iter() {
        state.execute(command);
    }

    1000 * state.position.row + 4 * state.position.col + state.direction as i32
}

fn part2(_input: &str) -> i64 {
    todo!()
}

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord, Clone, Copy)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn move_by(&self, dr: i32, dc: i32) -> Self {
        Point {
            row: self.row + dr,
            col: self.col + dc,
        }
    }
}

#[derive(Debug)]
struct State {
    position: Point,
    direction: Direction,
    board: HashMap<Point, Cell>,
    commands: Vec<Command>,
}

impl State {
    fn execute(&mut self, command: &Command) {
        match command {
            Command::Move(n) => {
                // let old_pos = self.position;
                for _ in 0..*n {
                    self.position = self.next_position();
                }
                // println!(
                //     "{:?} {:?} {:?} {:?}",
                //     command, self.direction, old_pos, self.position
                // );
            }
            Command::TurnLeft => {
                self.direction = match self.direction {
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Left,
                }
            }
            Command::TurnRight => {
                self.direction = match self.direction {
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                }
            }
        }
    }

    fn next_position(&self) -> Point {
        let (dr, dc) = self.direction.delta();
        let moved = self.position.move_by(dr, dc);
        let pos = if !self.board.contains_key(&moved) {
            match self.direction {
                Direction::Right => self
                    .board
                    .keys()
                    .filter(|p| p.row == self.position.row)
                    .min(),
                Direction::Down => self
                    .board
                    .keys()
                    .filter(|p| p.col == self.position.col)
                    .min(),
                Direction::Left => self
                    .board
                    .keys()
                    .filter(|p| p.row == self.position.row)
                    .max(),
                Direction::Up => self
                    .board
                    .keys()
                    .filter(|p| p.col == self.position.col)
                    .max(),
            }
            .copied()
            .unwrap()
        } else {
            moved
        };
        let cell = self.board.get(&pos).unwrap();
        if *cell == Cell::Open {
            pos
        } else {
            self.position
        }
    }
}

fn state(input: &str) -> IResult<&str, State> {
    let (input, board) = board(input)?;
    let (input, _) = many1(line_ending)(input)?;
    let (input, commands) = commands(input)?;

    let min_point = board.keys().min().copied().unwrap();
    Ok((
        input,
        State {
            position: min_point,
            direction: Direction::Right,
            board,
            commands,
        },
    ))
}
fn board(input: &str) -> IResult<&str, HashMap<Point, Cell>> {
    let (input, lines) = separated_list1(line_ending, many1(one_of(" .#")))(input)?;
    let mut board = HashMap::new();
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c != ' ' {
                let cell = if *c == '.' { Cell::Open } else { Cell::Wall };
                board.insert(
                    Point {
                        row: (row + 1) as i32,
                        col: (col + 1) as i32,
                    },
                    cell,
                );
            }
        }
    }
    Ok((input, board))
}

fn commands(input: &str) -> IResult<&str, Vec<Command>> {
    many1(command)(input)
}

fn command(input: &str) -> IResult<&str, Command> {
    alt((
        map(complete::i32, Command::Move),
        map(tag("L"), |_| Command::TurnLeft),
        map(tag("R"), |_| Command::TurnRight),
    ))(input)
}

fn read_file(filename: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 6032;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 301;
        assert_eq!(result, expected);
    }
}
