use std::{fs, mem};

use anyhow::{anyhow, Context};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete;
use nom::character::complete::char;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

fn main() -> anyhow::Result<()> {
    let filename = "day-15.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;
    let input = input.trim();

    let result = part1(input)?;
    println!("{result}");

    let result = part2(input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let (_, strings) = parse_groups(input).map_err(|e| anyhow!(e.to_string()))?;

    let result = strings.iter().map(|s| hash(s)).sum();

    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let commands = parse_input(input)?;
    let mut boxes: Vec<Box> = (0..256).map(|_| Box::default()).collect();
    for cmd in commands.into_iter() {
        let idx = hash(cmd.label);
        let b = &mut boxes[idx];

        match cmd.operation {
            Operation::Remove => {
                if let Some(idx) = b.items.iter().position(|i| i.label == cmd.label) {
                    b.items.remove(idx);
                }
            }
            Operation::Insert(focal_length) => {
                if let Some(lens) = b.items.iter_mut().find(|l| l.label == cmd.label) {
                    let mut new_lens = Lens {
                        label: cmd.label,
                        focal_length,
                    };
                    mem::swap(lens, &mut new_lens);
                } else {
                    b.items.push(Lens {
                        label: cmd.label,
                        focal_length,
                    });
                }
            }
        };
        // print_boxes(&boxes);
    }
    let result = boxes
        .iter()
        .enumerate()
        .map(|(box_idx, b)| {
            b.items
                .iter()
                .enumerate()
                .map(|(item_idx, item)| (box_idx + 1) * (item_idx + 1) * item.focal_length as usize)
                .sum::<usize>()
        })
        .sum();
    Ok(result)
}

fn hash(s: &str) -> usize {
    s.bytes().fold(0usize, |n, c| ((n + c as usize) * 17) % 256)
}

#[derive(Debug, Default)]
struct Box<'a> {
    items: Vec<Lens<'a>>,
}

#[derive(Debug, Clone)]
struct Command<'a> {
    label: &'a str,
    operation: Operation,
}

#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u32,
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Remove,
    Insert(u32),
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Command>> {
    let (_, items) = parse_commands(input).map_err(|e| anyhow!(e.to_string()))?;
    Ok(items)
}

fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(char(','), parse_command)(input)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, label) = is_not("=-")(input)?;
    let (input, operation) = parse_operation(input)?;
    Ok((input, Command { label, operation }))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let remove = tag("-").map(|_| Operation::Remove);
    let insert = preceded(char('='), complete::u32).map(Operation::Insert);
    alt((remove, insert))(input)
}

fn parse_groups(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(tag(","), is_not(","))(input)
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;

    use super::*;

    static INPUT: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    #[test]
    fn test1() -> anyhow::Result<()> {
        let (_, strings) = parse_groups(INPUT).map_err(|e| anyhow!(e))?;
        println!("{strings:?}");
        Ok(())
    }

    #[test]
    fn test2() -> anyhow::Result<()> {
        let result = hash("HASH");
        let expected = 52;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 1320;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 145;
        assert_eq!(result, expected);
        Ok(())
    }
}
