use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs;

use anyhow::{anyhow, Context};
use nom::character::complete;
use nom::character::complete::{line_ending, one_of, space1};
use nom::IResult;
use nom::multi::{many1, separated_list1};

fn main() -> anyhow::Result<()> {
    let filename = "day-7.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i64> {
    let hands = parse_input(input)?;
    let mut hands: Vec<_> = hands.into_iter().map(|h| Part1Hand::new(h.cards, h.bid)).collect();
    hands.sort_by(|a, b| a.score.cmp(&b.score));
    let result = hands.iter().enumerate().map(|(i, h)| (i + 1) as i64 * h.bid).sum();
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<i64> {
    let hands = parse_input(input)?;
    let mut hands: Vec<_> = hands.into_iter().map(|h| Part2Hand::new(h.cards, h.bid)).collect();
    hands.sort_by(compare);
    let result = hands.iter().enumerate().map(|(i, h)| (i + 1) as i64 * h.bid).sum();
    Ok(result)
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: i64,
}

struct Part1Hand {
    bid: i64,
    score: i64,
}

struct Part2Hand {
    bid: i64,
    value: i64,
    score: i64,
}

impl Part2Hand {
    fn new(cards: String, bid: i64) -> Self {
        let (value, _) = joker_hand_value(&cards);
        let score = cards.chars().fold(0, |n, c| n * 14 + card_score(c, 1));
        Part2Hand { bid, value, score }
    }
}

fn compare(h1: &Part2Hand, h2: &Part2Hand) -> Ordering {
    let v = h1.value.cmp(&h2.value);
    if v == Ordering::Equal {
        h1.score.cmp(&h2.score)
    } else {
        v
    }
}

impl Part1Hand {
    fn new(cards: String, bid: i64) -> Self {
        let score = hand_score(&cards, 11);
        Self { bid, score }
    }
}

fn joker_hand_value(cards: &str) -> (i64, String) {
    let result = "AKQT98765432".chars().map(|card| {
        let new_cards: String = cards.chars().map(|c| if c == 'J' { card } else { c }).collect();
        (hand_value(&new_cards), new_cards)
    }).max_by(|(s1, _), (s2, _)| s1.cmp(s2)).unwrap_or_default();
    result
}

fn hand_score(cards: &str, joker_value: i64) -> i64 {
    cards.chars().fold(hand_value(cards), |n, c| {
        n * 15 + card_score(c, joker_value)
    })
}

fn hand_value(cards: &str) -> i64 {
    let hist: BTreeMap<char, i64> = cards.chars().fold(BTreeMap::new(), |mut m, c|
        {
            let n = m.entry(c).or_default();
            *n += 1;
            m
        },
    );

    if hist.values().any(|n| *n == 5) {
        return 6;
    }

    if hist.values().any(|n| *n == 4) {
        return 5;
    }

    let has_three = hist.values().any(|n| *n == 3);
    let has_two = hist.values().any(|n| *n == 2);

    if has_three && has_two {
        return 4;
    }

    if has_three {
        return 3;
    }

    let two_pair = hist.values().filter(|&n| *n == 2).count() == 2;
    if two_pair {
        return 2;
    }

    if has_two {
        return 1;
    }

    0
}

fn card_score(card: char, joker_value: i64) -> i64 {
    match card {
        '2'..='9' => card.to_digit(10).unwrap() as i64,
        'T' => 10,
        'J' => joker_value,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!("{card}")
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Hand>> {
    let (_, hands) = parse_hands(input).map_err(|e| anyhow!(e.to_string()))?;
    Ok(hands)
}


fn parse_hands(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(line_ending, parse_hand)(input)
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, cards) = many1(one_of("AKQJT98765432"))(input)?;
    let (input, _) = space1(input)?;
    let (input, bid) = complete::i64(input)?;
    let cards: String = cards.into_iter().collect();

    Ok((input, Hand { cards, bid }))
}


#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 6440;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 5905;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn hand_score_test_1() {
        let s1 = hand_score("AAJAA", 1);
        let s2 = hand_score("JJJJJ", 1);
        assert!(s1 < s2);
    }

    #[test]
    fn test_parse_hand() -> anyhow::Result<()> {
        let s1 = parse_hand("AAJAA 123")?;
        println!("{s1:?}");
        Ok(())
    }
}