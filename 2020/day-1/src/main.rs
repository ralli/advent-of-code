use anyhow::anyhow;
use nom::character::complete;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::{character::complete::line_ending, IResult};
use std::{collections::HashSet, fs};

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let (a, b) = part1(&input)?;
    let result = a * b;
    println!("part1: {a} {b} {result}");
    let (a, b, c) = part2(&input)?;
    let result = a * b * c;
    println!("part2: {a} {b} {c} {result}");
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<(i32, i32)> {
    let (_, numbers) = parse_input(input).unwrap();
    let result = find_solution1(&numbers, 2020).ok_or_else(|| anyhow!("no solution found"))?;
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<(i32, i32, i32)> {
    let (_, numbers) = parse_input(input).unwrap();
    let result = find_solutions2(&numbers, 2020).ok_or_else(|| anyhow!("no solution found"))?;
    Ok(result)
}

fn find_solutions2(numbers: &HashSet<i32>, goal: i32) -> Option<(i32, i32, i32)> {
    let nums: Vec<i32> = Vec::from_iter(numbers.iter().copied());
    let size = nums.len();
    for i in 0..size {
        let a = nums[i];
        for j in (i + 1)..size {
            let b = nums[j];
            if a + b < goal && numbers.contains(&(goal - a - b)) {
                return Some((a, b, goal - a - b));
            }
        }
    }
    None
}

fn find_solution1(numbers: &HashSet<i32>, goal: i32) -> Option<(i32, i32)> {
    numbers
        .iter()
        .filter(|n| {
            let second = goal - *n;
            numbers.contains(&second)
        })
        .map(|n| (*n, goal - *n))
        .next()
}

fn parse_input(input: &str) -> IResult<&str, HashSet<i32>> {
    map(separated_list1(line_ending, complete::i32), |numbers| {
        HashSet::from_iter(numbers.into_iter())
    })(input)
}
