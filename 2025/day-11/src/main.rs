use anyhow::anyhow;
use pathfinding::prelude::*;
use std::collections::BTreeMap;
use winnow::ModalResult;
use winnow::Parser;
use winnow::ascii::{alpha1, line_ending, multispace0, space1};
use winnow::combinator::{eof, separated, separated_pair, terminated};

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day-11.txt").unwrap();

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut inp = input;
    let adj = terminated(parse_adj, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|e| anyhow!("{e}"))?;
    let result = count_paths(
        "you".to_string(),
        |from| adj.get(from).cloned().unwrap_or_default(),
        |s| s == "out",
    );
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut inp = input;
    let adj = terminated(parse_adj, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|e| anyhow!("{e}"))?;
    let mut p2 = Part2::new(adj);
    let result = p2.count_paths("svr", false, false);
    Ok(result)
}

struct Part2 {
    cache: BTreeMap<(String, bool, bool), usize>,
    adj: Adj,
}

impl Part2 {
    fn new(adj: Adj) -> Self {
        Self {
            cache: BTreeMap::new(),
            adj,
        }
    }

    fn count_paths(&mut self, from: &str, dac_seen: bool, fft_seen: bool) -> usize {
        if let Some(&count) = self.cache.get(&(from.to_string(), dac_seen, fft_seen)) {
            return count;
        }

        let from = from.to_string();
        if from == "out" {
            let count = if dac_seen && fft_seen { 1 } else { 0 };
            self.cache.insert((from, dac_seen, fft_seen), count);
            return count;
        }

        let next_dac_seen = dac_seen || from == "dac";
        let next_fft_seen = fft_seen || from == "fft";

        let count = if let Some(nodes) = self.adj.get(&from).cloned() {
            nodes
                .iter()
                .map(|n| self.count_paths(n.as_str(), next_dac_seen, next_fft_seen))
                .sum()
        } else {
            0
        };

        self.cache.insert((from, dac_seen, fft_seen), count);

        count
    }
}

type Adj = BTreeMap<String, Vec<String>>;

fn parse_adj(input: &mut &str) -> ModalResult<Adj> {
    separated(1.., parse_row, line_ending).parse_next(input)
}

fn parse_row(input: &mut &str) -> ModalResult<(String, Vec<String>)> {
    separated_pair(parse_str, ": ", parse_string_list).parse_next(input)
}

fn parse_string_list(input: &mut &str) -> ModalResult<Vec<String>> {
    separated(1.., parse_str, space1).parse_next(input)
}

fn parse_str(input: &mut &str) -> ModalResult<String> {
    alpha1.map(|s: &str| s.to_string()).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;

    #[test]
    fn test_part1() {
        let result = part1(INPUT).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part2() {
        let input = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;
        let result = part2(input).unwrap();
        assert_eq!(result, 2);
    }
}
