use std::collections::BTreeMap;

use anyhow::anyhow;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, line_ending, one_of};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded};

pub fn part1(input: &str) -> anyhow::Result<i64> {
    let state = parse_input(input)?;
    let result: i64 = state.parts
        .iter()
        .filter(|part| state_for_part(&state.workflows, part).as_str() == "A")
        .map(|part| part.ratiing_sum())
        .sum();
    Ok(result)
}

pub fn part2(input: &str) -> anyhow::Result<i64> {
    let state = parse_input(input)?;
    let start = "in".to_string();
    let mut ranges = all_combinations(&state.workflows, &start);
    ranges.sort();
    for r in ranges.iter() {
        println!("{} {:?}", r.number_of_combinations(), r);
    }
    let result = ranges.iter().map(|r| r.number_of_combinations()).sum();
    Ok(result)
}

fn state_for_part(workflows: &BTreeMap<String, Workflow>, part: &Part) -> String {
    let mut start = "in".to_string();

    while let Some(workflow) = workflows.get(&start) {
        start = workflow.rules.iter().filter_map(|r| r.matches(part)).next().unwrap();
    }

    start
}

fn all_combinations(workflows: &BTreeMap<String, Workflow>, name: &String) -> Vec<Ranges> {
    let result = if name.as_str() == "A" {
        vec![Ranges {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }]
    } else if name.as_str() == "R" {
        Vec::new()
    } else {
        let workflow = workflows.get(name).unwrap();

        workflow.rules.iter().flat_map(|rule| {
            let n = rule.name();
            match rule {
                Rule::Basic { next, .. } => {
                    all_combinations(workflows, next).iter().map(|range| rule.apply(range)).filter(|r| !r.is_empty()).collect::<Vec<Ranges>>()
                }
                Rule::Default { next } => {
                    all_combinations(workflows, next).iter().map(|ranges| ranges.apply_all_rules_inverted(&workflow.rules)).filter(|r| !r.is_empty()).collect::<Vec<Ranges>>()
                }
            }
        }).collect()
    };
    // println!("{}: {:?}", name, result);
    result
}


#[derive(Debug)]
struct State {
    workflows: BTreeMap<String, Workflow>,
    parts: Vec<Part>,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug)]
enum Rule {
    Basic {
        name: char,
        op: Op,
        value: i64,
        next: String,
    },
    Default {
        next: String,
    },
}

impl Rule {
    fn apply(&self, ranges: &Ranges) -> Ranges {
        match self {
            Rule::Default { .. } => *ranges,
            Rule::Basic { name, op, value, next } => {
                match op {
                    Op::Less => {
                        let mut ans = *ranges;
                        let (a, b) = ans.range_of(*name);
                        ans.set_range_of(*name, (a, b.min(*value - 1)));
                        ans
                    }
                    Op::Greater => {
                        let mut ans = *ranges;
                        let (a, b) = ans.range_of(*name);
                        ans.set_range_of(*name, (a.max(*value + 1), b));
                        ans
                    }
                }
            }
        }
    }

    fn name(&self) -> String {
        match self {
            Rule::Basic { next, .. } => next.to_string(),
            Rule::Default { next } => next.to_string()
        }
    }

    fn matches(&self, part: &Part) -> Option<String> {
        match self {
            Rule::Basic { name, op, value, next } => {
                let part_value = part.value_of(*name);
                match op {
                    Op::Less => if part_value < *value {
                        Some(next.to_string())
                    } else {
                        None
                    }
                    Op::Greater => if part_value > *value {
                        Some(next.to_string())
                    } else {
                        None
                    }
                }
            }
            Rule::Default { next } => { Some(next.to_string()) }
        }
    }
}

#[derive(Debug)]
enum Op {
    Less,
    Greater,
}

#[derive(Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Part {
    fn value_of(&self, name: char) -> i64 {
        match name {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => unreachable!("{}", name)
        }
    }

    fn ratiing_sum(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}


#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Ranges {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

