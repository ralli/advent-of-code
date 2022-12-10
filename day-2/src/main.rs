use std::fs::File;
use std::io::Read;

use anyhow::*;
use nom::character::complete::{line_ending, one_of, space1};
use nom::IResult;
use nom::multi::separated_list1;
use std::slice::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::preceded;

fn main() -> anyhow::Result<()> {
    let filename = "./day-2/input.txt";
    let content = read_file(filename).context(filename)?;

    let result = part1(&content);
    println!("{}", result);

    let result = part2(&content);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> u32 {
    let (_, commands) = commands(input).unwrap();
    let (x, y) = commands.into_iter().fold((0, 0), |(x, y), c|
        match c {
            Command::Forward(n) => (x + n, y),
            Command::Up(n) => (x, y - n),
            Command::Down(n) => (x, y + n),
        },
    );
    x * y
}

fn part2(input: &str) -> u32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    let (_, commands) = commands(input).unwrap();

    for command in commands {
        match command {
            Command::Forward(n) => {
                x += n;
                y += aim * n;
            }
            Command::Up(n) => { aim -= n; }
            Command::Down(n) => { aim += n; }
        };
    }

    x * y
}

#[derive(Debug)]
enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

fn commands(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(line_ending, command)(input)
}

fn command(input: &str) -> IResult<&str, Command> {
    use nom::character::complete::u32 as u32_parser;
    let forward = map(preceded(tag("forward "), u32_parser), |n| Command::Forward(n));
    let up = map(preceded(tag("up "), u32_parser), |n| Command::Up(n));
    let down = map(preceded(tag("down "), u32_parser), |n| Command::Down(n));
    alt((forward, up, down))(input)
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

    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 150;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 900;
        assert_eq!(result, expected);
    }
}
