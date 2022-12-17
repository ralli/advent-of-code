use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, line_ending};
use nom::multi::separated_list1;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let input = read_file("./day-16/input.txt")?;

    let result = part1(&input);
    println!("{}", result);

    let result = part2(&input);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> i32 {
    let (_, nodes) = nodes(input).unwrap();
    let mut solver = Solver::new();
    solver.find_solution(&nodes)
}

struct Solver<'a> {
    cache: HashMap<(&'a str, Vec<&'a str>, i32), i32>,
}

impl<'a> Solver<'a> {
    fn new() -> Self {
        Solver {
            cache: HashMap::new(),
        }
    }

    fn find_solution(&mut self, nodes: &'a [Node]) -> i32 {
        let node_map: HashMap<&str, &Node> = nodes.iter().map(|n| (n.from, n)).collect();
        let open = Vec::new();
        self.find("AA", &node_map, &open, 30)
    }

    fn find(
        &mut self,
        id: &'a str,
        node_map: &std::collections::HashMap<&str, &Node<'a>>,
        open: &[&'a str],
        time: i32,
    ) -> i32 {
        if time == 0 {
            return 0;
        }

        let key = (id, open.to_vec(), time);
        if let Some(r) = self.cache.get(&key) {
            return *r;
        }
        let mut result = 0;
        let node = node_map.get(id).unwrap();
        if node.flow > 0 && !open.contains(&node.from) {
            let mut next_open: Vec<&'a str> = open.to_owned();
            next_open.push(node.from);
            result =
                result.max((time - 1) * node.flow + self.find(id, node_map, &next_open, time - 1));
        }

        for s in node.next_nodes.iter() {
            result = result.max(self.find(s, node_map, open, time - 1));
        }

        self.cache.insert((id, open.to_vec(), time), result);
        result
    }
}

fn part2(_input: &str) -> i32 {
    todo!()
}

#[derive(Debug)]
struct Node<'a> {
    from: &'a str,
    flow: i32,
    next_nodes: Vec<&'a str>,
}

fn nodes(input: &str) -> IResult<&str, Vec<Node>> {
    separated_list1(line_ending, node)(input)
}

fn node(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, node) = alpha1(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, flow_rate) = complete::i32(input)?;
    let (input, _) = alt((
        tag("; tunnels lead to valves "),
        tag("; tunnel leads to valve "),
    ))(input)?;
    let (input, next_nodes) = separated_list1(tag(", "), alpha1)(input)?;

    Ok((
        input,
        Node {
            from: node,
            flow: flow_rate,
            next_nodes: next_nodes,
        },
    ))
}

fn read_file(filename: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 1651;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 1707;
        assert_eq!(result, expected);
    }
}