impl Ranges {
    fn apply_all_rules_inverted(&self, rules: &[Rule]) -> Ranges {
        let mut ans = *self;
        for rule in rules.iter() {
            match rule {
                Rule::Default { .. } => {}
                Rule::Basic { name, op, value, next: _ } => {
                    match op {
                        Op::Less => {
                            let (a, b) = self.range_of(*name);
                            ans.set_range_of(*name, (*value, b));
                        }
                        Op::Greater => {
                            // >x => (a, b) = (a, x)
                            let (a, b) = self.range_of(*name);
                            ans.set_range_of(*name, (a, *value));
                        }
                    }
                }
            }
        }
        ans
    }
}

fn is_range_empty(r: (i64, i64)) -> bool {
    r.0 > r.1
}

fn range_combinations(r: (i64, i64)) -> i64 {
    let (a, b) = r;
    b - a + 1
}

impl Ranges {
    fn number_of_combinations(&self) -> i64 {
        range_combinations(self.x)
            * range_combinations(self.m)
            * range_combinations(self.a)
            * range_combinations(self.s)
    }

    fn is_empty(&self) -> bool {
        is_range_empty(self.x) || is_range_empty(self.m) || is_range_empty(self.a) || is_range_empty(self.s)
    }

    fn range_of(&self, name: char) -> (i64, i64) {
        match name {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => unreachable!("{}", name)
        }
    }

    fn set_range_of(&mut self, name: char, range: (i64, i64)) {
        match name {
            'x' => self.x = range,
            'm' => self.m = range,
            'a' => self.a = range,
            's' => self.s = range,
            _ => unreachable!("{}", name)
        }
    }
}


fn parse_input(input: &str) -> anyhow::Result<State> {
    let (_, state) = parse_state(input).map_err(|e| anyhow!(e.to_string()))?;
    Ok(state)
}

fn parse_state(input: &str) -> IResult<&str, State> {
    let (input, workflows) = separated_list1(line_ending, parse_workflow)(input)?;
    let (input, _) = many1(line_ending)(input)?;
    let (input, parts) = separated_list1(line_ending, parse_part)(input)?;
    let workflows: BTreeMap<String, Workflow> = workflows.into_iter().fold(BTreeMap::new(), |mut m, w| {
        m.insert(w.name.to_string(), w);
        m
    });
    Ok((input, State { workflows, parts }))
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, _) = tag("{")(input)?;
    let (input, x) = preceded(tag("x="), complete::i64)(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, m) = preceded(tag("m="), complete::i64)(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, a) = preceded(tag("a="), complete::i64)(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, s) = preceded(tag("s="), complete::i64)(input)?;
    let (input, _) = tag("}")(input)?;
    Ok((input, Part { x, m, a, s }))
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = alpha1(input)?;
    let (input, rules) = delimited(tag("{"), separated_list1(tag(","), parse_rule), tag("}"))(input)?;
    Ok((input, Workflow { name: name.to_string(), rules }))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    alt((parse_basic_rule, parse_default_rule))(input)
}

fn parse_basic_rule(input: &str) -> IResult<&str, Rule> {
    let (input, name) = one_of("xmas")(input)?;
    let (input, op) = parse_op(input)?;
    let (input, value) = complete::i64(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, next) = alpha1(input)?;
    Ok((input, Rule::Basic { name: name, op, value, next: next.to_string() }))
}

fn parse_default_rule(input: &str) -> IResult<&str, Rule> {
    let (input, next) = alpha1(input)?;
    Ok((input, Rule::Default { next: next.to_string() }))
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    let (input, op) = one_of("<>")(input)?;
    let op = match op {
        '<' => Op::Less,
        '>' => Op::Greater,
        _ => unreachable!(),
    };
    Ok((input, op))
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

    #[test]
    fn test1() {
        let ranges = Ranges {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        };
        let rules = vec![Rule::Basic { name: 'm', op: Op::Greater, value: 1548, next: "A".to_string() }, Rule::Default { next: "A".to_string() }];
        let ans = ranges.apply_all_rules_inverted(&rules);
        println!("{:?}", ans);
    }

    #[test]
    fn test2() {
        let ranges = Ranges {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        };
        let result = ranges.number_of_combinations();
        let expected = 4000 * 4000 * 4000 * 4000;
        assert_eq!(result, expected);
    }
}