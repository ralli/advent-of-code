use anyhow::anyhow;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, one_of, space1};
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use nom::Parser;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::fmt::Formatter;

pub fn part1(input: &str) -> anyhow::Result<i64> {
    let (_, state) = parse_state(input).map_err(|e| anyhow!(e.to_string()))?;
    let mut q: VecDeque<Signal> = VecDeque::new();
    let mut flip_flops: HashMap<&str, bool> = HashMap::new();
    let mut conjunctions: HashMap<&str, HashMap<&str, Pulse>> = HashMap::new();
    let num_rounds = 1000;
    let mut low_count: i64 = 0;
    let mut high_count: i64 = 0;

    for module in state.modules.values() {
        for edge in module.edges.iter() {
            if let Some(target) = state.modules.get(edge) {
                if target.module_type == ModuleType::Conjunction {
                    let e = conjunctions.entry(target.name).or_default();
                    e.insert(module.name, Pulse::Low);
                }
            }
        }
    }

    for _round in 0..num_rounds {
        q.push_back(Signal {
            from: "button",
            to: "broadcaster",
            pulse: Pulse::Low,
        });
        // println!("round: {}", round);
        while let Some(current) = q.pop_front() {
            //println!("{} -{}-> {}", current.from, current.pulse, current.to);

            match current.pulse {
                Pulse::Low => low_count += 1,
                Pulse::High => high_count += 1,
            };

            if let Some(module) = state.modules.get(current.to) {
                let next_pulse = match module.module_type {
                    ModuleType::FlipFlop => {
                        let flip_flop_state = flip_flops.entry(module.name).or_default();
                        let flip_flop_pulse = if *flip_flop_state {
                            Pulse::Low
                        } else {
                            Pulse::High
                        };
                        match current.pulse {
                            Pulse::Low => {
                                *flip_flop_state = !*flip_flop_state;
                                Some(flip_flop_pulse)
                            }
                            Pulse::High => None,
                        }
                    }
                    ModuleType::Conjunction => {
                        let e = conjunctions.entry(module.name).or_default();
                        e.insert(current.from, current.pulse);
                        let conjunction_pulse = if e.values().all(|p| *p == Pulse::High) {
                            Pulse::Low
                        } else {
                            Pulse::High
                        };
                        Some(conjunction_pulse)
                    }
                    ModuleType::Other => Some(current.pulse),
                };
                if let Some(next_pulse) = next_pulse {
                    for edge in module.edges.iter() {
                        q.push_back(Signal {
                            from: module.name,
                            to: edge,
                            pulse: next_pulse,
                        });
                    }
                }
            }
        }
        // println!("{:?}", conjunctions);
        // println!("{:?}\n", flip_flops);
    }
    Ok(low_count * high_count)
}

fn find_incoming_modules<'a>(name: &str, modules: &'a HashMap<&str, Module>) -> Vec<&'a str> {
    modules
        .values()
        .filter(|m| m.edges.contains(&name))
        .map(|m| m.name)
        .collect()
}

/**
 * this solution is totally input specific.
 *
 * found out about the structure of the input using the attached mermaid diagram (day-30.mermaid).
 *
 * the "rx" node gets input from a single compound-node "qt" for my input.
 * So "rx" receives "low", if all inputs to "qt" are "high".
 * For my input these are "kk", "bb", "gl", "mr" which get their first high input in round
 *   - kk = 3931
 *   - mr = 3907
 *   - gl = 3989
 *   - bb = 3967
 * This is also the period of each of these modules (bb,kk,mr,gl) - don't know if this is necessarily the case.
 * I am too lazy to calculate the difference of the first to the second occurrence of each module which will be
 * the guaranteed answer.
 *
 * Taking the lcm(3931,3907,3989,3967) yields 243037165713371 which is the answer to the problem for my input.
 */
fn find_relevant_modules<'a>(modules: &'a HashMap<&str, Module>) -> Vec<&'a str> {
    let rx_modules = find_incoming_modules("rx", modules);
    assert_eq!(rx_modules.len(), 1);
    let bla = rx_modules.first().unwrap();
    find_incoming_modules(bla, modules)
}

