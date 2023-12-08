use std::fs;

use anyhow::{anyhow, Context};
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, multispace0, space1};
use nom::combinator::all_consuming;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, terminated, tuple};
use nom::{character::complete, IResult};

fn main() -> anyhow::Result<()> {
    let filename = "day-4.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;
    let result = part1(&input)?;
    println!("{result}");
    let result = part2(&input)?;
    println!("{result}");
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<u32> {
    let cards = parse_input(input)?;
    let result: u32 = cards
        .iter()
        .map(|card| {
            card.numbers
                .iter()
                .filter(|number| card.winning.contains(number))
                .copied()
                .collect::<Vec<u32>>()
        })
        .map(|v| v.len() as u32)
        .map(pow2)
        .sum();
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let cards = parse_input(input)?;
    let result = number_of_cards(&cards);
    Ok(result)
}

fn number_of_cards(cards: &[Card]) -> usize {
    let mut memo: Vec<usize> = vec![1; cards.len() + 1];

    for idx in 1..=cards.len() {
        let current = memo[idx];
        let matching_cards: Vec<_> = cards[idx - 1]
            .numbers
            .iter()
            .filter(|n| cards[idx - 1].winning.contains(n))
            .collect();
        let matching: Vec<_> = (0..matching_cards.len()).map(|i| 1 + i + idx).collect();
        for i in matching {
            memo[i] += current;
        }
    }
    let result: usize = memo.iter().sum();
    result - 1
}

fn pow2(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    let mut result = 1;
    let mut x = n - 1;
    while x > 0 {
        result *= 2;
        x -= 1;
    }
    result
}

#[derive(Debug)]
struct Card {
    id: u32,
    numbers: Vec<u32>,
    winning: Vec<u32>,
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Card>> {
    let (_, cards) =
        all_consuming(terminated(separated_list0(line_ending, card), multispace0))(input)
            .map_err(|e| anyhow!(e.to_string()))?;
    Ok(cards)
}
fn card(input: &str) -> IResult<&str, Card> {
    let (input, id) = delimited(
        tuple((tag("Card"), space1)),
        complete::u32,
        tuple((tag(":"), space1)),
    )(input)?;
    let (input, numbers) = separated_list1(space1, complete::u32)(input)?;
    let (input, _) = delimited(space1, tag("|"), space1)(input)?;
    let (input, winning) = separated_list1(space1, complete::u32)(input)?;
    Ok((
        input,
        Card {
            id,
            numbers,
            winning,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 13;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 30;
        assert_eq!(result, expected);
        Ok(())
    }
}
