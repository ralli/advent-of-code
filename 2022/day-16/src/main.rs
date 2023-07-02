/// Did not get part 2 of my own solution to work.
/// this solution is based on the work of hyper-neutrino
/// Python implementation: https://github.com/hyper-neutrino/advent-of-code/blob/main/2022/day16p2.py
/// Video: https://www.youtube.com/watch?v=bLMj50cpOug&list=PLnNm9syGLD3yf-YW-a5XNh1CJN07xr0Kz&index=16
///
/// Chose this solution as it looks relatively simple to me.
///
/// The basic idea of part 1 is a brute force approach:
///   valves, opening and tunnels form a directed graph (all weights are 1).
///   traverse all paths through that graph and get the maximum flow (sum of all opened valves)
///   Optimization: make the graph more compact by removing all valves with flow 0.
///   Use a depth first search through that compact graph. Memoize intermediate results.
///
/// Part 2: The human and the elephant can act independently as long as they try to open a
/// distinct set of valves.
///
/// Generate all permutations of open valves.
///
/// For each permutation
///   get the optimum result of the human for that permutation.
///   get the optimum result for the elephant for the inverted permutation (if valve[i] is open
///   for the human, it is closed for the elephant an vice versa)
///
/// As hyper-neutrinos solution this solution uses bit-vectors as representation of open and
/// closed valves.
///
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
    let (_, valves) = valves(input).unwrap();
    let max_time = 30;
    let start = "AA";
    let mut searcher = MaxFlowSearcher::new(&valves);
    searcher.max_flow(max_time, start, 0)
}

fn part2(input: &str) -> i32 {
    let (_, valves) = valves(input).unwrap();
    let non_empty_count = valves.values().filter(|v| v.flow > 0).count();
    let num_tries = (1u32 << non_empty_count) - 1;
    let mut searcher = MaxFlowSearcher::new(&valves);
    let max_time = 26;
    let start = "AA";

    (0..((num_tries + 1) / 2))
        .map(|i| searcher.max_flow(max_time, start, i) + searcher.max_flow(max_time, start, !i))
        .max()
        .unwrap()
}

#[derive(Debug, Clone, Copy)]
struct ValveDistance<'a> {
    name: &'a str,
    distance: i32,
}

fn all_distances_from<'a>(
    valves: &'a HashMap<&str, Valve>,
) -> HashMap<&'a str, Vec<ValveDistance<'a>>> {
    valves
        .values()
        .filter(|v| v.name == "AA" || v.flow > 0)
        .map(|valve| (valve.name, distances_from(valve.name, valves)))
        .collect()
}

fn distances_from<'a>(
    start_valve: &str,
    valves: &'a HashMap<&str, Valve>,
) -> Vec<ValveDistance<'a>> {
    let mut visited = HashSet::from([start_valve]);
    let mut distances: Vec<ValveDistance> = Vec::new();
    let mut q = VecDeque::from([(0, valves.get(start_valve).unwrap())]);

    while let Some((distance, current)) = q.pop_front() {
        for neighbour in current.tunnels.iter() {
            if visited.insert(neighbour) {
                let neighbour_valve = valves.get(neighbour).unwrap();
                if neighbour_valve.flow > 0 {
                    distances.push(ValveDistance {
                        name: neighbour,
                        distance: distance + 1,
                    });
                }
                q.push_back((distance + 1, neighbour_valve));
            }
        }
    }

    distances
}

fn valve_indexes<'a>(valves: &'a HashMap<&str, Valve>) -> HashMap<&'a str, u32> {
    valves
        .values()
        .filter(|v| v.flow > 0)
        .enumerate()
        .map(|(i, v)| (v.name, i as u32))
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct CacheKey<'a> {
    valve: &'a str,
    time_remaining: i32,
    valves_open: u32,
}

struct MaxFlowSearcher<'a> {
    valves: &'a HashMap<&'a str, Valve<'a>>,
    all_distances: HashMap<&'a str, Vec<ValveDistance<'a>>>,
    valve_indexes: HashMap<&'a str, u32>,
    cache: HashMap<CacheKey<'a>, i32>,
}

impl<'a> MaxFlowSearcher<'a> {
    fn new(valves: &'a HashMap<&'a str, Valve>) -> Self {
        Self {
            valves,
            all_distances: all_distances_from(valves),
            valve_indexes: valve_indexes(valves),
            cache: HashMap::new(),
        }
    }

    fn max_flow(&mut self, time: i32, valve: &'a str, valves_open: u32) -> i32 {
        let key = CacheKey {
            time_remaining: time,
            valve,
            valves_open,
        };
        if let Some(result) = self.cache.get(&key) {
            return *result;
        }
        let mut max_val = 0;

        let distances = self.all_distances.get(valve).cloned().unwrap();
        for neighbour in distances {
            let valve_index = self.valve_indexes.get(neighbour.name).copied().unwrap();
            let bit = 1u32 << valve_index;
            if (valves_open & bit) != 0 {
                continue;
            }
            let time_left = time - neighbour.distance - 1;
            if time_left <= 0 {
                continue;
            }
            let flow = self.valves.get(neighbour.name).unwrap().flow;
            max_val = max_val.max(
                time_left * flow + self.max_flow(time_left, neighbour.name, valves_open | bit),
            );
        }

        self.cache.insert(key, max_val);
        max_val
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Valve<'a> {
    name: &'a str,
    flow: i32,
    tunnels: Vec<&'a str>,
}

fn valves(input: &str) -> IResult<&str, HashMap<&str, Valve>> {
    let (input, valves) = separated_list1(line_ending, valve)(input)?;
    let valve_map = valves
        .into_iter()
        .map(|valve| (valve.name, valve))
        .collect();

    Ok((input, valve_map))
}

fn valve(input: &str) -> IResult<&str, Valve> {
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
        Valve {
            name: node,
            flow: flow_rate,
            tunnels: next_nodes,
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
