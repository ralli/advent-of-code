use std::fs;
use std::str::FromStr;
use anyhow::Context;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let filename = "day-11.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let grid: Grid = input.parse()?;
    let expansion_factor = 1;
    let result = grid.sum_of_distances(expansion_factor);
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let grid: Grid = input.parse()?;
    let expansion_factor = 1_000_000 - 1; // each empty row/col is **replaced** by 1_000_000 rows / cols. That's a factor of 999_999
    let result = grid.sum_of_distances(expansion_factor);
    Ok(result)
}

#[derive(Debug)]
struct Grid {
    positions: Vec<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl Grid {
    fn sum_of_distances(&self, expansion_factor: usize) -> usize {
        let size = self.positions.len();
        let mut result = 0;
        for i in 0..size {
            for j in (i + 1)..size {
                let d = self.distance(&self.positions[i], &self.positions[j], expansion_factor);
                result += d;
            }
        }
        result
    }

    fn distance(&self, p1: &(usize, usize), p2: &(usize, usize), expansion_factor: usize) -> usize {
        let (row1, col1) = *p1;
        let (row2, col2) = *p2;

        // translate each point by the number of relevant empty rows/cols
        let row1 = row1 + self.empty_rows.iter().filter(|&&r| r < row1).count() * expansion_factor;
        let row2 = row2 + self.empty_rows.iter().filter(|&&r| r < row2).count() * expansion_factor;
        let col1 = col1 + self.empty_cols.iter().filter(|&&c| c < col1).count() * expansion_factor;
        let col2 = col2 + self.empty_cols.iter().filter(|&&c| c < col2).count() * expansion_factor;

        let dx = (col2 as isize - col1 as isize).unsigned_abs();
        let dy = (row2 as isize - row1 as isize).unsigned_abs();

        dx + dy
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let pixels: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let height = pixels.len();
        let width = pixels.first().map(|line| line.len()).unwrap_or_default();
        let empty_rows: Vec<usize> = (0..height).filter(|&row| (0..width).all(|col| pixels[row][col] == '.')).collect();
        let empty_cols: Vec<usize> = (0..width).filter(|&col| (0..height).all(|row| pixels[row][col] == '.')).collect();
        let positions = (0..height).cartesian_product(0..width).filter(|&(row, col)| pixels[row][col] == '#').collect();
        Ok(Self {
            positions,
            empty_rows,
            empty_cols,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 374;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn sum_of_distances_with_an_expansion() -> anyhow::Result<()> {
        let grid: Grid = INPUT.parse()?;
        let result = grid.sum_of_distances(10 - 1);
        let expected = 1030;
        assert_eq!(result, expected);
        Ok(())
    }
}