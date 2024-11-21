extern crate core;

use anyhow::{anyhow, Context};
use core::fmt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, line_ending};
use nom::multi::separated_list0;
use nom::IResult;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let content = read_file("day-6/day-6.txt")?;
    let result = part1(&content)?;
    println!("{result}");
    let result = part2(&content)?;
    println!("{result}");
    Ok(())
}

#[derive(Debug, Clone)]
struct Edge {
    from: String,
    to: String,
}

fn part1(input: &str) -> anyhow::Result<i32> {
    let adj = parse_input(input)?;
    let mut q = VecDeque::from([(0, "COM".to_string())]);
    let mut result = 0;

    while let Some((d, current)) = q.pop_front() {
        result += d;
        if let Some(a) = adj.get(&current) {
            for b in a.iter() {
                q.push_back((d + 1, b.to_string()))
            }
        }
    }

    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<i32> {
    let adj = parse_input(input)?;
    let inv = invert(&adj);
    let mut q = VecDeque::from([(0, "YOU".to_string())]);
    let mut visited: BTreeSet<String> = BTreeSet::from(["YOU".to_string()]);

    while let Some((d, current)) = q.pop_front() {
        if current == "SAN" {
            return Ok(d - 2);
        }
        if let Some(a) = adj.get(&current) {
            for b in a.iter() {
                if visited.insert(b.to_string()) {
                    q.push_back((d + 1, b.to_string()))
                }
            }
        }
        if let Some(a) = inv.get(&current) {
            for b in a.iter() {
                if visited.insert(b.to_string()) {
                    q.push_back((d + 1, b.to_string()))
                }
            }
        }
    }

    Err(anyhow!("'SAN' not found"))
}

fn invert(adj: &BTreeMap<String, Vec<String>>) -> BTreeMap<String, Vec<String>> {
    let mut result = BTreeMap::new();
    for (from, bla) in adj.iter() {
        for to in bla.iter() {
            let v: &mut Vec<String> = result.entry(to.to_string()).or_default();
            v.push(from.to_string());
        }
    }
    result
}

fn parse_input(input: &str) -> anyhow::Result<BTreeMap<String, Vec<String>>> {
    let (_, edges) = parse_edges(input).map_err(|e| anyhow!(e.to_string()))?;
    let adj = edges.into_iter().fold(
        BTreeMap::new(),
        |mut acc: BTreeMap<String, Vec<String>>, e| {
            let ent = acc.entry(e.from).or_default();
            ent.push(e.to);
            acc
        },
    );
    Ok(adj)
}

fn parse_edges(input: &str) -> IResult<&str, Vec<Edge>> {
    separated_list0(line_ending, parse_edge)(input)
}

fn parse_edge(input: &str) -> IResult<&str, Edge> {
    let (rest, from) = alphanumeric1(input)?;
    let (rest, _) = tag(")")(rest)?;
    let (rest, to) = alphanumeric1(rest)?;
    Ok((
        rest,
        Edge {
            from: from.to_string(),
            to: to.to_string(),
        },
    ))
}

fn read_file(filename: impl AsRef<Path> + fmt::Display) -> anyhow::Result<String> {
    fs::read_to_string(filename.as_ref()).with_context(|| format!("cannot read file {filename}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let input = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"#;
        let result = part1(input)?;
        assert_eq!(result, 42);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let input = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"#;
        let result = part2(input)?;
        assert_eq!(result, 4);
        Ok(())
    }
}
