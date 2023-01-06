extern crate core;

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::File;
use std::io::Read;

use anyhow::Context;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending};
use nom::multi::separated_list1;
use nom::{IResult, Parser};

fn main() -> anyhow::Result<()> {
    let filename = "./day-12/input.txt";
    let content = read_file(filename).context(filename)?;

    let result = part1(&content);
    println!("{}", result);

    let result = part2(&content);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> usize {
    let (_, edges) = edges(input).unwrap();
    let adj: BTreeMap<&str, Vec<&str>> = build_adj(&edges);
    num_paths(&adj)
}

fn part2(input: &str) -> usize {
    let (_, edges) = edges(input).unwrap();
    let adj: BTreeMap<&str, Vec<&str>> = build_adj(&edges);
    num_paths2(&adj)
}

fn build_adj<'a>(edges: &[Edge<'a>]) -> BTreeMap<&'a str, Vec<&'a str>> {
    let mut adj: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for edge in edges.into_iter() {
        let e = adj.entry(edge.from).or_default();

        if edge.to != "start" {
            e.push(edge.to);
        }

        let e = adj.entry(edge.to).or_default();
        if edge.from != "start" {
            e.push(edge.from);
        }
    }
    adj
}

fn num_paths(adj: &BTreeMap<&str, Vec<&str>>) -> usize {
    let mut stack = vec![("start", vec!["start"])];
    let mut result = 0;

    while let Some((from, path)) = stack.pop() {
        if from == "end" {
            result += 1;
            continue;
        }
        for to in adj.get(from).unwrap().iter() {
            if to.chars().next().unwrap().is_lowercase() && path.contains(to) {
                continue;
            }
            let mut next_path = path.to_vec();
            next_path.push(to);
            stack.push((to, next_path));
        }
    }

    result
}

fn num_paths2(adj: &BTreeMap<&str, Vec<&str>>) -> usize {
    let mut stack = vec![("start", vec!["start"])];
    let mut result = 0;

    while let Some((from, path)) = stack.pop() {
        if from == "end" {
            result += 1;
            continue;
        }
        for to in adj.get(from).unwrap().iter() {
            if !check_valid(&path, to) {
                continue;
            }
            let mut next_path = path.to_vec();
            next_path.push(to);
            stack.push((to, next_path));
        }
    }

    result
}

fn first_is_lowercase(s: &str) -> bool {
    s.chars().next().map(|c| c.is_lowercase()).unwrap_or(false)
}

fn check_valid(path: &[&str], to: &str) -> bool {
    if !first_is_lowercase(to) {
        return true;
    }

    let hist = create_hist(path);
    let count = hist.get(to).copied().unwrap_or(0);

    if to == "start" || to == "end" && count > 0 {
        return false;
    }

    let max_count = hist.values().max().copied().unwrap_or(0);

    if max_count < 2 {
        return count <= 1;
    }

    count == 0
}

fn create_hist<'a>(path: &[&'a str]) -> HashMap<&'a str, i32> {
    let mut hist = HashMap::new();
    for node in path.into_iter().filter(|n| first_is_lowercase(*n)) {
        let entry = hist.entry(*node).or_insert(0);
        *entry += 1;
    }
    hist
}

#[derive(Debug)]
struct Edge<'a> {
    from: &'a str,
    to: &'a str,
}

fn edges(input: &str) -> IResult<&str, Vec<Edge>> {
    separated_list1(line_ending, edge)(input)
}

fn edge(input: &str) -> IResult<&str, Edge> {
    let (input, from) = alpha1(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, to) = alpha1(input)?;

    Ok((input, Edge { from, to }))
}

fn read_file(name: &str) -> anyhow::Result<String> {
    let mut f = File::open(name)?;
    let mut result = String::new();
    f.read_to_string(&mut result)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test.txt");

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 10;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 36;
        assert_eq!(result, expected);
    }
}
