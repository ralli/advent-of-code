extern crate core;

use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;

use itertools::EitherOrBoth;
use itertools::Itertools;
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
        .map(|(i, (a, b))| {
            // println!("== Pair {} ==", i + 1);
            if a < b {
                i + 1
            } else {
                0
            }
        })
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

    // for p in packets.iter() {
    //     println!("{}", p);
    // }

    let first_idx = 1 + packets.iter().position(|x| x == &first).unwrap();
    let second_idx = 1 + packets.iter().position(|x| x == &second).unwrap();

    first_idx * second_idx
}

#[derive(Debug, Clone)]
enum Item {
    List(Vec<Item>),
    Constant(i32),
}

fn compare(a: &Item, b: &Item) -> Ordering {
    use Item::*;
    let result = match (a, b) {
        (Constant(a_value), Constant(b_value)) => a_value.cmp(b_value),
        (List(a_list), List(b_list)) => a_list
            .iter()
            .zip_longest(b_list.iter())
            .map(|x| match x {
                EitherOrBoth::Right(_y) => {
                    // println!("left is empty right {}", y);
                    Ordering::Less
                }
                EitherOrBoth::Left(_y) => {
                    // println!("right is empty left {}", y);
                    Ordering::Greater
                }
                EitherOrBoth::Both(aaa, bbb) => compare(aaa, bbb),
            })
            .find(|&c| c != Ordering::Equal)
            .unwrap_or(Ordering::Equal),
        (List(_), Constant(b_value)) => {
            let b_list = List(vec![Constant(*b_value)]);
            compare(a, &b_list)
        }
        (Constant(a_value), List(_)) => {
            let a_list = List(vec![Constant(*a_value)]);
            compare(&a_list, b)
        }
    };

    // println!("compare {} {} {:?}", a, b, result);

    result
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        compare(self, other)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare(self, other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        compare(self, other) == Ordering::Equal
    }
}

impl Eq for Item {}

impl Default for Item {
    fn default() -> Self {
        Item::Constant(i32::MIN)
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Item::Constant(n) => write!(f, "{}", n),
            Item::List(lst) => {
                write!(f, "[")?;
                for (i, v) in lst.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
        }
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
