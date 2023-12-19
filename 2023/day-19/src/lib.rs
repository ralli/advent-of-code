use std::collections::HashMap;

use anyhow::anyhow;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, line_ending, one_of};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded};
use nom::{character, IResult};

pub fn part1(input: &str) -> anyhow::Result<i64> {
    let (_, state) = parse_state(input).map_err(|e| anyhow!(e.to_string()))?;
    let result: i64 = state
        .parts
        .iter()
        .filter(|part| state_from_start(&state.workflows, part) == "A")
        .map(|part| part.x + part.m + part.a + part.s)
        .sum();
    Ok(result)
}

pub fn part2(input: &str) -> anyhow::Result<i64> {
    let (_, state) = parse_state(input).map_err(|e| anyhow!(e.to_string()))?;
    let start_ranges = Ranges {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    };
    let result = count_solutions(&state.workflows, &start_ranges, "in");
    Ok(result)
}

fn count_solutions(workflows: &HashMap<&str, Workflow>, ranges: &Ranges, name: &str) -> i64 {
    if name == "R" {
        0
    } else if name == "A" {
        ranges.num_choices()
    } else {
        let workflow = workflows.get(name).unwrap();

        let mut ranges = *ranges;
        let mut result = 0;
        for rule in workflow.rules.iter() {
            match rule {
                Rule::Compare {
                    name,
                    op,
                    value,
                    next,
                } => {
                    let (a, b) = ranges.get(*name);
                    let matching_range = match op {
                        '<' => (a, b.min(*value - 1)),
                        '>' => (a.max(*value + 1), b),
                        _ => unreachable!("{}", op),
                    };
                    let not_matching_range = match op {
                        '<' => (*value, b),
                        '>' => (a, *value),
                        _ => unreachable!("{}", op),
                    };
                    if matching_range.0 <= matching_range.1 {
                        let mut copy = ranges;
                        copy.set(*name, matching_range);
                        result += count_solutions(workflows, &copy, next);
                    }
                    if not_matching_range.0 <= not_matching_range.1 {
                        ranges.set(*name, not_matching_range);
                    } else {
                        break;
                    }
                }
                Rule::Default { next } => {
                    result += count_solutions(workflows, &ranges, next);
                }
            }
        }
        result
    }
}

fn state_from_start<'a>(workflows: &'a HashMap<&str, Workflow>, part: &Part) -> &'a str {
    let mut name = "in";
    while let Some(w) = workflows.get(name) {
        let next = w
            .rules
            .iter()
            .find(|rule| match rule {
                Rule::Compare {
                    name,
                    op,
                    value,
                    next: _,
                } => match *op {
                    '<' => {
                        let part_value = part.get(*name);
                        part_value < *value
                    }
                    '>' => {
                        let part_value = part.get(*name);
                        part_value > *value
                    }
                    _ => unreachable!("{}", op),
                },
                Rule::Default { .. } => true,
            })
            .map(|rule| match rule {
                Rule::Compare { next, .. } => next,
                Rule::Default { next } => next,
            })
            .unwrap();
        name = next;
    }
    name
}

#[derive(Debug)]
struct State<'a> {
    workflows: HashMap<&'a str, Workflow<'a>>,
    parts: Vec<Part>,
}

#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

#[derive(Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn get(&self, key: char) -> i64 {
        match key {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => unreachable!("{}", key),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Ranges {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

impl Ranges {
    fn num_choices(&self) -> i64 {
        fn range_choices(r: (i64, i64)) -> i64 {
            r.1 - r.0 + 1
        }

        range_choices(self.x)
            * range_choices(self.m)
            * range_choices(self.a)
            * range_choices(self.s)
    }

    fn get(&self, key: char) -> (i64, i64) {
        match key {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => unreachable!("{}", key),
        }
    }

    fn set(&mut self, key: char, value: (i64, i64)) {
        match key {
            'x' => self.x = value,
            'm' => self.m = value,
            'a' => self.a = value,
            's' => self.s = value,
            _ => unreachable!("{}", key),
        }
    }
}

#[derive(Debug)]
enum Rule<'a> {
    Compare {
        name: char,
        op: char,
        value: i64,
        next: &'a str,
    },
    Default {
        next: &'a str,
    },
}

fn parse_state(input: &str) -> IResult<&str, State> {
    let (input, workflows) = separated_list1(line_ending, parse_workflow)(input)?;
    let (input, _) = many1(line_ending)(input)?;
    let (input, parts) = separated_list1(line_ending, parse_part)(input)?;
    let workflows: HashMap<&str, Workflow> =
        workflows.into_iter().fold(HashMap::new(), |mut m, wf| {
            m.insert(wf.name, wf);
            m
        });
    Ok((input, State { workflows, parts }))
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    // {x=787,m=2655,a=1222,s=2876}
    let (input, _) = complete::char('{')(input)?;
    let (input, x) = preceded(tag("x="), complete::i64)(input)?;
    let (input, _) = complete::char(',')(input)?;
    let (input, m) = preceded(tag("m="), complete::i64)(input)?;
    let (input, _) = complete::char(',')(input)?;
    let (input, a) = preceded(tag("a="), complete::i64)(input)?;
    let (input, _) = complete::char(',')(input)?;
    let (input, s) = preceded(tag("s="), complete::i64)(input)?;
    let (input, _) = complete::char('}')(input)?;
    Ok((input, Part { x, m, a, s }))
}
fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = alpha1(input)?;
    let (input, rules) = delimited(
        complete::char('{'),
        separated_list1(complete::char(','), parse_rule),
        complete::char('}'),
    )(input)?;
    Ok((input, Workflow { name, rules }))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    alt((parse_compare_rule, parse_default_rule))(input)
}

fn parse_compare_rule(input: &str) -> IResult<&str, Rule> {
    let (input, name) = one_of("xmas")(input)?;
    let (input, op) = one_of("<>")(input)?;
    let (input, value) = complete::i64(input)?;
    let (input, _) = character::complete::char(':')(input)?;
    let (input, next) = alpha1(input)?;

    Ok((
        input,
        Rule::Compare {
            name,
            op,
            value,
            next,
        },
    ))
}

fn parse_default_rule(input: &str) -> IResult<&str, Rule> {
    let (input, next) = alpha1(input)?;
    Ok((input, Rule::Default { next }))
}
#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 19114;
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
