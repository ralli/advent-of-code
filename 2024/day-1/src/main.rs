use anyhow::{anyhow, Context};
use nom::character::complete;
use nom::character::complete::{multispace0, newline, space1};
use nom::multi::separated_list0;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;
use std::collections::BTreeMap;
use std::path::Path;
use std::{fmt, fs};

fn main() -> anyhow::Result<()> {
    let content = read_file("day-1/day-1.txt")?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i64> {
    let (mut list1, mut list2) = parse_input(&input)?;
    list1.sort();
    list2.sort();
    Ok(list1
        .iter()
        .zip(list2.iter())
        .map(|(i1, i2)| (i2 - i1).abs())
        .sum())
}

fn part2(input: &str) -> anyhow::Result<i64> {
    let (list1, list2) = parse_input(&input)?;
    let hist2: BTreeMap<i64, i64> = list2.iter().fold(BTreeMap::new(), |mut m, n| {
        let e = m.entry(*n).or_insert(0);
        *e += 1;
        m
    });
    let sum1 = list1
        .iter()
        .map(|n| {
            let count = hist2.get(n).unwrap_or(&0);
            *n * count
        })
        .sum();

    Ok(sum1)
}

type Input = (Vec<i64>, Vec<i64>);

fn parse_input(input: &str) -> anyhow::Result<Input> {
    let (_, input) = terminated(parse_lists, multispace0)(input).map_err(|e| anyhow!("{e}"))?;
    Ok(input)
}

fn parse_lists(input: &str) -> IResult<&str, Input> {
    let (rest, pairs) = separated_list0(newline, parse_pair)(input)?;
    let mut list1 = Vec::with_capacity(pairs.len());
    let mut list2 = Vec::with_capacity(pairs.len());

    for (s1, s2) in pairs.into_iter() {
        list1.push(s1);
        list2.push(s2);
    }

    Ok((rest, (list1, list2)))
}

fn parse_pair(input: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(complete::i64, space1, complete::i64)(input)
}

fn read_file(filename: impl AsRef<Path> + fmt::Display) -> anyhow::Result<String> {
    fs::read_to_string(filename.as_ref()).context(format!("cannot load file {}", filename))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        let result = part2(input)?;
        assert_eq!(31, result);

        Ok(())
    }
}
