extern crate core;

use std::collections::BTreeSet;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Read;

use anyhow::Context;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, one_of};
use nom::multi::{many1, separated_list1};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let filename = "./day-13/input.txt";
    let content = read_file(filename).context(filename)?;

    let result = part1(&content);
    println!("{}", result);

    part2(&content);

    Ok(())
}

fn part1(input: &str) -> usize {
    let (_, bla) = parse_input(input).unwrap();
    let instruction = bla.instructions.first().unwrap();
    let next_points: BTreeSet<Point> = match instruction {
        FoldInstruction::X(x) => bla
            .points
            .into_iter()
            .map(|p| p.folded_along_x(*x))
            .collect(),
        FoldInstruction::Y(y) => bla
            .points
            .into_iter()
            .map(|p| p.folded_along_y(*y))
            .collect(),
    };
    let result = next_points.len();
    result
}

fn part2(input: &str) {
    let (_, bla) = parse_input(input).unwrap();
    let final_points = bla
        .instructions
        .iter()
        .fold(bla.points, |points, instruction| {
            let next_points: BTreeSet<Point> = match instruction {
                FoldInstruction::X(x) => points.into_iter().map(|p| p.folded_along_x(*x)).collect(),
                FoldInstruction::Y(y) => points.into_iter().map(|p| p.folded_along_y(*y)).collect(),
            };
            next_points
        });
    println!(
        "{}",
        Input {
            points: final_points,
            instructions: Vec::new()
        }
    );
}

#[derive(Debug, Copy, Clone)]
enum FoldInstruction {
    X(i32),
    Y(i32),
}

#[derive(Debug)]
struct Input {
    points: BTreeSet<Point>,
    instructions: Vec<FoldInstruction>,
}

impl Input {
    fn char_at(&self, x: i32, y: i32) -> char {
        if self.points.contains(&Point { x, y }) {
            '#'
        } else {
            '.'
        }
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let width = self.points.iter().map(|p| p.x).max().unwrap_or(0);
        let height = self.points.iter().map(|p| p.y).max().unwrap_or(0);

        for y in 0..=height {
            for x in 0..=width {
                write!(f, "{}", self.char_at(x, y))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn folded_along_x(&self, x: i32) -> Point {
        if self.x < x {
            *self
        } else {
            let xdiff = self.x - x;
            Point {
                x: x - xdiff,
                y: self.y,
            }
        }
    }

    fn folded_along_y(&self, y: i32) -> Point {
        if self.y < y {
            *self
        } else {
            let ydiff = self.y - y;
            Point {
                x: self.x,
                y: y - ydiff,
            }
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, points) = points(input)?;
    let (input, _) = many1(line_ending)(input)?;
    let (input, instructions) = instructions(input)?;

    Ok((
        input,
        Input {
            points: points.into_iter().collect(),
            instructions,
        },
    ))
}

fn instructions(input: &str) -> IResult<&str, Vec<FoldInstruction>> {
    separated_list1(line_ending, instruction)(input)
}

fn instruction(input: &str) -> IResult<&str, FoldInstruction> {
    let (input, _) = tag("fold along ")(input)?;
    let (input, xy) = one_of("xy")(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, coordinate) = complete::i32(input)?;
    let instruction = match xy {
        'x' => FoldInstruction::X(coordinate),
        'y' => FoldInstruction::Y(coordinate),
        _ => unreachable!("{}", xy),
    };

    Ok((input, instruction))
}

fn points(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(line_ending, point)(input)
}

fn point(input: &str) -> IResult<&str, Point> {
    let (input, x) = complete::i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = complete::i32(input)?;
    Ok((input, Point { x, y }))
}

fn read_file(name: &str) -> anyhow::Result<String> {
    let mut f = File::open(name)?;
    let mut result = String::new();
    f.read_to_string(&mut result)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test.txt");

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 17;
        assert_eq!(result, expected);
    }
}
