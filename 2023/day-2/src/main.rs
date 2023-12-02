use std::collections::HashMap;
use std::fs;

use anyhow::{anyhow, Context};
use nom::character::complete;
use nom::character::complete::{line_ending, multispace0, space1};
use nom::combinator::all_consuming;
use nom::multi::separated_list0;
use nom::sequence::{delimited, terminated};
use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

fn main() -> anyhow::Result<()> {
    let filename = "day-2.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot read {filename}"))?;
    let result = part1(&input)?;
    println!("{result}");
    let result = part2(&input)?;
    println!("{result}");
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<u32> {
    let games = parse_input(input)?;
    let result = games
        .games
        .iter()
        .filter(|game| game.is_possible())
        .map(|game| game.id)
        .sum();
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<u32> {
    let games = parse_input(input)?;
    let result = games
        .games
        .iter()
        .map(|game| game.min_cube_counts().power())
        .sum();
    Ok(result)
}

#[derive(Debug)]
struct GameList {
    games: Vec<Game>,
}

#[derive(Debug)]
struct Game {
    id: u32,
    moves: Vec<Move>,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.moves.iter().all(|m| m.is_possible())
    }

    fn min_cube_counts(&self) -> CubeCounts {
        let cube_counts: Vec<CubeCounts> = self.moves.iter().map(|m| m.cube_counts()).collect();
        let red = cube_counts.iter().map(|c| c.red).max().unwrap_or_default();
        let green = cube_counts
            .iter()
            .map(|c| c.green)
            .max()
            .unwrap_or_default();
        let blue = cube_counts.iter().map(|c| c.blue).max().unwrap_or_default();
        CubeCounts { red, green, blue }
    }
}

#[derive(Debug)]
struct Move {
    cubes: Vec<Cube>,
}

impl Move {
    fn is_possible(&self) -> bool {
        let counts = self.cube_counts();
        counts.red <= 12 && counts.green <= 13 && counts.blue <= 14
    }

    fn cube_counts(&self) -> CubeCounts {
        let counts: HashMap<Color, u32> = self.cubes.iter().fold(HashMap::new(), |mut m, cube| {
            let e = m.entry(cube.color).or_default();
            *e += cube.count;
            m
        });
        let red = counts.get(&Color::Red).copied().unwrap_or_default();
        let green = counts.get(&Color::Green).copied().unwrap_or_default();
        let blue = counts.get(&Color::Blue).copied().unwrap_or_default();
        CubeCounts { red, green, blue }
    }
}

#[derive(Debug)]
struct CubeCounts {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeCounts {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Cube {
    count: u32,
    color: Color,
}

#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue,
}

fn parse_input(input: &str) -> anyhow::Result<GameList> {
    let (_, games) =
        all_consuming(terminated(separated_list0(line_ending, game), multispace0))(input)
            .map_err(|e| anyhow!(e.to_string()))?;

    Ok(GameList { games })
}
fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = delimited(tag("Game "), complete::u32, tag(": "))(input)?;
    let (input, moves) = moves(input)?;

    Ok((input, Game { id, moves }))
}

fn moves(input: &str) -> IResult<&str, Vec<Move>> {
    let mv = map(separated_list0(tag(", "), cube), |cubes| Move { cubes });
    let (input, moves) = separated_list0(tag("; "), mv)(input)?;
    Ok((input, moves))
}
fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, count) = complete::u32(input)?;
    let (input, _) = space1(input)?;
    let (input, color) = color(input)?;

    Ok((input, Cube { count, color }))
}

fn color(input: &str) -> IResult<&str, Color> {
    let red = map(tag("red"), |_| Color::Red);
    let green = map(tag("green"), |_| Color::Green);
    let blue = map(tag("blue"), |_| Color::Blue);
    alt((red, green, blue))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 8;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 2286;
        assert_eq!(result, expected);
        Ok(())
    }
}
