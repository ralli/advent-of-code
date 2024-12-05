use anyhow::anyhow;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::newline;
use nom::multi::{many1, separated_list0, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fs;

fn main() -> anyhow::Result<()> {
    let filename = "day-5/day-5.txt";
    let content = fs::read_to_string(filename)?;

    let result = part1(&content)?;
    println!("{}", result);

    let result = part2(&content)?;
    println!("{}", result);

    Ok(())
}

fn part1(content: &str) -> anyhow::Result<u32> {
    let (_, input) = parse_input(content).map_err(|e| anyhow!("{e}"))?;
    let edge_set = BTreeSet::from_iter(input.edges.iter().copied());
    let result = input
        .updates
        .iter()
        .filter(|update| all_pages_sorted(&edge_set, update))
        .map(|update| {
            if update.is_empty() {
                0
            } else {
                update[update.len() / 2]
            }
        })
        .sum();
    Ok(result)
}

fn part2(content: &str) -> anyhow::Result<u32> {
    let (_, input) = parse_input(content).map_err(|e| anyhow!("{e}"))?;
    let edge_set = BTreeSet::from_iter(input.edges.iter().copied());
    let result = input
        .updates
        .iter()
        .filter(|update| !all_pages_sorted(&edge_set, update))
        .map(|update| sorted_pages(&edge_set, update))
        .map(|update| {
            if update.is_empty() {
                0
            } else {
                update[update.len() / 2]
            }
        })
        .sum();
    Ok(result)
}

fn sorted_pages(edge_set: &BTreeSet<Edge>, pages: &[u32]) -> Vec<u32> {
    let mut page_vec = pages.to_vec();
    page_vec.sort_unstable_by(|a, b| {
        if edge_set.contains(&(*a, *b)) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    page_vec
}

fn all_pages_sorted(edge_set: &BTreeSet<Edge>, pages: &[u32]) -> bool {
    pages.windows(2).all(|w| edge_set.contains(&(w[0], w[1])))
}

type Edge = (u32, u32);

#[derive(Debug, Clone)]
struct Input {
    edges: Vec<Edge>,
    updates: Vec<Vec<u32>>,
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (rest, edges) = parse_edges(input)?;
    let (rest, _) = many1(newline)(rest)?;
    let (rest, updates) = parse_updates(rest)?;
    // let (rest, _) = terminated(multispace0, eof)(input)?;
    Ok((rest, Input { edges, updates }))
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list0(newline, parse_pages)(input)
}

fn parse_pages(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), complete::u32)(input)
}
fn parse_edges(input: &str) -> IResult<&str, Vec<Edge>> {
    separated_list0(newline, parse_edge)(input)
}

fn parse_edge(input: &str) -> IResult<&str, Edge> {
    separated_pair(complete::u32, tag("|"), complete::u32)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        let result = part1(input)?;
        assert_eq!(result, 143);
        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        let result = part2(input)?;
        assert_eq!(result, 123);
        Ok(())
    }
}
