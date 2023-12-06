use std::fs;

use anyhow::{anyhow, Context};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, space1};
use nom::combinator::complete;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated};

fn main() -> anyhow::Result<()> {
    let filename = "day-6.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i64> {
    let races = parse_input(input)?;
    let result = races.iter().map(|race| race.number_of_wins()).product();
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<i64> {
    let race = parse_race(input)?;
    let result = race.number_of_wins();
    Ok(result)
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Race>> {
    let (_, races) = parse_races(input).map_err(|e| anyhow!(e.to_string()))?;
    Ok(races)
}

fn parse_race(input: &str) -> anyhow::Result<Race> {
    let lines: Vec<&str> = input.lines().collect();
    let time_line: String = lines.first().unwrap().chars().filter(|c| c.is_ascii_digit()).collect();
    let distance_line: String = lines[1].chars().filter(|c| c.is_ascii_digit()).collect();
    let time = time_line.parse()?;
    let distance = distance_line.parse()?;
    Ok(Race { time, distance })
}

fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, times) = preceded(terminated(tag("Time:"), space1), separated_list1(space1, complete::i64))(input)?;
    let (input, _) = line_ending(input)?;
    let (input, distances) = preceded(terminated(tag("Distance:"), space1), separated_list1(space1, complete::i64))(input)?;
    let races = times.into_iter().zip(distances.into_iter()).map(|(time, distance)| Race { time, distance }).collect();
    Ok((input, races))
}

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn number_of_wins(&self) -> i64 {
        (0..self.time).map(|time| {
            let speed = time;
            (self.time - time) * speed
        }).filter(|&distance| distance > self.distance).count() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 288;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 71503;
        assert_eq!(result, expected);
        Ok(())
    }
}
