use std::{fs, mem};

use anyhow::{anyhow, Context};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete;
use nom::character::complete::char;
use nom::IResult;
use nom::multi::{separated_list0, separated_list1};
use nom::Parser;
use nom::sequence::preceded;

fn main() -> anyhow::Result<()> {
    let filename = "day-15.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let (_, strings) = parse_groups(input).map_err(|e| anyhow!(e.to_string()))?;

    let result = strings.iter().map(|s| hash(&s)).sum();

    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut items = parse_input(input)?;
    let mut boxes: Vec<Box> = (0..256).map(|_| Box::default()).collect();
    for item in items.iter_mut() {
        let idx = hash(&item.label);
        let b = &mut boxes[idx];
        // println!("After: {:?}", item);
        match item.operation {
            Operation::Remove => {
                if let Some((idx, _)) = b.items.iter().enumerate().find(|(_, i)| i.label == item.label) {
                    b.items.remove(idx);
                }
            }
            Operation::Insert(_focal_length) => {
                if let Some(it) = b.items.iter_mut().find(|i| i.label == item.label) {
                    mem::swap(it, item);
                } else {
                    b.items.push(item.clone());
                }
            }
        };
        // print_boxes(&boxes);
    }
    let mut result = 0;
    for (box_idx, b) in boxes.iter().enumerate() {
        for (item_idx, item) in b.items.iter().enumerate() {
            let focal_length = match item.operation {
                Operation::Insert(f) => f,
                Operation::Remove => unreachable!()
            };
            result += (box_idx + 1) * (item_idx + 1) * focal_length as usize;
        }
    }
    Ok(result)
}

fn print_boxes(boxes: &[Box]) {
    for (i, b) in boxes.iter().enumerate() {
        if !b.is_empty() {
            println!("Box({}): {:?}", i, b);
        }
    }
}

fn hash(s: &str) -> usize {
    s.bytes().fold(0usize, |n, c| {
        ((n + c as usize) * 17) % 256
    })
}

#[derive(Debug, Default)]
struct Box {
    items: Vec<Item>,
}

impl Box {
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[derive(Debug, Clone)]
struct Item {
    label: String,
    operation: Operation,
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Remove,
    Insert(u32),
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Item>> {
    let (_, items) = parse_items(input).map_err(|e| anyhow!(e.to_string()))?;
    Ok(items)
}

fn parse_items(input: &str) -> IResult<&str, Vec<Item>> {
    separated_list1(char(','), parse_item)(input)
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    let (input, label) = is_not("=-")(input)?;
    let (input, operation) = parse_operation(input)?;
    Ok((input, Item { label: label.to_string(), operation }))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let remove = tag("-").map(|_| Operation::Remove);
    let insert = preceded(char('='), complete::u32).map(Operation::Insert);
    alt((remove, insert))(input)
}

fn parse_groups(input: &str) -> IResult<&str, Vec<String>> {
    separated_list0(tag(","), is_not(",").map(|b: &str| b.chars().filter(|&c| c != '\n').collect()))(input)
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
