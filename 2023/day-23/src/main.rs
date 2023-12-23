use std::collections::VecDeque;
use std::fmt::Formatter;
use std::str::FromStr;
use std::{fmt, fs, iter};

use anyhow::{anyhow, Context};

fn main() -> anyhow::Result<()> {
    let filename = "day-23.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;
    let result = part1(&input)?;
    println!("{result}");
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i64> {
    let grid: Grid = input.parse()?;
    let start = grid.find_start().ok_or_else(|| anyhow!("no start"))?;
    let mut q = VecDeque::from([(start, Vec::new())]);

    let mut result = 0;
    while let Some(((row, col), path)) = q.pop_front() {
        if row + 1 == grid.height {
            result = result.max(path.len());
        }
        let directions = grid.next_directions(row, col);
        for &(dr, dc) in directions.iter() {
            let (next_row, next_col) = (row + dr, col + dc);
            if next_row >= 0
                && next_row < grid.height
                && next_col >= 0
                && next_col < grid.width
                && grid.cells[next_row as usize][next_col as usize] != '#'
                && !path.contains(&(next_row, next_col))
            {
                let next_path = path
                    .iter()
                    .copied()
                    .chain(iter::once((next_row, next_col)))
                    .collect();
                q.push_back(((next_row, next_col), next_path));
            }
        }
        //
    }
    Ok(result as i64)
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<char>>,
    width: isize,
    height: isize,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        let height = cells.len() as isize;
        let width = cells
            .first()
            .filter(|line| !line.is_empty())
            .map(|line| line.len())
            .unwrap_or_default() as isize;
        Ok(Grid {
            cells,
            width,
            height,
        })
    }
}

impl Grid {
    fn find_start(&self) -> Option<(isize, isize)> {
        self.cells.iter().enumerate().find_map(|(row, line)| {
            line.iter()
                .enumerate()
                .find(|(_, ch)| **ch != '#')
                .map(|(col, _)| (row as isize, col as isize))
        })
    }

    fn next_directions(&self, row: isize, col: isize) -> &'static [(isize, isize)] {
        match self.cells[row as usize][col as usize] {
            '^' => [(-1, 0)].as_slice(),
            'v' => [(1, 0)].as_slice(),
            '<' => [(0, -1)].as_slice(),
            '>' => [(0, 1)].as_slice(),
            _ => [(-1, 0), (1, 0), (0, -1), (0, 1)].as_slice(),
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in self.cells.iter() {
            for ch in line.iter() {
                write!(f, "{ch}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 94;
        assert_eq!(result, expected);
        Ok(())
    }
}
