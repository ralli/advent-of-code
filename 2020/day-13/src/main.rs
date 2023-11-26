use std::fs;

use anyhow::{anyhow, Context};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, multispace0};
use nom::combinator::{all_consuming, map};
use nom::multi::separated_list0;
use nom::sequence::terminated;
use nom::IResult;

use crate::Departure::Frequency;

fn main() -> anyhow::Result<()> {
    let filename = "day-13.txt";
    let input =
        fs::read_to_string(filename).with_context(|| format!("cannot load file {filename}"))?;
    let result = part1(&input)?;
    println!("{result}");
    Ok(())
}

struct ProgramInput {
    earliest_time: i32,
    departures: Vec<Departure>,
}

enum Departure {
    X,
    Frequency(i32),
}

fn part1(input: &str) -> anyhow::Result<i32> {
    let program_input = parse_input(input)?;
    let (min_time, min_delta): (i32, i32) = program_input
        .departures
        .iter()
        .filter_map(|d| match d {
            Departure::X => None,
            Frequency(x) => Some(x),
        })
        .copied()
        .map(|x| {
            let m = ((program_input.earliest_time / x) * x + x) - program_input.earliest_time;
            (x, m)
        })
        .min_by(|(_, y1), (_, y2)| y1.cmp(y2))
        .ok_or_else(|| anyhow!("no departures given"))?;
    Ok(min_time * min_delta)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let program_input = parse_input(input)?;
    let busses: Vec<(usize, usize)> = program_input
        .departures
        .iter()
        .enumerate()
        .filter_map(|(i, d)| match d {
            Departure::X => None,
            Frequency(x) => Some((*x as usize, i)),
        })
        .collect();
    let mut i: usize = 0;
    let mut d: usize = 1;
    for (bus, offset) in busses.into_iter() {
        loop {
            i += d;
            if (i + offset) % bus == 0 {
                d *= bus;
                break;
            }
        }
    }
    Ok(i)
}

fn parse_input(input: &str) -> anyhow::Result<ProgramInput> {
    let (_, result) = all_consuming(terminated(program_input, multispace0))(input)
        .map_err(|e| anyhow!(e.to_string()))?;
    Ok(result)
}

fn program_input(input: &str) -> IResult<&str, ProgramInput> {
    let (input, earliest_time) = complete::i32(input)?;
    let (input, _) = line_ending(input)?;
    let (input, departures) = departures(input)?;
    Ok((
        input,
        ProgramInput {
            earliest_time,
            departures,
        },
    ))
}
fn departures(input: &str) -> IResult<&str, Vec<Departure>> {
    separated_list0(tag(","), departure)(input)
}

fn departure(input: &str) -> IResult<&str, Departure> {
    let x = map(tag("x"), |_| Departure::X);
    let frequency = map(complete::i32, Frequency);
    alt((x, frequency))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"939
7,13,x,x,59,x,31,19"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 295;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 1068781;
        assert_eq!(result, expected);
        Ok(())
    }
}
