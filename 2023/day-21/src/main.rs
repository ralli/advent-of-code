use std::collections::BTreeSet;
use std::fmt::Formatter;
use std::str::FromStr;
use std::{fmt, fs};

use anyhow::{anyhow, Context};

fn main() -> anyhow::Result<()> {
    let filename = "day-21.txt";
    let input =
        fs::read_to_string(filename).with_context(|| format!("cannot open file {filename}"))?;

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i64> {
    let grid: Grid = input.parse()?;
    grid.number_of_garden_plots(64)
}

fn part2(input: &str) -> anyhow::Result<i64> {
    let grid: Grid = input.parse()?;
    grid.number_of_garden_plots2(26501365)
}

struct Grid {
    cells: Vec<Vec<char>>,
    width: isize,
    height: isize,
}

impl Grid {
    fn number_of_garden_plots(&self, max_steps: i64) -> anyhow::Result<i64> {
        let (start_row, start_col) = self
            .start_pos()
            .ok_or_else(|| anyhow!("no starting position"))?;
        static DELTAS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut points = BTreeSet::from([(start_row, start_col)]);

        for round in 0..max_steps {
            let mut visited = BTreeSet::new();
            for &(row, col) in points.iter() {
                for &(dr, dc) in DELTAS.iter() {
                    let (next_row, next_col) = (row + dr, col + dc);
                    if next_row >= 0
                        && next_row < self.height
                        && next_col >= 0
                        && next_col <= self.width
                        && self.get(next_row, next_col) != '#'
                    {
                        visited.insert((next_row, next_col));
                    }
                }
            }
            points = visited;
        }

        Ok(points.len() as i64)
    }

    fn number_of_garden_plots2(&self, max_steps: i64) -> anyhow::Result<i64> {
        let (start_row, start_col) = self
            .start_pos()
            .ok_or_else(|| anyhow!("no starting position"))?;
        static DELTAS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut points = BTreeSet::from([(start_row, start_col)]);

        for round in 0..max_steps {
            let mut visited = BTreeSet::new();
            for &(row, col) in points.iter() {
                for &(dr, dc) in DELTAS.iter() {
                    let (next_row, next_col) = (row + dr, col + dc);
                    if self.get(next_row, next_col) != '#' {
                        visited.insert((next_row, next_col));
                    }
                }
            }
            points = visited;
            if round % 10 == 0 {
                println!("{} {}", round, points.len());
            }
        }

        Ok(points.len() as i64)
    }

    fn start_pos(&self) -> Option<(isize, isize)> {
        self.cells.iter().enumerate().find_map(|(row, line)| {
            line.iter()
                .enumerate()
                .find(|(_, c)| **c == 'S')
                .map(|(col, _)| (row as isize, col as isize))
        })
    }

    fn get(&self, row: isize, col: isize) -> char {
        let r = row.rem_euclid(self.height);
        let c = col.rem_euclid(self.width);
        self.cells[r as usize][c as usize]
    }

    fn set(&mut self, row: isize, col: isize, value: char) {
        let r = row.rem_euclid(self.height);
        let c = col.rem_euclid(self.width);
        self.cells[r as usize][c as usize] = value;
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells: Vec<Vec<char>> = s
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect();
        let height = cells.len() as isize;
        let width = cells.first().map(|line| line.len()).unwrap_or_default() as isize;
        Ok(Grid {
            cells,
            width,
            height,
        })
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in self.cells.iter() {
            for c in line.iter() {
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

    #[test]
    fn test1() -> anyhow::Result<()> {
        let grid: Grid = INPUT.parse()?;
        println!("{grid}");
        let result = grid.number_of_garden_plots(6)?;
        let expected = 16;
        assert_eq!(result, expected);
        Ok(())
    }
}
