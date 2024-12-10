use anyhow::Context;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;

fn main() -> anyhow::Result<()> {
    let filename = "day-10/input.txt";
    let content = fs::read_to_string(filename).context(format!("cannot load {filename}"))?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> Result<usize, anyhow::Error> {
    let grid = parse_grid(input);
    let result = walk_grid(&grid);
    Ok(result)
}

fn part2(input: &str) -> Result<usize, anyhow::Error> {
    let grid = parse_grid(input);
    let result = walk_grid2(&grid);
    Ok(result)
}

#[derive(Debug)]
struct Grid {
    width: isize,
    height: isize,
    grid: Vec<Vec<i32>>,
}

impl Grid {
    fn get(&self, row: isize, col: isize) -> i32 {
        if row < 0 || row >= self.height || col < 0 || col >= self.width {
            return -1;
        }
        self.grid[row as usize][col as usize]
    }
}

const DIRS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn walk_grid(grid: &Grid) -> usize {
    let mut result = 0;
    for row in 0..grid.height {
        for col in 0..grid.width {
            if grid.get(row, col) == 0 {
                let count = walk_grid_from_start(grid, row, col);
                result += count;
            }
        }
    }
    result
}

fn walk_grid2(grid: &Grid) -> usize {
    let mut result = 0;
    for row in 0..grid.height {
        for col in 0..grid.width {
            if grid.get(row, col) == 0 {
                let count = walk_grid_from_start2(grid, row, col);
                result += count;
            }
        }
    }
    result
}

fn walk_grid_from_start(grid: &Grid, start_row: isize, start_col: isize) -> usize {
    let mut q = VecDeque::from([(start_row, start_col, 0)]);
    let mut visited: BTreeSet<(isize, isize)> = BTreeSet::new();

    while let Some((row, col, height)) = q.pop_front() {
        if height == 9 {
            visited.insert((row, col));
        }
        for (dr, dc) in DIRS.iter() {
            let (next_row, next_col) = (row + dr, col + dc);
            if grid.get(next_row, next_col) == height + 1 {
                q.push_back((next_row, next_col, height + 1));
            }
        }
    }
    visited.len()
}

fn walk_grid_from_start2(grid: &Grid, start_row: isize, start_col: isize) -> usize {
    let mut q = VecDeque::from([(start_row, start_col, 0)]);
    let mut counts: BTreeMap<(isize, isize), usize> = BTreeMap::new();

    while let Some((row, col, height)) = q.pop_front() {
        if height == 9 {
            let entry = counts.entry((row, col)).or_insert(0);
            *entry += 1;
        }
        for (dr, dc) in DIRS.iter() {
            let (next_row, next_col) = (row + dr, col + dc);
            if grid.get(next_row, next_col) == height + 1 {
                q.push_back((next_row, next_col, height + 1));
            }
        }
    }
    counts.values().sum()
}

fn parse_grid(input: &str) -> Grid {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_digit(10) || *c == '.')
                .map(|c| {
                    if c == '.' {
                        -1
                    } else {
                        c.to_digit(10).unwrap() as i32
                    }
                })
                .collect()
        })
        .collect();
    let width = grid.iter().next().map(|r| r.len()).unwrap_or_default() as isize;
    let height = grid.len() as isize;
    Grid {
        width,
        height,
        grid,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT2: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    #[test]
    fn test_part1() {
        let result = part1(INPUT2).unwrap();
        assert_eq!(result, 36);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT2).unwrap();
        assert_eq!(result, 81);
    }
}
