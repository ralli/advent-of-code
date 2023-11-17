use std::fs;

use anyhow::anyhow;
use nom::{character, IResult};
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, multispace0, newline, not_line_ending, space1};
use nom::combinator::eof;
use nom::multi::separated_list0;
use nom::sequence::tuple;

fn main() -> anyhow::Result<()> {
    let filename = "input.txt";
    let input = fs::read_to_string(&filename)?;
    println!("{input}");
    let num_valid = part1(&input)?;
    println!("part1: {num_valid}");
    let num_valid = part2(&input)?;
    println!("part2: {num_valid}");
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<u32> {
    let (_, lines) = parse_lines(&input).map_err(|e| anyhow!(e.to_string()))?;
    Ok(lines.iter().filter(|line| line.is_valid_part_1()).count() as u32)
}

fn part2(input: &str) -> anyhow::Result<u32> {
    let (_, lines) = parse_lines(&input).map_err(|e| anyhow!(e.to_string()))?;
    Ok(lines.iter().filter(|line| line.is_valid_part_2()).count() as u32)
}

#[derive(Debug)]
struct Policy {
    min_occurs: u32,
    max_occurs: u32,
    letter: char,
}

impl Policy {
    fn is_valid(&self, password: &str) -> bool {
        let num_occurs = password.chars().filter(|&c| c == self.letter).count() as u32;
        self.min_occurs <= num_occurs && self.max_occurs >= num_occurs
    }
}

#[derive(Debug)]
struct Line {
    policy: Policy,
    password: String,
}

impl Line {
    fn is_valid_part_1(&self) -> bool {
        self.policy.is_valid(&self.password)
    }

    fn is_valid_part_2(&self) -> bool {
        let pwbytes = self.password.as_bytes();
        let pwlen = pwbytes.len();
        let letter = self.policy.letter as u8;
        let i1 = (self.policy.min_occurs - 1) as usize;
        let i2 = (self.policy.max_occurs - 1) as usize;
        let first_char = i1 < pwlen && pwbytes[i1] == letter;
        let second_char = i2 < pwlen && pwbytes[i2] == letter;
        first_char ^ second_char
    }
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list0(newline, parse_line)(input)?;
    let (input, _) = tuple((multispace0, eof))(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, policy) = parse_policy(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space1(input)?;
    let (input, password) = not_line_ending(input)?;
    Ok((
        input,
        Line {
            policy,
            password: password.to_string(),
        },
    ))
}

fn parse_policy(input: &str) -> IResult<&str, Policy> {
    let (input, min_occurs) = character::complete::u32(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, max_occurs) = character::complete::u32(input)?;
    let (input, _) = space1(input)?;
    let (input, letter) = anychar(input)?;

    Ok((
        input,
        Policy {
            min_occurs,
            max_occurs,
            letter,
        },
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {}
}

