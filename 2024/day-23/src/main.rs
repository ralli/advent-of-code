use anyhow::anyhow;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, multispace0};
use nom::combinator::eof;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

fn main() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("day-23/input.txt")?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(_input: &str) -> anyhow::Result<usize> {
    let (_, edges) = parse_edge_list(_input).map_err(|e| anyhow!("{e}"))?;
    let adj = build_adj_list(&edges);
    let triples = find_all_triples(&adj);
    let filtered_triples = triples
        .iter()
        .filter(|triple| triple.iter().any(|s| s.starts_with('t')));
    let result = filtered_triples.count();
    Ok(result)
}

fn part2(_input: &str) -> anyhow::Result<String> {
    let (_, edges) = parse_edge_list(_input).map_err(|e| anyhow!("{e}"))?;
    let adj = build_adj_list(&edges);
    let results = find_all(&adj);
    let result = results.iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap();
    let result: Vec<_> = result.iter().map(|s| s.to_string()).collect();
    Ok(result.join(","))
}

type Edge<'a> = (&'a str, &'a str);
type EdgeList<'a> = Vec<Edge<'a>>;
type AdjList<'a> = BTreeMap<&'a str, BTreeSet<&'a str>>;

fn find_all_triples<'a>(adj: &AdjList<'a>) -> BTreeSet<Vec<&'a str>> {
    let mut result = BTreeSet::new();
    let empty_set = BTreeSet::new();
    for (x, xs) in adj.iter() {
        for y in xs.iter() {
            let ys = adj.get(y).unwrap_or(&empty_set);
            for z in ys.iter() {
                let zs = adj.get(z).unwrap_or(&empty_set);
                if z != x && zs.contains(x) {
                    let mut v = vec![*x, *y, *z];
                    v.sort();
                    result.insert(v);
                }
            }
        }
    }
    result
}

fn find_all<'a>(adj: &AdjList<'a>) -> BTreeSet<BTreeSet<&'a str>> {
    let mut result = BTreeSet::new();
    let mut q = VecDeque::new();

    for start in adj.keys() {
        q.push_back((*start, BTreeSet::from([*start])));
    }

    while let Some((from, reachable)) = q.pop_front() {
        if result.contains(&reachable) {
            continue;
        }
        let froms = adj.get(from).unwrap();
        for to in froms.iter() {
            if reachable.contains(to) {
                continue;
            }
            if !reachable
                .iter()
                .all(|r| adj.get(r).map(|r| r.contains(to)).unwrap_or(false))
            {
                continue;
            }
            let mut next_reachable = reachable.clone();
            next_reachable.insert(to);
            q.push_back((to, next_reachable));
        }
        result.insert(reachable);
    }

    result
}

fn build_adj_list<'a>(edges: &'a [Edge]) -> AdjList<'a> {
    edges
        .iter()
        .fold(BTreeMap::new(), |mut adj_list, (from, to)| {
            let e = adj_list.entry(from).or_default();
            e.insert(to);
            let e = adj_list.entry(to).or_default();
            e.insert(from);
            adj_list
        })
}

fn parse_edge_list(input: &str) -> IResult<&str, EdgeList> {
    let (input, list) = separated_list0(line_ending, parse_edge)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = eof(input)?;
    Ok((input, list))
}

fn parse_edge(input: &str) -> IResult<&str, Edge> {
    separated_pair(alpha1, tag("-"), alpha1)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        assert_eq!(result, 7);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        println!("{result}");
        assert_eq!(result, "co,de,ka,ta");
        Ok(())
    }
}
