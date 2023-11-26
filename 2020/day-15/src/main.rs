use std::collections::BTreeMap;

use anyhow::anyhow;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list0;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let input = r#"0,12,6,13,20,1,17"#;
    let result = part1(input)?;
    println!("{result}");
    let result = part2(input)?;
    println!("{result}");
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i32> {
    let (_, numbers) = parse_input(input).map_err(|e| anyhow!(e.to_string()))?;
    let last_spoken = game(&numbers, 2020);
    Ok(last_spoken)
}

fn part2(input: &str) -> anyhow::Result<i32> {
    let (_, numbers) = parse_input(input).map_err(|e| anyhow!(e.to_string()))?;
    let last_spoken = game(&numbers, 30000000);
    Ok(last_spoken)
}

fn game(numbers: &[i32], num_rounds: usize) -> i32 {
    let mut last_turn: BTreeMap<i32, Vec<usize>> =
        BTreeMap::from_iter(numbers.iter().enumerate().map(|(i, v)| (*v, vec![i + 1])));
    let mut last_spoken = numbers.last().copied().unwrap_or_default();
    for turn in numbers.len() + 1..=num_rounds {
        let visits = last_turn.entry(last_spoken).or_default();
        let nvisits = visits.len();
        last_spoken = if nvisits > 1 {
            visits[nvisits - 1] - visits[nvisits - 2]
        } else {
            0
        } as i32;
        let visits = last_turn.entry(last_spoken).or_default();
        visits.push(turn);
        if visits.len() > 2 {
            visits.remove(0);
        }
    }
    last_spoken
}

fn parse_input(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list0(tag(","), complete::i32)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let input = "0,3,6";
        let result = part1(input)?;
        let expected = 436;
        assert_eq!(result, expected);
        Ok(())
    }
}
