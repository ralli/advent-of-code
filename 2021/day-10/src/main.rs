extern crate core;

use std::fs::File;
use std::io::Read;

use anyhow::Context;
use nom::character::complete::{line_ending, one_of};
use nom::combinator::recognize;
use nom::multi::{many1, separated_list1};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let filename = "./day-10/input.txt";
    let content = read_file(filename).context(filename)?;

    let result = part1(&content);
    println!("{}", result);

    let result = part2(&content);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> i32 {
    let (_, lines) = lines(input).unwrap();
    lines
        .iter()
        .filter_map(|&line| first_incorrect(line))
        .map(|(found, _)| score_for_part1(found))
        .sum()
}

fn part2(input: &str) -> i64 {
    let (_, lines) = lines(input).unwrap();
    let bla: Vec<_> = lines
        .iter()
        .filter_map(|line| missing_closing(line))
        .collect();
    let mut scores: Vec<_> = bla
        .iter()
        .map(|s| s.chars().fold(0, |a, c| a * 5 + score_for_part2(c)))
        .collect();
    scores.sort();
    scores[scores.len() / 2]
}

fn score_for_part1(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        c => unreachable!("{}", c),
    }
}

fn score_for_part2(c: char) -> i64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        c => unreachable!("{}", c),
    }
}

fn first_incorrect(input: &str) -> Option<(char, char)> {
    let mut stack = Vec::new();

    for c in input.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                if let Some(open) = stack.pop() {
                    let wanted = closing_character(open);
                    if c != wanted {
                        return Some((c, wanted));
                    }
                }
            }
            c => unreachable!("{}", c),
        }
    }
    None
}

fn missing_closing(input: &str) -> Option<String> {
    let mut stack = Vec::new();

    for c in input.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                if let Some(open) = stack.pop() {
                    let wanted = closing_character(open);
                    if wanted != c {
                        return None;
                    }
                }
            }
            c => unreachable!("{}", c),
        }
    }

    Some(stack.iter().rev().map(|c| closing_character(*c)).collect())
}

fn closing_character(open: char) -> char {
    match open {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!("{}", open),
    }
}

fn lines(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(line_ending, line)(input)
}

fn line(input: &str) -> IResult<&str, &str> {
    recognize(many1(one_of("()[]{}<>")))(input)
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
    fn hase() {
        let input = "[({(<(())[]>[[{[]{<()<>>";
        let result = first_incorrect(input);
        assert_eq!(result, None);
    }

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 26397;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 288957;
        assert_eq!(result, expected);
    }
}
