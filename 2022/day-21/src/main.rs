use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, line_ending, space0};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let input = read_file("./day-21/input.txt")?;

    let result = part1(&input);
    println!("{}", result);

    let result = part2(&input);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> i64 {
    let (_, monkeys) = monkeys(input).unwrap();
    let monkey_map: HashMap<&str, &Monkey> = monkeys.iter().map(|m| (m.name, m)).collect();
    find_solution("root", &monkey_map).unwrap()
}

fn part2(input: &str) -> i64 {
    let (_, monkeys) = monkeys(input).unwrap();
    let monkey_map: HashMap<&str, &Monkey> = monkeys.iter().map(|m| (m.name, m)).collect();
    let root = monkey_map.get("root").unwrap();
    let result = if let Job::Calculation(_, left, right) = root.job {
        let left_contains_key = contains_key(left, "humn", &monkey_map);
        if left_contains_key {
            let right_value = find_solution(right, &monkey_map).unwrap();
            find_goal(left, right_value, &monkey_map)
        } else {
            let left_value = find_solution(left, &monkey_map).unwrap();
            find_goal(right, left_value, &monkey_map)
        }
    } else {
        None
    };
    result.unwrap()
}

fn find_goal(name: &str, goal: i64, monkey_map: &HashMap<&str, &Monkey>) -> Option<i64> {
    if name == "humn" {
        return Some(goal);
    }
    let result = monkey_map.get(name).and_then(|monkey| match &monkey.job {
        Job::Number(_) => Some(goal),
        Job::Calculation(op, left, right) => {
            let left_contains_key = contains_key(left, "humn", monkey_map);
            if left_contains_key {
                find_solution(right, monkey_map).and_then(|right_value| {
                    let next_goal = match op {
                        // l + r = g => l = g - r
                        Op::Add => goal - right_value,
                        // l - r = g => l = g + r
                        Op::Sub => goal + right_value,
                        // l * r = g => l = g / r
                        Op::Mul => goal / right_value,
                        // l / r = g => l = g * r
                        Op::Div => goal * right_value,
                    };
                    find_goal(left, next_goal, monkey_map)
                })
            } else {
                find_solution(left, monkey_map).and_then(|left_value| {
                    let next_goal = match op {
                        // l + r = g => r = g - l
                        Op::Add => goal - left_value,
                        // l - r = g => l - r - g = 0 =>  r = l - g
                        Op::Sub => left_value - goal,
                        // l * r = g => r = g / l
                        Op::Mul => goal / left_value,
                        // l / r = g => l = g * r => l / g = r
                        Op::Div => left_value / goal,
                    };
                    find_goal(right, next_goal, monkey_map)
                })
            }
        }
    });

    result
}

fn find_solution(name: &str, monkey_map: &HashMap<&str, &Monkey>) -> Option<i64> {
    monkey_map.get(name).and_then(|monkey| match &monkey.job {
        Job::Number(n) => Some(*n),
        Job::Calculation(op, left_name, right_name) => find_solution(left_name, monkey_map)
            .and_then(|left_value| {
                find_solution(right_name, monkey_map)
                    .map(|right_value| op.evaluate(left_value, right_value))
            }),
    })
}

fn contains_key(current: &str, name: &str, monkey_map: &HashMap<&str, &Monkey>) -> bool {
    if current == name {
        return true;
    }
    monkey_map
        .get(current)
        .map(|monkey| match monkey.job {
            Job::Number(_) => false,
            Job::Calculation(_, left, right) => {
                contains_key(left, name, monkey_map) || contains_key(right, name, monkey_map)
            }
        })
        .unwrap_or(false)
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn evaluate(&self, left_value: i64, right_value: i64) -> i64 {
        match self {
            Op::Add => left_value + right_value,
            Op::Sub => left_value - right_value,
            Op::Mul => left_value * right_value,
            Op::Div => left_value / right_value,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Job<'a> {
    Number(i64),
    Calculation(Op, &'a str, &'a str),
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Monkey<'a> {
    name: &'a str,
    job: Job<'a>,
}

fn monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(line_ending, monkey)(input)
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, job) = job(input)?;

    Ok((input, Monkey { name, job }))
}

fn job(input: &str) -> IResult<&str, Job> {
    alt((map(complete::i64, Job::Number), calculation))(input)
}

fn calculation(input: &str) -> IResult<&str, Job> {
    let (input, lhs) = alpha1(input)?;
    let (input, op) = delimited(space0, op, space0)(input)?;
    let (input, rhs) = alpha1(input)?;
    Ok((input, Job::Calculation(op, lhs, rhs)))
}

fn op(input: &str) -> IResult<&str, Op> {
    alt((
        map(tag("+"), |_| Op::Add),
        map(tag("-"), |_| Op::Sub),
        map(tag("*"), |_| Op::Mul),
        map(tag("/"), |_| Op::Div),
    ))(input)
}

fn read_file(filename: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 152;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 301;
        assert_eq!(result, expected);
    }
}
