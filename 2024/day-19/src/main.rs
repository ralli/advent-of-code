use anyhow::{anyhow, Context};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, multispace0};
use nom::combinator::eof;
use nom::multi::{many1, separated_list0};
use nom::IResult;
use pathfinding::prelude::{bfs, count_paths};
use std::fs;

fn main() -> anyhow::Result<()> {
    let filename = "day-19/input.txt";
    let content = fs::read_to_string(filename).context(format!("cannot load {filename}"))?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let (_, data) = parse_input(input).map_err(|e| anyhow!("{e}"))?;
    // println!("{:?}", data);
    let result = data
        .designs
        .iter()
        .filter(|design| has_solution(design, &data.patterns))
        .count();
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let (_, data) = parse_input(input).map_err(|e| anyhow!("{e}"))?;
    let result = data
        .designs
        .iter()
        .map(|design| count_solutions(design, &data.patterns))
        .sum();
    Ok(result)
}

fn has_solution(design: &str, patterns: &[&str]) -> bool {
    bfs(
        &design.to_string(),
        |suffix| {
            patterns
                .iter()
                .filter_map(|p| suffix.strip_prefix(p).map(|s| s.to_string()))
                .collect::<Vec<_>>()
        },
        |suffix| suffix.is_empty(),
    )
    .is_some()
}
// fn has_solution(design: &str, patterns: &[&str]) -> bool {
//     let mut q = VecDeque::from([design]);
//     let mut visited = HashSet::new();
//
//     while let Some(current) = q.pop_front() {
//         if !visited.insert(current) {
//             continue;
//         }
//         if current.is_empty() {
//             return true;
//         }
//         for p in patterns.iter() {
//             if let Some(next) = current.strip_prefix(p) {
//                 q.push_back(next);
//             }
//         }
//     }
//
//     false
// }

fn count_solutions(design: &str, patterns: &[&str]) -> usize {
    count_paths(
        design,
        |suffix| {
            patterns
                .iter()
                .filter_map(|p| suffix.strip_prefix(p))
                .collect::<Vec<_>>()
        },
        |suffix| suffix.is_empty(),
    )
}

#[derive(Debug)]
struct Input<'a> {
    patterns: Vec<&'a str>,
    designs: Vec<&'a str>,
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, patterns) = parse_patterns(input)?;
    let (input, _) = many1(line_ending)(input)?;
    let (input, designs) = parse_designs(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = eof(input)?;

    Ok((input, Input { patterns, designs }))
}

fn parse_patterns(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(tag(", "), alpha1)(input)
}

fn parse_designs(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(line_ending, alpha1)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        assert_eq!(result, 6);
        Ok(())
    }

    #[test]
    fn test_count_solutions() {
        let patterns = ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
        let design = "brwrr";
        let result = count_solutions(design, &patterns);
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        assert_eq!(result, 16);
        Ok(())
    }
}
