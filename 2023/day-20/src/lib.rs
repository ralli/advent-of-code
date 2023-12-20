use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fmt::Formatter;
use anyhow::anyhow;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, one_of, space1};
use nom::IResult;
use nom::multi::separated_list1;
use nom::Parser;
use nom::sequence::{delimited, tuple};

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
        q.push_back(Signal { from: "button", to: "broadcaster", pulse: Pulse::Low });
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
                        let flip_flop_pulse = if *flip_flop_state { Pulse::Low } else { Pulse::High };
                        match current.pulse {
                            Pulse::Low => {
                                *flip_flop_state = !*flip_flop_state;
                                Some(flip_flop_pulse)
                            }
                            Pulse::High => {
                                None
                            }
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
                    ModuleType::Other => {
                        Some(current.pulse)
                    }
                };
                if let Some(next_pulse) = next_pulse {
                    for edge in module.edges.iter() {
                        q.push_back(Signal { from: module.name, to: edge, pulse: next_pulse });
                    }
                }
            }
        }
        // println!("{:?}", conjunctions);
        // println!("{:?}\n", flip_flops);
    }
    Ok(low_count * high_count)
}

pub fn part2(_input: &str) -> anyhow::Result<i64> {
    Ok(0)
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
    let modules: HashMap<&str, Module> = modules.into_iter().fold(HashMap::new(), |mut m, module| {
        m.insert(module.name, module);
        m
    });
    Ok((input, State { modules }))
}

fn parse_module(input: &str) -> IResult<&str, Module> {
    let (input, (module_type, name)) = alt((parse_module_type_and_name, parse_module_name))(input)?;
    let (input, _) = delimited(space1, tag("->"), space1)(input)?;
    let (input, edges) = separated_list1(tuple((tag(","), space1)), alpha1)(input)?;
    Ok((input, Module { name, module_type, edges }))
}

fn parse_module_type_and_name(input: &str) -> IResult<&str, (ModuleType, &str)> {
    tuple(
        (
            one_of("%&").map(|c| match c {
                '%' => ModuleType::FlipFlop,
                '&' => ModuleType::Conjunction,
                _ => unreachable!()
            }),
            alpha1
        )
    )(input)
}

fn parse_module_name(input: &str) -> IResult<&str, (ModuleType, &str)> {
    let (input, name) = alpha1(input)?;
    Ok((input, (ModuleType::Other, name)))
}


#[cfg(test)]
mod tests {
    use super::*;

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
}
