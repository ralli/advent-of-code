extern crate core;

use std::cmp::Ordering;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::{many1, separated_list0, separated_list1};
use nom::sequence::delimited;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let input = read_file("./day-13/input.txt")?;
    let result = part1(&input);

    println!("{}", result);

    let result = part2(&input);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> usize {
    let (_, sequence_pairs) = sequence_pairs(input).unwrap();
    sequence_pairs
        .iter()
        .enumerate()
        .map(|(i, (a, b))| if a < b { i + 1 } else { 0 })
        .sum()
}

fn part2(input: &str) -> usize {
    let (_, sequence_pairs) = sequence_pairs(input).unwrap();
    let mut packets = Vec::new();

    for (a, b) in sequence_pairs {
        packets.push(a);
        packets.push(b);
    }

    let first = Item::List(vec![Item::List(vec![Item::Constant(2)])]);
    let second = Item::List(vec![Item::List(vec![Item::Constant(6)])]);

    packets.push(first.clone());
    packets.push(second.clone());

    packets.sort();

    let first_idx = 1 + packets.iter().position(|x| x == &first).unwrap();
    let second_idx = 1 + packets.iter().position(|x| x == &second).unwrap();

    first_idx * second_idx
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    List(Vec<Item>),
    Constant(i32),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        use Item::*;

        match (self, other) {
            (Constant(a), Constant(b)) => a.cmp(b),
            (List(a), List(b)) => a.cmp(b),
            (Constant(a), List(b)) => vec![Constant(*a)].cmp(b),
            (List(a), Constant(b)) => {
                let vec1 = vec![Constant(*b)];
                a.cmp(&vec1)
            }
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn sequence_pairs(input: &str) -> IResult<&str, Vec<(Item, Item)>> {
    separated_list1(many1(line_ending), sequence_pair)(input)
}

fn sequence_pair(input: &str) -> IResult<&str, (Item, Item)> {
    let (input, first) = sequence(input)?;
    let (input, _) = line_ending(input)?;
    let (input, second) = sequence(input)?;
    Ok((input, (Item::List(first), Item::List(second))))
}

fn sequence(input: &str) -> IResult<&str, Vec<Item>> {
    delimited(tag("["), separated_list0(tag(","), item), tag("]"))(input)
}

fn item(input: &str) -> IResult<&str, Item> {
    alt((
        map(sequence, Item::List),
        map(nom::character::complete::i32, Item::Constant),
    ))(input)
}

fn read_file(filename: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 13;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 140;
        assert_eq!(result, expected);
    }
}
