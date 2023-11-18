use std::{collections::BTreeSet, fs};

use anyhow::Context;
use nom::{
    character::complete::{alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list0,
    IResult,
};

fn main() -> anyhow::Result<()> {
    let filename = "day-6.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannnot load {filename}"))?;
    let result = part1(&input)?;
    println!("{result}");
    let result = part2(&input)?;
    println!("{result}");
    Ok(())
}

#[derive(Debug)]
struct Group {
    persons: Vec<Person>,
}

impl Group {
    fn all_answers(&self) -> BTreeSet<char> {
        self.persons.iter().fold(BTreeSet::new(), |mut v, p| {
            v.extend(p.answers.iter().copied());
            v
        })
    }

    fn common_answers(&self) -> BTreeSet<char> {
        let mut result = BTreeSet::new();
        for (i, v) in self.persons.iter().enumerate() {
            if i == 0 {
                result = v.answers.clone();
            } else {
                result = result.intersection(&v.answers).copied().collect();
            }
        }
        result
    }
}

#[derive(Debug, Clone)]
struct Person {
    answers: BTreeSet<char>,
}

impl Person {
    fn new(input: &str) -> Self {
        Self {
            answers: BTreeSet::from_iter(input.chars()),
        }
    }
}

fn part1(input: &str) -> anyhow::Result<u32> {
    let groups = parse_input(input)?;
    Ok(groups
        .iter()
        .map(|group| group.all_answers().len() as u32)
        .sum())
}

fn part2(input: &str) -> anyhow::Result<u32> {
    let groups = parse_input(input)?;
    Ok(groups
        .iter()
        .map(|group| group.common_answers().len() as u32)
        .sum())
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Group>> {
    let (_, g) = all_consuming(groups)(input).map_err(|e| anyhow::Error::msg(e.to_string()))?;
    Ok(g)
}

fn groups(input: &str) -> IResult<&str, Vec<Group>> {
    separated_list0(line_ending, group)(input)
}

fn group(input: &str) -> IResult<&str, Group> {
    let (input, persons) = separated_list0(line_ending, person)(input)?;
    Ok((input, Group { persons }))
}

fn person(input: &str) -> IResult<&str, Person> {
    let (input, chars) = alpha1(input)?;
    Ok((input, Person::new(chars)))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 11;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 6;
        assert_eq!(result, expected);
        Ok(())
    }
}
