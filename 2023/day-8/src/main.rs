use std::fs;
use std::collections::BTreeMap;

use anyhow::{anyhow, Context};
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, line_ending, one_of, space1};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, terminated};

fn main() -> anyhow::Result<()> {
    let filename = "day-8.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i64> {
    let network = parse_input(input)?;
    let num_steps = path_length(&network, "AAA");
    Ok(num_steps)
}

fn path_length(network: &Network, start: &str) -> i64 {
    let mut instructions = network.instructions.iter().cycle();
    let node_map: BTreeMap<&str, &Node> = network.nodes.iter().fold(BTreeMap::new(), |mut m, n| {
        m.insert(n.id.as_str(), n);
        m
    });
    let mut num_steps = 0;
    let mut current = start;
    loop {
        if current == "ZZZ" {
            break;
        }
        let instruction = instructions.next().unwrap();
        let node = node_map.get(&current).expect("node");
        current = match *instruction {
            'L' => node.left.as_str(),
            'R' => node.right.as_str(),
            _ => unreachable!("{}", *instruction),
        };
        num_steps += 1;
    }
    num_steps
}

fn path_length2(network: &Network, start: &str) -> i64 {
    let mut instructions = network.instructions.iter().cycle();
    let node_map: BTreeMap<&str, &Node> = network.nodes.iter().fold(BTreeMap::new(), |mut m, n| {
        m.insert(n.id.as_str(), n);
        m
    });
    let mut num_steps = 0;
    let mut current = start;
    loop {
        if current.ends_with('Z') {
            break;
        }
        let instruction = instructions.next().unwrap();
        let node = node_map.get(&current).expect("node");
        current = match *instruction {
            'L' => node.left.as_str(),
            'R' => node.right.as_str(),
            _ => unreachable!("{}", *instruction),
        };
        num_steps += 1;
    }
    num_steps
}

fn part2(input: &str) -> anyhow::Result<i64> {
    let network = parse_input(input)?;
    let starts: Vec<&str> = network.nodes.iter().map(|n| n.id.as_str()).filter(|s| s.ends_with('A')).collect();
    let path_lengths: Vec<_> = starts.iter().map(|s| path_length2(&network, s)).collect();
    let path_length = path_lengths.into_iter().reduce(lcm).unwrap();
    Ok(path_length)
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

#[derive(Debug)]
struct Network {
    instructions: Vec<char>,
    nodes: Vec<Node>,
}

#[derive(Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

fn parse_input(input: &str) -> anyhow::Result<Network> {
    let (_, network) = parse_network(input).map_err(|e| anyhow!(e.to_string()))?;
    Ok(network)
}

fn parse_network(input: &str) -> IResult<&str, Network> {
    let (input, instructions) = many1(one_of("LR"))(input)?;
    let (input, _) = many1(line_ending)(input)?;
    let (input, nodes) = separated_list1(line_ending, parse_node)(input)?;
    Ok((input, Network { instructions, nodes }))
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    let (input, id) = alphanumeric1(input)?;
    let (input, _) = delimited(space1, tag("="), space1)(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, left) = alphanumeric1(input)?;
    let (input, _) = terminated(tag(","), space1)(input)?;
    let (input, right) = alphanumeric1(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, Node { id: id.to_string(), left: left.to_string(), right: right.to_string() }))
}


#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 2;
        assert_eq!(result, expected);
        Ok(())
    }

    static INPUT2: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT2)?;
        let expected = 6;
        assert_eq!(result, expected);
        Ok(())
    }
}