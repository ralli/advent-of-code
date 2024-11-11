use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use anyhow::{anyhow, Context};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, space0, space1};
use nom::multi::separated_list0;
use nom::sequence::terminated;
use nom::IResult;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() -> anyhow::Result<()> {
    let filename = "day-25.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;

    let result = part1(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let (_, state) = parse_state(input).map_err(|e| anyhow!(e.to_string()))?;
    let mut labels = BTreeSet::new();
    for (start, edges) in state.adj.iter() {
        labels.insert(start);
        for v in edges.iter() {
            labels.insert(v);
        }
    }
    let ids: BTreeMap<&str, usize> =
        BTreeMap::from_iter(labels.iter().enumerate().map(|(i, v)| (**v, i)));

    let mut edges = Vec::new();
    for (start, dests) in state.adj.iter() {
        let u = ids.get(start).copied().unwrap();
        for e in dests.iter() {
            let v = ids.get(e).copied().unwrap();
            edges.push(Edge { u: u, v: v });
        }
    }

    // karger_min_cut finds multiple solutions.
    // use the first, that removes 3 edges...
    let (n1, n2): (usize, usize) = (0..)
        .map(|_| karger_min_cut(&edges, ids.len()))
        .find(|(_n1, _n2, count)| *count == 3)
        .map(|(n1, n2, _count)| (n1, n2))
        .unwrap();

    Ok(n1 * n2)
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone, Copy)]
struct Edge {
    u: usize,
    v: usize,
}

fn karger_min_cut(edges: &[Edge], num_vertices: usize) -> (usize, usize, usize) {
    let mut subsets = DisjointSet::new(num_vertices);
    let mut rng = thread_rng();
    let mut vertices = num_vertices;

    while vertices > 2 {
        let edge = edges.choose(&mut rng).unwrap();

        let set1 = subsets.find(edge.u);
        let set2 = subsets.find(edge.v);

        if set1 != set2 {
            vertices -= 1;
            subsets.union(set1, set2);
        }
    }

    // calculate the sizes of the two groups
    let mut hist = BTreeMap::new();
    for i in 0..num_vertices {
        let set = subsets.find(i);
        *hist.entry(set).or_default() += 1;
    }
    let values: Vec<usize> = hist.values().copied().collect();

    // calculate the number of edges to be removed
    let count = edges
        .iter()
        .filter(|e| {
            let s1 = subsets.find(e.u);
            let s2 = subsets.find(e.v);
            s1 != s2
        })
        .count();

    (values[0], values[1], count)
}

struct DisjointSet {
    rank: Vec<usize>,
    parent: Vec<usize>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        let rank: Vec<usize> = (0..size).map(|_| 0).collect();
        let parent: Vec<usize> = (0..size).collect();
        Self { parent, rank }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root == y_root {
            return;
        }

        match self.rank[x_root].cmp(&self.rank[y_root]) {
            std::cmp::Ordering::Less => {
                self.parent[x_root] = y_root;
            }
            std::cmp::Ordering::Greater => {
                self.parent[y_root] = x_root;
            }
            std::cmp::Ordering::Equal => {
                self.parent[y_root] = x_root;
                self.rank[x_root] += 1;
            }
        }
    }
}

fn parse_state(input: &str) -> IResult<&str, State> {
    let (input, edge_list) = separated_list0(line_ending, parse_edge)(input)?;
    let adj = BTreeMap::from_iter(edge_list);
    Ok((input, State { adj }))
}

fn parse_edge(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let (input, start) = alpha1(input)?;
    let (input, _) = terminated(tag(":"), space0)(input)?;
    let (input, edges) = separated_list0(space1, alpha1)(input)?;

    Ok((input, (start, edges)))
}

// fn print_mermaid(state: &State) {
//     println!("flowchart");
//
//     for (v, edges) in state.adj.iter() {
//         for e in edges.iter() {
//             println!("  {v} --- {e}");
//         }
//     }
// }

#[derive(Debug)]
struct State<'a> {
    adj: BTreeMap<&'a str, Vec<&'a str>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 54;
        assert_eq!(result, expected);
        Ok(())
    }
}
