use anyhow::Context;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::combinator::map;
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;
use std::path::Path;
use std::{fmt, fs};

fn main() -> anyhow::Result<()> {
    let content = read_file("day-3/day-3.txt")?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i32> {
    let muls = parse_input(input)?;
    Ok(muls.iter().map(|(a, b)| a * b).sum())
}

fn part2(input: &str) -> anyhow::Result<i32> {
    let instructions = parse_instructions(input)?;
    let mut ok = true;
    let mut result = 0;
    for i in instructions.iter() {
        match i {
            Instruction::Do => ok = true,
            Instruction::Dont => ok = false,
            Instruction::Mul(a, b) => {
                if ok {
                    result += a * b
                }
            }
        }
    }
    Ok(result)
}

fn parse_input(input: &str) -> anyhow::Result<Vec<(i32, i32)>> {
    let mut result = Vec::new();
    let mut it = input.chars();
    while let Some(_) = it.next() {
        if let Ok((_, pair)) = parse_mul(it.as_str()) {
            result.push(pair);
        }
    }
    Ok(result)
}

fn parse_instructions(input: &str) -> anyhow::Result<Vec<Instruction>> {
    let mut result = Vec::new();
    let mut it = input.chars();
    while let Some(_) = it.next() {
        if let Ok((_, pair)) = parse_instruction(it.as_str()) {
            result.push(pair);
        }
    }
    Ok(result)
}

enum Instruction {
    Do,
    Dont,
    Mul(i32, i32),
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(parse_mul, |(a, b)| Instruction::Mul(a, b)),
        map(parse_do, |_| Instruction::Do),
        map(parse_dont, |_| Instruction::Dont),
    ))(input)
}

fn parse_mul(input: &str) -> IResult<&str, (i32, i32)> {
    let (rest, result) = preceded(
        tag("mul"),
        delimited(
            tag("("),
            separated_pair(complete::i32, tag(","), complete::i32),
            tag(")"),
        ),
    )(input)?;
    Ok((rest, result))
}

fn parse_dont(input: &str) -> IResult<&str, ()> {
    let (rest, _) = tag("don't()")(input)?;
    Ok((rest, ()))
}

fn parse_do(input: &str) -> IResult<&str, ()> {
    let (rest, _) = tag("do()")(input)?;
    Ok((rest, ()))
}

fn read_file(filename: impl AsRef<Path> + fmt::Display) -> anyhow::Result<String> {
    fs::read_to_string(filename.as_ref()).context(format!("cannot load file {}", filename))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> anyhow::Result<()> {
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        let result = part1(input)?;
        assert_eq!(result, 161);
        Ok(())
    }
}
