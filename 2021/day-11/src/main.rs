extern crate core;

use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

use anyhow::Context;
use itertools::Itertools;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let filename = "./day-11/input.txt";
    let content = read_file(filename).context(filename)?;

    let result = part1(&content);
    println!("{}", result);

    let result = part2(&content);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> i32 {
    let (_, mut grid) = lines(input).unwrap();
    let num_rounds = 100;
    let mut result = 0;
    for _round in 1..=num_rounds {
        increase_energy_levels(&mut grid);
        result += flash_octopuses(&mut grid);
    }
    result
}

fn part2(input: &str) -> i32 {
    let (_, mut grid) = lines(input).unwrap();
    let mut result = 0;
    loop {
        result += 1;
        increase_energy_levels(&mut grid);
        flash_octopuses(&mut grid);
        if all_octopuses_flashed(&grid) {
            break;
        }
    }
    result
}

const DELTAS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn all_octopuses_flashed(grid: &Grid) -> bool {
    let width = grid[0].len();
    let height = grid.len();

    for row in 0..height {
        for col in 0..width {
            if grid[row][col] != 0 {
                return false;
            }
        }
    }

    true
}

fn flash_octopuses(grid: &mut Grid) -> i32 {
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    let iter = (0..height)
        .cartesian_product(0..width)
        .filter(|(row, col)| grid[*row as usize][*col as usize] > 9)
        .map(|(row, col)| (row as i32, col as i32));

    let mut q = Vec::from_iter(iter.clone());
    let mut visited: HashSet<(i32, i32)> = HashSet::from_iter(iter);
    let mut result = 0;
    while let Some((row, col)) = q.pop() {
        result += 1;

        let next_coords = DELTAS
            .into_iter()
            .map(|(dr, dc)| (row + dr, col + dc))
            .filter(|(r, c)| *r >= 0 && *c >= 0 && *r < height && *c < width);

        for (r, c) in next_coords.clone() {
            grid[r as usize][c as usize] += 1;
        }

        let next_coords = next_coords.filter(|(r, c)| grid[*r as usize][*c as usize] > 9);

        for (r, c) in next_coords {
            if visited.insert((r, c)) {
                q.push((r, c));
            }
        }
    }

    for (row, col) in visited.into_iter() {
        grid[row as usize][col as usize] = 0;
    }

    result
}

fn increase_energy_levels(grid: &mut Grid) {
    let width = grid[0].len();
    let height = grid.len();

    for row in 0..height {
        for col in 0..width {
            grid[row][col] += 1;
        }
    }
}

type Grid = Vec<Vec<i32>>;

fn lines(input: &str) -> IResult<&str, Grid> {
    separated_list1(line_ending, line)(input)
}

fn line(input: &str) -> IResult<&str, Vec<i32>> {
    map(digit1, |s: &str| {
        s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()
    })(input)
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

    const INPUT: &str = include_str!("../test.txt");

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 1656;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 195;
        assert_eq!(result, expected);
    }
}
