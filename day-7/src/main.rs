use std::fs::File;
use std::io::Read;

use anyhow::Context;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::multi::separated_list1;

fn main() -> anyhow::Result<()> {
    let filename = "./day-7/input.txt";
    let content = read_file(filename).context(filename)?;

    let result = part1(&content);
    println!("{}", result);

    let result = part2(&content);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> i32 {
    let (_, positions) = positions(input).unwrap();
    let min_position = positions.iter().min().copied().unwrap();
    let max_position = positions.iter().max().copied().unwrap();
    (min_position..=max_position).map(|p| fuel_consumtion(&positions, p)).min().unwrap()
}

fn fuel_consumtion(positions: &[i32], position: i32) -> i32 {
    positions.iter().map(|&p| (p - position).abs()).sum()
}

fn part2(input: &str) -> i32 {
    let (_, positions) = positions(input).unwrap();
    let min_position = positions.iter().min().copied().unwrap();
    let max_position = positions.iter().max().copied().unwrap();
    (min_position..=max_position).map(|p| crab_fuel_consumtion(&positions, p)).min().unwrap()
}

fn crab_fuel_consumtion(positions: &[i32], position: i32) -> i32 {
    positions.iter().map(|&p| {
        let n = (p - position).abs();
        n * (n + 1) / 2
    }).sum()
}

fn positions(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(tag(","), nom::character::complete::i32)(input)
}

fn read_file(name: &str) -> anyhow::Result<String> {
    let mut f = File::open(name)?;
    let mut result = String::new();
    f.read_to_string(&mut result)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 37;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 168;
        assert_eq!(result, expected);
    }
}
