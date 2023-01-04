extern crate core;

use std::collections::{BTreeSet, VecDeque};
use std::fs::File;
use std::io::Read;

use anyhow::Context;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;

fn main() -> anyhow::Result<()> {
    let filename = "./day-9/input.txt";
    let content = read_file(filename).context(filename)?;

    let result = part1(&content);
    println!("{}", result);

    let result = part2(&content);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> i32 {
    let (_, grid) = grid(input).unwrap();
    let result = low_points(&grid);
    result.into_iter().map(|(_, _, v)| v + 1).sum()
}


fn part2(input: &str) -> usize {
    let (_, grid) = grid(input).unwrap();
    let points = low_points(&grid);
    let basins: BTreeSet<_> = points.into_iter().map(|(row, col, _)| basin_size(&grid, row, col)).collect();
    let mut sizes: Vec<usize> = basins.into_iter().map(|basin| basin.len()).collect();
    sizes.sort_by(|a, b| b.cmp(a));
    sizes.iter().take(3).product()
}

fn basin_size(grid: &Grid, row: usize, col: usize) -> BTreeSet<(usize, usize)> {
    let value = grid[row][col];
    let width = grid[0].len();
    let height = grid.len();
    let mut q = VecDeque::from([(row, col, value)]);
    let mut visited = BTreeSet::from([(row, col)]);


    while let Some((row, col, _)) = q.pop_front() {
        if row > 0 && grid[row - 1][col] != 9 && visited.insert((row - 1, col)) {
            q.push_back((row - 1, col, grid[row - 1][col]));
        }

        if row + 1 < height && grid[row + 1][col] != 9 && visited.insert((row + 1, col)) {
            q.push_back((row + 1, col, grid[row + 1][col]));
        }

        if col > 0 && grid[row][col - 1] != 9 && visited.insert((row, col - 1)) {
            q.push_back((row, col - 1, grid[row][col - 1]));
        }

        if col + 1 < width && grid[row][col + 1] != 9 && visited.insert((row, col + 1)) {
            q.push_back((row, col + 1, grid[row][col + 1]));
        }
    }

    visited
}

fn low_points(grid: &Grid) -> Vec<(usize, usize, i32)> {
    let width = grid[0].len();
    let height = grid.len();
    let mut result = Vec::new();
    for row in 0..height {
        for col in 0..width {
            let value = grid[row][col];

            if row > 0 && value >= grid[row - 1][col] {
                continue;
            }

            if row + 1 < height && value >= grid[row + 1][col] {
                continue;
            }

            if col > 0 && value >= grid[row][col - 1] {
                continue;
            }

            if col + 1 < width && value >= grid[row][col + 1] {
                continue;
            }

            result.push((row, col, value));
        }
    }
    result
}

type Grid = Vec<Vec<i32>>;

fn grid(input: &str) -> IResult<&str, Grid> {
    separated_list1(line_ending, line)(input)
}

fn line(input: &str) -> IResult<&str, Vec<i32>> {
    map(digit1, |s: &str| s.chars().map(|c| (c as i32 - '0' as i32)).collect())(input)
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
        let expected = 15;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 1134;
        assert_eq!(result, expected);
    }
}
