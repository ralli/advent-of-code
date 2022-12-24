use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, line_ending, space0};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::IResult;
use std::collections::{HashMap, HashSet};

fn main() -> anyhow::Result<()> {
    let input = read_file("./day-21/input.txt")?;

    let result = part1(&input);
    println!("{}", result);

    // let result = part2(&input);
    // println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> i64 {
    let (_, monkeys) = monkeys(input).unwrap();
    let empty = HashMap::new();
    let initial_values: Vec<_> = monkeys
        .iter()
        .filter_map(|monkey| {
            monkey
                .job
                .evaluate(&empty)
                .map(|result| (monkey.name, result))
        })
        .collect();
    let mut context: HashMap<&str, i64> = HashMap::from_iter(initial_values.iter().copied());

    loop {
        let remaining: HashSet<_> = monkeys
            .iter()
            .filter(|n| !context.contains_key(n.name))
            .filter_map(|m| m.job.evaluate(&context).map(|v| (m.name, v)))
            .collect();

        for (name, value) in remaining {
            if name == "root" {
                return value;
            }
            context.insert(name, value);
        }
        // println!("{:?}", context);
    }
}

fn part2(_input: &str) -> i64 {
    todo!()
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Job<'a> {
    Number(i64),
    Calculation(Op, &'a str, &'a str),
}

impl<'a> Job<'a> {
    fn evaluate(&self, context: &HashMap<&str, i64>) -> Option<i64> {
        use Job::*;

        match self {
            Number(n) => Some(*n),
            Calculation(op, lhs, rhs) => context.get(lhs).and_then(|left_value| {
                context.get(rhs).map(|right_value| match op {
                    Op::Add => *left_value + *right_value,
                    Op::Sub => *left_value - *right_value,
                    Op::Mul => *left_value * *right_value,
                    Op::Div => *left_value / *right_value,
                })
            }),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Monkey<'a> {
    name: &'a str,
    job: Job<'a>,
}

impl<'a> Monkey<'a> {}

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
        let expected = 1623178306i64;
        assert_eq!(result, expected);
    }
}
