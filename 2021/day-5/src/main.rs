use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

use anyhow::Context;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space0, space1};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let filename = "./day-5/input.txt";
    let content = read_file(filename).context(filename)?;

    let result = part1(&content);
    println!("{}", result);

    let result = part2(&content);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> usize {
    let (_, lines) = lines(input).unwrap();
    let mut points: HashMap<(i32, i32), i32> = HashMap::new();

    for line in lines
        .iter()
        .filter(|l| l.p1.0 == l.p2.0 || l.p1.1 == l.p2.1)
    {
        let x1 = line.p1.0.min(line.p2.0);
        let x2 = line.p1.0.max(line.p2.0);
        let y1 = line.p1.1.min(line.p2.1);
        let y2 = line.p1.1.max(line.p2.1);

        for x in x1..=x2 {
            for y in y1..=y2 {
                *points.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    points.values().filter(|&&c| c > 1).count()
}

fn part2(input: &str) -> usize {
    let (_, lines) = lines(input).unwrap();
    let mut points: HashMap<(i32, i32), i32> = HashMap::new();

    for line in lines.iter() {
        let (x1, y1) = line.p1;
        let (x2, y2) = line.p2;
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();

        let mut x = x1;
        let mut y = y1;
        *points.entry((x, y)).or_insert(0) += 1;

        while x != x2 || y != y2 {
            x += dx;
            y += dy;
            *points.entry((x, y)).or_insert(0) += 1;
        }
    }

    points.values().filter(|&&c| c > 1).count()
}

#[derive(Debug)]
struct Line {
    p1: (i32, i32),
    p2: (i32, i32),
}

fn lines(input: &str) -> IResult<&str, Vec<Line>> {
    separated_list1(line_ending, line)(input)
}

fn line(input: &str) -> IResult<&str, Line> {
    use nom::character::complete::i32 as i32_parser;
    let (input, p1) = point(input)?;
    let (input, _) = delimited(space1, tag("->"), space1)(input)?;
    let (input, p2) = point(input)?;

    Ok((input, Line { p1, p2 }))
}

fn point(input: &str) -> IResult<&str, (i32, i32)> {
    use nom::character::complete::i32 as i32_parser;
    let (input, x) = i32_parser(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = i32_parser(input)?;

    Ok((input, (x, y)))
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

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 5;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 12;
        assert_eq!(result, expected);
    }
}