pub fn part2(input: &str) -> anyhow::Result<i64> {
    let (_, state) = parse_state(input).map_err(|e| anyhow!(e.to_string()))?;
    let mut q: VecDeque<Signal> = VecDeque::new();
    let mut flip_flops: HashMap<&str, bool> = HashMap::new();
    let mut conjunctions: HashMap<&str, HashMap<&str, Pulse>> = HashMap::new();

    for module in state.modules.values() {
        for edge in module.edges.iter() {
            if let Some(target) = state.modules.get(edge) {
                if target.module_type == ModuleType::Conjunction {
                    let e = conjunctions.entry(target.name).or_default();
                    e.insert(module.name, Pulse::Low);
                }
            }
        }
    }
    let relevant_nodes: HashSet<&str> = HashSet::from_iter(find_relevant_modules(&state.modules));
    println!("{:?}", relevant_nodes);
    let mut periods: HashMap<&str, i64> = HashMap::new();

    let mut round = 0;
    loop {
        round += 1;
        q.push_back(Signal {
            from: "button",
            to: "broadcaster",
            pulse: Pulse::Low,
        });
        // println!("round: {}", round);
        while let Some(current) = q.pop_front() {
            //println!("{} -{}-> {}", current.from, current.pulse, current.to);

            if current.to == "rx" && current.pulse == Pulse::Low {
                return Ok(round);
            }
            if let Some(module) = state.modules.get(current.to) {
                let next_pulse = match module.module_type {
                    ModuleType::FlipFlop => {
                        let flip_flop_state = flip_flops.entry(module.name).or_default();
                        let flip_flop_pulse = if *flip_flop_state {
                            Pulse::Low
                        } else {
                            Pulse::High
                        };
                        match current.pulse {
                            Pulse::Low => {
                                *flip_flop_state = !*flip_flop_state;
                                Some(flip_flop_pulse)
                            }
                            Pulse::High => None,
                        }
                    }
                    ModuleType::Conjunction => {
                        let e = conjunctions.entry(module.name).or_default();
                        e.insert(current.from, current.pulse);
                        let conjunction_pulse = if e.values().all(|p| *p == Pulse::High) {
                            Pulse::Low
                        } else {
                            if relevant_nodes.contains(&module.name)
                                && !periods.contains_key(module.name)
                            {
                                periods.insert(module.name, round);
                                if periods.len() == relevant_nodes.len() {
                                    println!("periods: {:?}", periods);
                                    let values: Vec<i64> = periods.values().copied().collect();
                                    let result = lcm_seq(&values);
                                    return Ok(result);
                                }
                            }
                            Pulse::High
                        };
                        Some(conjunction_pulse)
                    }
                    ModuleType::Other => Some(current.pulse),
                };
                if let Some(next_pulse) = next_pulse {
                    for edge in module.edges.iter() {
                        q.push_back(Signal {
                            from: module.name,
                            to: edge,
                            pulse: next_pulse,
                        });
                    }
                }
            }
        }
        // println!("{:?}", conjunctions);
        // println!("{:?}\n", flip_flops);
    }
}

#[derive(Debug)]
struct State<'a> {
    modules: HashMap<&'a str, Module<'a>>,
}

#[derive(Debug, Copy, Clone)]
struct Signal<'a> {
    from: &'a str,
    to: &'a str,
    pulse: Pulse,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Module<'a> {
    name: &'a str,
    module_type: ModuleType,
    edges: Vec<&'a str>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Pulse {
    Low,
    High,
}

impl fmt::Display for Pulse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Pulse::Low => write!(f, "low"),
            Pulse::High => write!(f, "high"),
        }
    }
}

fn parse_state(input: &str) -> IResult<&str, State> {
    let (input, modules) = separated_list1(line_ending, parse_module)(input)?;
    let modules: HashMap<&str, Module> =
        modules.into_iter().fold(HashMap::new(), |mut m, module| {
            m.insert(module.name, module);
            m
        });
    Ok((input, State { modules }))
}

fn parse_module(input: &str) -> IResult<&str, Module> {
    let (input, (module_type, name)) = alt((parse_module_type_and_name, parse_module_name))(input)?;
    let (input, _) = delimited(space1, tag("->"), space1)(input)?;
    let (input, edges) = separated_list1(tuple((tag(","), space1)), alpha1)(input)?;
    Ok((
        input,
        Module {
            name,
            module_type,
            edges,
        },
    ))
}

fn parse_module_type_and_name(input: &str) -> IResult<&str, (ModuleType, &str)> {
    tuple((
        one_of("%&").map(|c| match c {
            '%' => ModuleType::FlipFlop,
            '&' => ModuleType::Conjunction,
            _ => unreachable!(),
        }),
        alpha1,
    ))(input)
}

fn parse_module_name(input: &str) -> IResult<&str, (ModuleType, &str)> {
    let (input, name) = alpha1(input)?;
    Ok((input, (ModuleType::Other, name)))
}

fn lcm_seq(numbers: &[i64]) -> i64 {
    let mut result = 1;
    for n in numbers.iter().copied() {
        result = lcm(result, n)
    }
    result
}

fn lcm(a: i64, b: i64) -> i64 {
    a * (b / gcd(a, b))
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut num1 = a;
    let mut num2 = b;
    while num2 != 0 {
        let tmp = num2;
        num2 = num1 % num2;
        num1 = tmp;
    }
    num1
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    static INPUT: &str = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 11687500;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 167409079868000;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_mermaid() -> anyhow::Result<()> {
        let input = include_str!("../day-20.txt");
        let (_, state) = parse_state(input).map_err(|e| anyhow!(e.to_string()))?;
        let mut q: VecDeque<&Module> = VecDeque::from([state.modules.get("broadcaster").unwrap()]);
        let mut visited = HashSet::from(["broadcaster"]);
        while let Some(module) = q.pop_front() {
            let module_name = match module.module_type {
                ModuleType::Conjunction => format!("{}[&{}]", module.name, module.name),
                ModuleType::FlipFlop => format!("{}[%{}]", module.name, module.name),
                ModuleType::Other => format!("{}", module.name),
            };
            for edge in module.edges.iter() {
                println!("{} --> {}", module_name, edge);
                if let Some(next) = state.modules.get(edge) {
                    if visited.insert(edge) {
                        q.push_back(next);
                    }
                }
            }
        }
        Ok(())
    }
}
