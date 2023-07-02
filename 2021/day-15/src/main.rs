use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};
use std::fs::File;
use std::io::Read;

use anyhow::Context;
use nom::character::complete::{digit1, line_ending, multispace0};
use nom::combinator::{eof, map};
use nom::multi::separated_list1;
use nom::sequence::{terminated, tuple};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let filename = "./day-15/input.txt";
    let content = read_file(filename).context(filename)?;

    let result = part1(&content);
    println!("{}", result);

    let result = part2(&content);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> u32 {
    let (_, grid) = terminated(grid, tuple((multispace0, eof)))(input).unwrap();
    find_lowest_total_risk_part1(&grid).unwrap()
}

fn part2(input: &str) -> u32 {
    let (_, grid) = terminated(grid, tuple((multispace0, eof)))(input).unwrap();
    find_lowest_total_risk_part2(&grid).unwrap()
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Entry {
    cost: u32,
    row: usize,
    col: usize,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_lowest_total_risk_part1(grid: &Grid) -> Option<u32> {
    let mut q = BinaryHeap::from([Entry {
        row: 0,
        col: 0,
        cost: 0,
    }]);
    let width = grid.cells[0].len();
    let height = grid.cells.len();
    let mut dists = BTreeMap::new();

    while let Some(Entry { row, col, cost }) = q.pop() {
        let mut adj = Vec::new();

        if row == width - 1 && col == height - 1 {
            return Some(cost);
        }

        if row > 0 {
            adj.push((row - 1, col));
        }

        if row + 1 < height {
            adj.push((row + 1, col));
        }

        if col > 0 {
            adj.push((row, col - 1));
        }

        if col + 1 < width {
            adj.push((row, col + 1));
        }

        for (next_row, next_col) in adj {
            let next_cost = grid.cells[next_row][next_col] + cost;

            if let Some(&test_cost) = dists.get(&(next_row, next_col)) {
                if next_cost < test_cost {
                    q.push(Entry {
                        row: next_row,
                        col: next_col,
                        cost: next_cost,
                    });
                    dists.insert((next_row, next_col), next_cost);
                }
            } else {
                q.push(Entry {
                    row: next_row,
                    col: next_col,
                    cost: next_cost,
                });
                dists.insert((next_row, next_col), next_cost);
            }
        }
    }

    None
}

fn find_lowest_total_risk_part2(grid: &Grid) -> Option<u32> {
    let mut q = BinaryHeap::from([Entry {
        row: 0,
        col: 0,
        cost: 0,
    }]);
    let width = grid.width();
    let height = grid.height();
    let mut dists = BTreeMap::new();

    while let Some(Entry { cost, row, col }) = q.pop() {
        let mut adj = Vec::new();

        if row == height - 1 && col == width - 1 {
            return Some(cost);
        }
        if row > 0 {
            adj.push((row - 1, col));
        }
        if row + 1 < height {
            adj.push((row + 1, col));
        }
        if col > 0 {
            adj.push((row, col - 1));
        }
        if col + 1 < width {
            adj.push((row, col + 1));
        }
        for (next_row, next_col) in adj {
            let next_cost = grid.get(row, col) + cost;
            let entry = dists.entry((next_row, next_col)).or_insert(u32::MAX);
            if next_cost < *entry {
                *entry = next_cost;
                q.push(Entry {
                    cost: next_cost,
                    row: next_row,
                    col: next_col,
                });
            }
        }
    }

    None
}

#[derive(Debug, Copy, Clone)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<u32>>,
}

impl Grid {
    fn get(&self, row: usize, col: usize) -> u32 {
        let width = self.cells[0].len();
        let height = self.cells.len();
        let grid_col = col / width;
        let grid_row = row / height;

        let value = self.cells[row % height][col % width];

        (value + grid_col as u32 + grid_row as u32 - 1) % 9 + 1
    }

    fn width(&self) -> usize {
        5 * self.cells[0].len()
    }

    fn height(&self) -> usize {
        5 * self.cells.len()
    }
}

fn grid(input: &str) -> IResult<&str, Grid> {
    let (input, lines) = separated_list1(
        line_ending,
        map(digit1, |s: &str| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        }),
    )(input)?;

    Ok((input, Grid { cells: lines }))
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
        let expected = 40;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 307;
        assert_eq!(result, expected);
    }
}
