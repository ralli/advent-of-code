use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;

use anyhow::Context;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete;
use nom::character::complete::{line_ending, multispace0, space1};
use nom::combinator::{eof, map, opt};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let filename = "day-7.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot read {filename}"))?;
    let result = part1(&input)?;
    println!("{result}");
    let result = part2(&input)?;
    println!("{result}");
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let rules = parse_input(input)?;
    let mut adj: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for rule in rules.iter() {
        for from in rule.contains.iter() {
            let e = adj.entry(from.bag_name.to_string()).or_default();
            e.push(rule.bag_name.to_string());
        }
    }

    let start = "shiny gold";
    let mut visited = BTreeSet::from([start.to_string()]);
    let mut q = VecDeque::from([start.to_string()]);

    while let Some(current) = q.pop_front() {
        if let Some(next) = adj.get(&current) {
            for n in next.iter() {
                if visited.insert(n.to_string()) {
                    q.push_back(n.to_string());
                }
            }
        }
    }

    Ok(visited.len() - 1)
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let rules = parse_input(input)?;
    let mut adj: BTreeMap<String, Vec<ContainedBag>> = BTreeMap::new();
    for rule in rules.into_iter() {
        adj.insert(rule.bag_name, rule.contains);
    }
    let start = "shiny gold";

    fn visit(node: &str, count: u64, adj: &BTreeMap<String, Vec<ContainedBag>>) -> u64 {
        if let Some(next) = adj.get(node) {
            let mut result = count;
            for n in next {
                result += count * visit(&n.bag_name, n.count as u64, adj);
            }
            result
        } else {
            count
        }
    }

    Ok(visit(start, 1, &adj) - 1)
}

#[derive(Debug, Clone)]
struct Rule {
    bag_name: String,
    contains: Vec<ContainedBag>,
}

#[derive(Debug, Clone)]
struct ContainedBag {
    count: u32,
    bag_name: String,
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Rule>> {
    let (_, rules) = terminated(rule_list, tuple((multispace0, eof)))(input)
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
    Ok(rules)
}

fn rule_list(input: &str) -> IResult<&str, Vec<Rule>> {
    separated_list0(line_ending, rule)(input)
}

fn rule(input: &str) -> IResult<&str, Rule> {
    let (input, bag_name) = bag_name(input)?;
    let (input, contains) = contained_bag_list(input)?;
    let (input, _) = tag(".")(input)?;
    Ok((
        input,
        Rule {
            bag_name: bag_name.to_string(),
            contains,
        },
    ))
}

fn bag_name(input: &str) -> IResult<&str, String> {
    let (input, name) = take_until(" bags contain ")(input)?;
    let (input, _) = tag(" bags contain ")(input)?;
    Ok((input, name.to_string()))
}

fn contained_bag_list(input: &str) -> IResult<&str, Vec<ContainedBag>> {
    let p1 = map(tag("no other bags"), |_| Vec::new());
    let p2 = separated_list1(tag(", "), contained_bag);
    let (input, v) = alt((p1, p2))(input)?;
    Ok((input, v))
}

fn contained_bag(input: &str) -> IResult<&str, ContainedBag> {
    let (input, count) = complete::u32(input)?;
    let (input, bag_name) = preceded(space1, take_until(" bag"))(input)?;
    let (input, _) = tag(" bag")(input)?;
    let (input, _) = opt(tag("s"))(input)?;
    Ok((
        input,
        ContainedBag {
            count,
            bag_name: bag_name.to_string(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 4;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 32;
        assert_eq!(result, expected);
        Ok(())
    }

    static INPUT2: &str = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;

    #[test]
    fn part2_works2() -> anyhow::Result<()> {
        let result = part2(INPUT2)?;
        let expected = 126;
        assert_eq!(result, expected);
        Ok(())
    }
}
