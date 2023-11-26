use std::fs;

use anyhow::Context;
use nom::character::complete;
use nom::character::complete::{line_ending, multispace0, one_of};
use nom::combinator::all_consuming;
use nom::multi::separated_list0;
use nom::sequence::{terminated, tuple};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let filename = "day-12.txt";
    let input =
        fs::read_to_string(filename).with_context(|| format!("cannot read file {filename}"))?;
    let result = part1(&input)?;
    println!("{result}");
    let result = part2(&input)?;
    println!("{result}");
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i32> {
    let commands = parse_input(input)?;
    let mut state = State::default();
    for command in commands.iter() {
        state = state.apply(command);
    }
    Ok(state.x.abs() + state.y.abs())
}

fn part2(input: &str) -> anyhow::Result<i32> {
    let commands = parse_input(input)?;
    let mut state = State2::default();
    for command in commands.iter() {
        state = state.apply(command);
    }
    Ok(state.position.x.abs() + state.position.y.abs())
}

#[derive(Debug, Copy, Clone)]
struct State {
    direction: Direction,
    x: i32,
    y: i32,
}

impl State {
    fn apply(&self, command: &Command) -> State {
        let next_direction = self.direction.after_command(command);
        let (next_x, next_y) = match command {
            Command::North(d) => (self.x, self.y + *d),
            Command::East(d) => (self.x + *d, self.y),
            Command::South(d) => (self.x, self.y - *d),
            Command::West(d) => (self.x - *d, self.y),
            Command::Forward(d) => match &self.direction {
                Direction::North => (self.x, self.y + *d),
                Direction::East => (self.x + *d, self.y),
                Direction::South => (self.x, self.y - *d),
                Direction::West => (self.x - *d, self.y),
            },
            _ => (self.x, self.y),
        };
        State {
            direction: next_direction,
            x: next_x,
            y: next_y,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            direction: Direction::East,
            x: 0,
            y: 0,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State2 {
    waypoint: Point,
    position: Point,
}

impl State2 {
    fn apply(&self, command: &Command) -> State2 {
        let next_waypoint = self.waypoint.after_command(command);
        let next_position = match command {
            Command::Forward(d) => Point {
                x: self.position.x + *d * self.waypoint.x,
                y: self.position.y + *d * self.waypoint.y,
            },
            _ => self.position,
        };
        State2 {
            waypoint: next_waypoint,
            position: next_position,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn after_command(&self, command: &Command) -> Point {
        match command {
            Command::Left(mut degrees) => {
                let mut p = *self;
                while degrees > 0 {
                    p = p.left();
                    degrees -= 90;
                }
                p
            }
            Command::Right(mut degrees) => {
                let mut p = *self;
                while degrees > 0 {
                    p = p.right();
                    degrees -= 90;
                }
                p
            }
            Command::North(distance) => Point {
                x: self.x,
                y: self.y + *distance,
            },
            Command::East(distance) => Point {
                x: self.x + *distance,
                y: self.y,
            },
            Command::South(distance) => Point {
                x: self.x,
                y: self.y - *distance,
            },
            Command::West(distance) => Point {
                x: self.x - *distance,
                y: self.y,
            },
            _ => *self,
        }
    }

    fn left(&self) -> Point {
        Point {
            x: -self.y,
            y: self.x,
        }
    }

    fn right(&self) -> Point {
        Point {
            x: self.y,
            y: -self.x,
        }
    }
}

impl Default for State2 {
    fn default() -> Self {
        State2 {
            waypoint: Point { x: 10, y: 1 },
            position: Point { x: 0, y: 0 },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    North(i32),
    West(i32),
    South(i32),
    East(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn after_command(&self, command: &Command) -> Direction {
        match command {
            Command::Left(mut degrees) => {
                let mut d = *self;
                while degrees > 0 {
                    d = d.left();
                    degrees -= 90;
                }
                d
            }
            Command::Right(mut degrees) => {
                let mut d = *self;
                while degrees > 0 {
                    d = d.right();
                    degrees -= 90;
                }
                d
            }
            _ => *self,
        }
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Command>> {
    let (_, commands) = all_consuming(terminated(command_list, multispace0))(input)
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
    Ok(commands)
}

fn command_list(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list0(line_ending, command)(input)
}

fn command(input: &str) -> IResult<&str, Command> {
    let (input, (c, n)) = tuple((one_of("NWSELRF"), complete::i32))(input)?;
    let command = match c {
        'N' => Command::North(n),
        'W' => Command::West(n),
        'S' => Command::South(n),
        'E' => Command::East(n),
        'L' => Command::Left(n),
        'R' => Command::Right(n),
        'F' => Command::Forward(n),
        _ => unreachable!(),
    };
    Ok((input, command))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"F10
N3
F7
R90
F11"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 25;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 286;
        assert_eq!(result, expected);
        Ok(())
    }
}
