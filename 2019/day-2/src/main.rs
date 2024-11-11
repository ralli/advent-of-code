use anyhow::{anyhow, Context};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list0;
use nom::IResult;
use std::fs;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let filename = "day-2/day-2.txt";
    let input = read_file(filename).with_context(|| format!("cannot load {filename}"))?;

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i64> {
    let (_, values) = parse_input(input).map_err(|e| anyhow!(e.to_string()))?;
    Ok(run_program(&values, 12, 2))
}

fn part2(input: &str) -> anyhow::Result<i64> {
    let (_, values) = parse_input(input).map_err(|e| anyhow!(e.to_string()))?;
    for noun in 0..=99 {
        for verb in 0..=99 {
            if run_program(&values, noun, verb) == 19690720 {
                return Ok(100 * noun + verb);
            }
        }
    }
    Err(anyhow!("no solution"))
}

fn run_program(values: &[i64], noun: i64, verb: i64) -> i64 {
    let mut values = values.to_vec();
    values[1] = noun;
    values[2] = verb;
    let size = values.len();
    for i in (0..size).step_by(4) {
        let arr = &values[i..i + 4];
        assert_eq!(arr.len(), 4);
        let opcode = arr[0];
        if opcode == 99 {
            break;
        }
        let a_idx = arr[1] as usize;
        let a = values[a_idx];
        let b_idx = arr[2] as usize;
        let b = values[b_idx];
        let c = arr[3] as usize;
        match opcode {
            1 => {
                values[c] = a + b;
            }
            2 => {
                values[c] = a * b;
            }
            _ => unreachable!()
        }
    }
    values[0]
}

fn parse_input(input: &str) -> IResult<&str, Vec<i64>> {
    let (rest, values): (&str, Vec<i64>) = separated_list0(tag(","), complete::i64)(input)?;
    Ok((rest, values))
}

fn read_file(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let s = fs::read_to_string(&path)?;
    Ok(s)
}
