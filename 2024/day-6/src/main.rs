use day_6::{count_guard_positions, count_obstructions, parse_grid};
use std::fs;

fn main() -> anyhow::Result<()> {
    let content = fs::read_to_string("input.txt")?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let grid = parse_grid(input)?;
    Ok(count_guard_positions(&grid))
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let grid = parse_grid(input)?;
    Ok(count_obstructions(&grid))
}
