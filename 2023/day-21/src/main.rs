use std::fs;

use anyhow::Context;

use day_21::{part1, part2};

fn main() -> anyhow::Result<()> {
    let filename = "day-21.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;
    let input = input.trim();

    let result = part1(input)?;
    println!("{result}");

    let result = part2(input)?;
    println!("{result}");

    Ok(())
}

