use anyhow::{anyhow, Context};
use nom::character::complete;
use nom::character::complete::{multispace0, newline, space1};
use nom::multi::separated_list0;
use nom::sequence::terminated;
use nom::IResult;
use std::path::Path;
use std::{fmt, fs};

fn main() -> anyhow::Result<()> {
    let content = read_file("day-2/day-2.txt")?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let reports = parse_input(input)?;
    let result = reports.iter().filter(|report| is_safe(report)).count();
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let reports = parse_input(input)?;
    let result = reports.iter().filter(|report| is_safe2(report)).count();
    Ok(result)
}

fn is_safe(arr: &[i32]) -> bool {
    let all_increasing = arr.windows(2).all(|w| w[0] < w[1]);
    let all_decreasing = arr.windows(2).all(|w| w[0] > w[1]);
    let in_range = arr.windows(2).all(|w| {
        let diff = (w[0] - w[1]).abs();
        (1..=3).contains(&diff)
    });
    (all_increasing || all_decreasing) && in_range
}

fn is_safe2(arr: &[i32]) -> bool {
    for (i, _) in arr.iter().enumerate() {
        let mut values = arr.to_vec();
        values.remove(i);
        if is_safe(&values) {
            return true;
        }
    }
    false
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<i32>>> {
    let (_, result) = terminated(separated_list0(newline, parse_report), multispace0)(input)
        .map_err(|e| anyhow!("{e}"))?;
    Ok(result)
}

fn parse_report(input: &str) -> IResult<&str, Vec<i32>> {
    let (rest, list) = separated_list0(space1, complete::i32)(input)?;
    Ok((rest, list))
}

fn read_file(filename: impl AsRef<Path> + fmt::Display) -> anyhow::Result<String> {
    fs::read_to_string(filename.as_ref()).context(format!("cannot load file {}", filename))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        let result = part1(input)?;
        assert_eq!(2, result);
        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        let result = part2(input)?;
        assert_eq!(4, result);
        Ok(())
    }
}
