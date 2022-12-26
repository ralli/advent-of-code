use std::collections::HashMap;
use std::hash::Hash;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, one_of};
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::IResult;

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

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
        }
    }
}

const SIZE: i32 = 50;
const FACES: [Face; 6] = [
    Face::Top,
    Face::Right,
    Face::Front,
    Face::Left,
    Face::Bottom,
    Face::Back,
];
#[derive(Debug, Clone, Copy)]
enum Face {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

impl Face {
    fn delta(&self) -> (i32, i32) {
        use Face::*;

        match self {
            Top => (0, 1),
            Right => (0, 2),
            Front => (1, 1),
            Left => (2, 0),
            Bottom => (2, 1),
            Back => (3, 0),
        }
    }

    fn offset(&self) -> (i32, i32) {
        let (r, c) = self.delta();
        (r * SIZE, c * SIZE)
    }

    fn from_global(p: &Point) -> Face {
        let (r, c) = (p.row / SIZE, p.col / SIZE);
        FACES
            .iter()
            .find(|f| {
                let (fr, fc) = f.delta();
                fr == r && fc == c
            })
            .copied()
            .unwrap()
    }

    fn transform_to_global(&self, p: &Point) -> Point {
        let (dr, dc) = self.offset();
        Point {
            row: p.row + dr,
            col: p.col + dc,
        }
    }

    fn transform_to_local(&self, p: &Point) -> Point {
        let (dr, dc) = self.offset();
        Point {
            row: p.row - dr,
            col: p.col - dc,
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

    1000 * (state.position.row + 1) + 4 * (state.position.col + 1) + state.direction as i32
}

fn part2(input: &str) -> i32 {
    let (_, mut state) = state(input).unwrap();
    let commands = state.commands.to_vec();
    for command in commands.iter() {
        state.execute_cube(command);
    }

    1000 * (state.position.row + 1) + 4 * (state.position.col + 1) + state.direction as i32
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

    fn execute_cube(&mut self, command: &Command) {
        match command {
            Command::Move(n) => {
                // let old_pos = self.position;
                for _ in 0..*n {
                    let (next_position, next_direction) = self.next_cube_position();
                    self.position = next_position;
                    self.direction = next_direction;
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

    fn next_cube_position(&self) -> (Point, Direction) {
        let face = Face::from_global(&self.position);
        let local = face.transform_to_local(&self.position);
        let (dr, dc) = self.direction.delta();
        let moved = Point {
            row: local.row + dr,
            col: local.col + dc,
        };
        let n = SIZE - 1;
        let (next_face, next_local, next_direction) =
            if (0..SIZE).contains(&moved.row) && (0..SIZE).contains(&moved.col) {
                (face, moved, self.direction)
            } else {
                match (face, self.direction) {
                    (Face::Top, Direction::Left) => (
                        Face::Left,
                        Point {
                            row: n - local.row,
                            col: 0,
                        },
                        Direction::Right,
                    ),
                    (Face::Top, Direction::Right) => (
                        Face::Right,
                        Point {
                            row: local.row,
                            col: 0,
                        },
                        Direction::Right,
                    ),
                    (Face::Top, Direction::Up) => (
                        Face::Back,
                        Point {
                            row: local.col,
                            col: 0,
                        },
                        Direction::Right,
                    ),
                    (Face::Top, Direction::Down) => (
                        Face::Front,
                        Point {
                            row: 0,
                            col: local.col,
                        },
                        Direction::Down,
                    ),
                    (Face::Right, Direction::Left) => (
                        Face::Top,
                        Point {
                            row: local.row,
                            col: n,
                        },
                        Direction::Left,
                    ),
                    (Face::Right, Direction::Right) => (
                        Face::Bottom,
                        Point {
                            row: n - local.row,
                            col: n,
                        },
                        Direction::Left,
                    ),
                    (Face::Right, Direction::Up) => (
                        Face::Back,
                        Point {
                            row: n,
                            col: local.col,
                        },
                        Direction::Up,
                    ),
                    (Face::Right, Direction::Down) => (
                        Face::Front,
                        Point {
                            row: local.col,
                            col: n,
                        },
                        Direction::Left,
                    ),
                    (Face::Front, Direction::Left) => (
                        Face::Left,
                        Point {
                            row: 0,
                            col: local.row,
                        },
                        Direction::Down,
                    ),
                    (Face::Front, Direction::Right) => (
                        Face::Right,
                        Point {
                            row: n,
                            col: local.row,
                        },
                        Direction::Up,
                    ),
                    (Face::Front, Direction::Up) => (
                        Face::Top,
                        Point {
                            row: n,
                            col: local.col,
                        },
                        Direction::Up,
                    ),
                    (Face::Front, Direction::Down) => (
                        Face::Bottom,
                        Point {
                            row: 0,
                            col: local.col,
                        },
                        Direction::Down,
                    ),
                    (Face::Left, Direction::Left) => (
                        Face::Top,
                        Point {
                            row: n - local.row,
                            col: 0,
                        },
                        Direction::Right,
                    ),
                    (Face::Left, Direction::Right) => (
                        Face::Bottom,
                        Point {
                            row: local.row,
                            col: 0,
                        },
                        Direction::Right,
                    ),
                    (Face::Left, Direction::Up) => (
                        Face::Front,
                        Point {
                            row: local.col,
                            col: 0,
                        },
                        Direction::Right,
                    ),
                    (Face::Left, Direction::Down) => (
                        Face::Back,
                        Point {
                            row: 0,
                            col: local.col,
                        },
                        Direction::Down,
                    ),
                    (Face::Bottom, Direction::Left) => (
                        Face::Left,
                        Point {
                            row: local.row,
                            col: n,
                        },
                        Direction::Left,
                    ),
                    (Face::Bottom, Direction::Right) => (
                        Face::Right,
                        Point {
                            row: n - local.row,
                            col: n,
                        },
                        Direction::Left,
                    ),
                    (Face::Bottom, Direction::Up) => (
                        Face::Front,
                        Point {
                            row: n,
                            col: local.col,
                        },
                        Direction::Up,
                    ),
                    (Face::Bottom, Direction::Down) => (
                        Face::Back,
                        Point {
                            row: local.col,
                            col: n,
                        },
                        Direction::Left,
                    ),
                    (Face::Back, Direction::Left) => (
                        Face::Top,
                        Point {
                            row: 0,
                            col: local.row,
                        },
                        Direction::Down,
                    ),
                    (Face::Back, Direction::Right) => (
                        Face::Bottom,
                        Point {
                            row: n,
                            col: local.row,
                        },
                        Direction::Up,
                    ),
                    (Face::Back, Direction::Up) => (
                        Face::Left,
                        Point {
                            row: n,
                            col: local.col,
                        },
                        Direction::Up,
                    ),
                    (Face::Back, Direction::Down) => (
                        Face::Right,
                        Point {
                            row: 0,
                            col: local.col,
                        },
                        Direction::Down,
                    ),
                }
            };

        let next_position = next_face.transform_to_global(&next_local);
        let cell = self.board.get(&next_position).unwrap();
        if *cell == Cell::Wall {
            (self.position, self.direction)
        } else {
            (next_face.transform_to_global(&next_local), next_direction)
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
                        row: row as i32,
                        col: col as i32,
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
