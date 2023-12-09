use std::fs;

use anyhow::{anyhow, Context};
use nom::character::complete;
use nom::character::complete::{line_ending, space1};
use nom::multi::separated_list1;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let filename = "day-9.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i64> {
    let sequences = parse_input(input)?;
    let result = sequences
        .iter()
        .map(|s| value_for_sequence(s.as_slice()))
        .sum();
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<i64> {
    let sequences = parse_input(input)?;
    let result = sequences
        .iter()
        .map(|s| value_for_sequence2(s.as_slice()))
        .sum();
    Ok(result)
}

fn value_for_sequence(sequence: &[i64]) -> i64 {
    let mut current = sequence.to_vec();
    let mut sequences: Vec<i64> = Vec::new();
    sequences.push(current.last().copied().unwrap_or_default());
    loop {
        current = next_sequence(&current);
        sequences.push(current.last().copied().unwrap_or_default());
        if current.iter().all(|&n| n == 0) {
            break;
        }
    }
    sequences
        .into_iter()
        .reduce(|a, b| a + b)
        .expect("must not be empty")
}

fn value_for_sequence2(sequence: &[i64]) -> i64 {
    let mut current = sequence.to_vec();
    let mut sequences: Vec<i64> = Vec::new();
    sequences.push(current.first().copied().unwrap_or_default());
    loop {
        current = next_sequence(&current);
        sequences.push(current.first().copied().unwrap_or_default());
        if current.iter().all(|&n| n == 0) {
            break;
        }
    }
    sequences
        .into_iter()
        .rev()
        .reduce(|a, b| b - a)
        .expect("must not be empty")
}

fn next_sequence(sequence: &[i64]) -> Vec<i64> {
    sequence.windows(2).map(|w| w[1] - w[0]).collect()
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<i64>>> {
    let (_, seqs) = parse_sequences(input).map_err(|e| anyhow!(e.to_string()))?;
    Ok(seqs)
}

fn parse_sequences(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    let sequence = separated_list1(space1, complete::i64);
    let (input, seqs) = separated_list1(line_ending, sequence)(input)?;
    Ok((input, seqs))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 114;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 2;
        assert_eq!(result, expected);
        Ok(())
    }
}
