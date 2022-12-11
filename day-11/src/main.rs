use std::collections::VecDeque;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, one_of, space0, space1};
use nom::combinator::{map, opt};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let input = read_file("./day-11/input.txt")?;
    let result = part1(&input);

    println!("{}", result);

    let result = part2(&input);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> usize {
    let (_, mut monkeys) = monkeys(input).unwrap();
    let mut inspections = vec![0; monkeys.len()];
    let num_rounds = 20;
    let num_monkeys = monkeys.len();

    for _ in 0..num_rounds {
        for monkey_no in 0..num_monkeys {
            while !monkeys[monkey_no].items.is_empty() {
                inspections[monkey_no] += 1;
                let old_value = monkeys[monkey_no].items.pop_front().unwrap();
                let mut new_value = monkeys[monkey_no].operation.value(old_value);
                new_value /= 3;
                let test_value = monkeys[monkey_no].test_operand;
                let next_monkey = if new_value % test_value == 0 {
                    monkeys[monkey_no].if_true
                } else {
                    monkeys[monkey_no].if_false
                };

                monkeys[next_monkey].items.push_back(new_value);
            }
        }
    }

    inspections.sort_by(|a, b| b.cmp(a));

    inspections[0] * inspections[1]
}

fn part2(input: &str) -> i32 {
    let (_, mut monkeys) = monkeys(input).unwrap();
    let mut inspections = vec![0; monkeys.len()];
    let num_rounds = 10_000;
    let num_monkeys = monkeys.len();

    for _ in 0..num_rounds {
        for monkey_no in 0..num_monkeys {
            while !monkeys[monkey_no].items.is_empty() {
                inspections[monkey_no] += 1;
                let old_value = monkeys[monkey_no].items.pop_front().unwrap();
                let new_value = monkeys[monkey_no].operation.value(old_value);
                let test_value = monkeys[monkey_no].test_operand;
                let next_monkey = if new_value % test_value == 0 {
                    monkeys[monkey_no].if_true
                } else {
                    monkeys[monkey_no].if_false
                };

                monkeys[next_monkey].items.push_back(new_value);
            }
        }
    }

    inspections.sort_by(|a, b| b.cmp(a));

    let a = inspections[0] as i32;
    let b = inspections[1] as i32;
    a * b
}

fn read_file(filename: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i32>,
    operation: Operation,
    test_operand: i32,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug)]
enum Operand {
    Old,
    Constant(i32),
}

impl Operand {
    fn value(&self, old: i32) -> i32 {
        match self {
            Operand::Old => old,
            Operand::Constant(n) => *n,
        }
    }
}

#[derive(Debug)]
struct Operation {
    op: Op,
    left: Operand,
    right: Operand,
}

impl Operation {
    fn value(&self, old: i32) -> i32 {
        let left_value = self.left.value(old);
        let right_value = self.right.value(old);
        match self.op {
            Op::Add => left_value + right_value,
            Op::Mul => left_value * right_value,
        }
    }
}

fn monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(many1(line_ending), monkey)(input)
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = monkey_name(input)?;
    let (input, _) = line_ending(input)?;
    let (input, items) = items(input)?;
    let (input, _) = line_ending(input)?;
    let (input, operation) = operation(input)?;
    let (input, _) = line_ending(input)?;
    let (input, test_operand) = test_operand(input)?;
    let (input, _) = line_ending(input)?;
    let (input, if_true) = if_true(input)?;
    let (input, _) = line_ending(input)?;
    let (input, if_false) = if_false(input)?;
    let (input, _) = opt(line_ending)(input)?;

    let result = Monkey {
        items: VecDeque::from(items),
        operation,
        test_operand,
        if_true,
        if_false,
    };

    Ok((input, result))
}

fn monkey_name(input: &str) -> IResult<&str, u32> {
    use nom::character::complete::u32 as u32_parser;
    delimited(tag("Monkey "), u32_parser, tag(":"))(input)
}

fn items(input: &str) -> IResult<&str, Vec<i32>> {
    use nom::character::complete::i32 as i32_parser;
    preceded(
        delimited(space1, tag("Starting items:"), space1),
        separated_list1(tuple((tag(","), space1)), i32_parser),
    )(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, (left, op, right)) = preceded(
        delimited(space1, tag("Operation: new ="), space1),
        tuple((operand, op, operand)),
    )(input)?;

    let result = Operation { left, op, right };

    Ok((input, result))
}

fn op(input: &str) -> IResult<&str, Op> {
    let (input, c) = delimited(space0, one_of("+*"), space0)(input)?;
    let result = match c {
        '+' => Op::Add,
        _ => Op::Mul,
    };

    Ok((input, result))
}

fn operand(input: &str) -> IResult<&str, Operand> {
    let old = map(tag("old"), |_: &str| Operand::Old);
    let constant = map(nom::character::complete::i32, |n| Operand::Constant(n));
    delimited(space0, alt((old, constant)), space0)(input)
}

fn test_operand(input: &str) -> IResult<&str, i32> {
    use nom::character::complete::i32 as i32_parser;

    preceded(
        delimited(space1, tag("Test: divisible by"), space1),
        i32_parser,
    )(input)
}

fn if_true(input: &str) -> IResult<&str, usize> {
    use nom::character::complete::u32 as u32_parser;

    map(
        preceded(
            delimited(space1, tag("If true: throw to monkey"), space1),
            u32_parser,
        ),
        |n| n as usize,
    )(input)
}

fn if_false(input: &str) -> IResult<&str, usize> {
    use nom::character::complete::u32 as u32_parser;

    map(
        preceded(
            delimited(space1, tag("If false: throw to monkey"), space1),
            u32_parser,
        ),
        |n| n as usize,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 10605;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        // let result = part2(INPUT);
        // let expected = 2713310158i32i32;
        // assert_eq!(result, expected);
    }
}
