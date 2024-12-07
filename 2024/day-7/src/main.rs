use anyhow::{anyhow, Context};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, multispace0, space1};
use nom::multi::separated_list0;
use nom::sequence::{terminated, tuple};
use nom::IResult;
use rayon::prelude::*;
use std::fs;

fn main() -> anyhow::Result<()> {
    let filename = "day-7/input.txt";
    let content = fs::read_to_string(filename).context(format!("cannot load {filename}"))?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i64> {
    let (_, equations) = parse_input(input).map_err(|e| anyhow!("{e}"))?;

    let result = equations
        .par_iter()
        .filter(|equation| has_solutions(equation.goal, 0, &equation.values))
        .map(|equation| equation.goal)
        .sum();

    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<i64> {
    let (_, equations) = parse_input(input).map_err(|e| anyhow!("{e}"))?;

    let result = equations
        .par_iter()
        .filter(|equation| has_solutions2(equation.goal, 0, &equation.values))
        .map(|equation| equation.goal)
        .sum();

    Ok(result)
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Equation {
    goal: i64,
    values: Vec<i64>,
}

fn has_solutions(goal: i64, current: i64, values: &[i64]) -> bool {
    if values.is_empty() {
        return goal == current;
    }

    if current > goal {
        return false;
    }

    let first_value = *values.first().unwrap();

    crate::has_solutions(goal, current + first_value, &values[1..])
        || crate::has_solutions(goal, current * first_value, &values[1..])
}

fn has_solutions2(goal: i64, current: i64, values: &[i64]) -> bool {
    if values.is_empty() {
        return goal == current;
    }

    if current > goal {
        return false;
    }

    let first_value = *values.first().unwrap();

    has_solutions2(goal, current + first_value, &values[1..])
        || has_solutions2(goal, current * first_value, &values[1..])
        || has_solutions2(goal, concatenate(current, first_value), &values[1..])
}

fn concatenate(x: i64, y: i64) -> i64 {
    let mut tmp = y;
    let mut x = x;
    while tmp > 0 {
        x *= 10;
        tmp /= 10;
    }
    x + y
}

fn parse_input(input: &str) -> IResult<&str, Vec<Equation>> {
    terminated(parse_equation_list, multispace0)(input)
}

fn parse_equation_list(input: &str) -> IResult<&str, Vec<Equation>> {
    separated_list0(line_ending, parse_equation)(input)
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    let (rest, goal) = complete::i64(input)?;
    let (rest, _) = tuple((tag(":"), space1))(rest)?;
    let (rest, values) = separated_list0(space1, complete::i64)(rest)?;
    Ok((rest, Equation { goal, values }))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        assert_eq!(result, 3749);
        Ok(())
    }

    #[test]
    fn test_concatenate() {
        assert_eq!(concatenate(15, 6), 156);
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        assert_eq!(result, 11387);
        Ok(())
    }
}
