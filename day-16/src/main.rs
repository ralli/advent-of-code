use std::collections::{HashMap, HashSet, VecDeque};

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
    let total_time = 30;

    let (_, nodes) = nodes(input).unwrap();
    let weights: HashMap<&str, i32> = nodes.iter().map(|(s, f, _)| (*s, *f)).collect();
    let adj: HashMap<&str, &Vec<&str>> = nodes.iter().map(|(s, _, n)| (*s, n)).collect();

    let mut max_flow = 0;

    let mut q = VecDeque::new();
    let mut best: HashMap<&str, i32> = HashMap::new();

    let start = "AA";
    q.push_back((start, 0, total_time, HashSet::<&str>::new()));
    // q.push_back((start, *f, total_time - 1, vec![start]));

    while !q.is_empty() {
        let (current, current_flow, time_left, opened) = q.pop_front().unwrap();

        if current_flow > max_flow {
            max_flow = current_flow;
        }

        if time_left > 0 && !opened.contains(&current) {
            let w = *weights.get(current).unwrap();
            if w > 0 {
                let mut next_opened = opened.clone();
                next_opened.insert(current);
                let next_flow = (time_left - 1) * w + current_flow;
                q.push_back((current, next_flow, time_left - 1, next_opened));
            }
        }

        if time_left > 0 {
            let next = *adj.get(current).unwrap();
            for s in next.iter() {
                let best_flow = best.entry(*s).or_insert(-1);
                if current_flow > *best_flow {
                    *best_flow = current_flow;
                    q.push_back((s, current_flow, time_left - 1, opened.clone()));
                }
            }
        }
    }

    max_flow
}

type Move<'a> = (&'a str, &'a str, i32, i32, Vec<&'a str>);

fn part2(input: &str) -> i32 {
    let total_time = 30;

    let (_, nodes) = nodes(input).unwrap();
    let weights: HashMap<&str, i32> = nodes.iter().map(|(s, f, _)| (*s, *f)).collect();
    let adj: HashMap<&str, &Vec<&str>> = nodes.iter().map(|(s, _, n)| (*s, n)).collect();

    let mut max_flow = 0;

    let mut q: VecDeque<Move> = VecDeque::new();
    let mut best: HashMap<(&str, &str, Vec<&str>), i32> = HashMap::new();

    let start = "AA";
    q.push_back((start, start, 0, total_time, Vec::<&str>::new()));
    // q.push_back((start, *f, total_time - 1, vec![start]));

    while !q.is_empty() {
        let (current, elephant_current, current_flow, time_left, opened) = q.pop_front().unwrap();

        println!(
            "{} {} {} {} {:?}",
            current, elephant_current, current_flow, time_left, &opened
        );

        if current_flow > max_flow {
            max_flow = current_flow;
        }

        if time_left > 0 && !opened.contains(&current) {
            let w = *weights.get(current).unwrap();
            if w > 0 {
                let mut next_opened = opened.clone();
                next_opened.push(current);
                let next_flow = (time_left - 1) * w + current_flow;
                let em = elephant_moves(
                    current,
                    elephant_current,
                    next_flow,
                    time_left,
                    &next_opened,
                    &weights,
                    &adj,
                );
                for m in em {
                    q.push_back(m);
                }
            }
        }

        if time_left > 0 {
            let next = *adj.get(current).unwrap();
            for s in next.iter() {
                let em = elephant_moves(
                    s,
                    elephant_current,
                    current_flow,
                    time_left,
                    &opened,
                    &weights,
                    &adj,
                );
                for m in em {
                    let best_flow = best.entry((m.0, m.1, opened.clone())).or_insert(-1);
                    let cf = m.3;
                    if cf > *best_flow {
                        *best_flow = cf;
                        q.push_back(m);
                    }
                }
            }
        }
    }

    max_flow
}

fn elephant_moves<'a>(
    current: &'a str,
    elephant_current: &'a str,
    current_flow: i32,
    time_left: i32,
    opened: &Vec<&'a str>,
    weights: &HashMap<&'a str, i32>,
    adj: &HashMap<&'a str, &Vec<&'a str>>,
) -> Vec<Move<'a>> {
    let mut result = Vec::new();

    if !opened.contains(&elephant_current) {
        let w = *weights.get(elephant_current).unwrap();
        if w > 0 {
            let mut next_opened = opened.clone();
            next_opened.push(elephant_current);
            let next_flow = (time_left - 1) * w + current_flow;
            result.push((
                current,
                elephant_current,
                next_flow,
                time_left - 1,
                next_opened,
            ));
        }
    }

    let next = adj.get(elephant_current).unwrap();
    for s in next.iter() {
        result.push((current, s, current_flow, time_left - 1, opened.clone()));
    }

    result
}

type Node<'a> = (&'a str, i32, Vec<&'a str>);

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

    Ok((input, (node, flow_rate, next_nodes)))
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
