use anyhow::Context;
use std::path::Path;
use std::{fmt, fs};

fn main() -> anyhow::Result<()> {
    let content = read_file("day-4/day-4.txt")?;

    let result = part1(&content);
    println!("{result}");

    let result = part2(&content);
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> usize {
    let grid = Grid::new(input);
    let mut result = 0;

    for row in 0..grid.height {
        for col in 0..grid.width {
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if grid.get(row, col) == 'X'
                        && grid.get(row + dr, col + dc) == 'M'
                        && grid.get(row + 2 * dr, col + 2 * dc) == 'A'
                        && grid.get(row + 3 * dr, col + 3 * dc) == 'S'
                    {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

fn part2(input: &str) -> usize {
    let grid = Grid::new(input);
    let mut result = 0;

    for row in 1..grid.height - 1 {
        for col in 1..grid.width - 1 {
            if grid.get(row, col) == 'A'
                && ((grid.get(row - 1, col - 1) == 'M' && grid.get(row + 1, col + 1) == 'S')
                    || (grid.get(row - 1, col - 1) == 'S' && grid.get(row + 1, col + 1) == 'M'))
                && ((grid.get(row + 1, col - 1) == 'M' && grid.get(row - 1, col + 1) == 'S')
                    || (grid.get(row + 1, col - 1) == 'S' && grid.get(row - 1, col + 1) == 'M'))
            {
                result += 1;
            }
        }
    }
    result
}

struct Grid {
    width: isize,
    height: isize,
    data: Vec<Vec<char>>,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        Self {
            height: data.len() as isize,
            width: data.first().map(|r| r.len()).unwrap_or_default() as isize,
            data,
        }
    }
    fn get(&self, row: isize, col: isize) -> char {
        if row < 0 || row >= self.height || col < 0 || col >= self.width {
            return ' ';
        }
        self.data[row as usize][col as usize]
    }
}

fn read_file(filename: impl AsRef<Path> + fmt::Display) -> anyhow::Result<String> {
    fs::read_to_string(filename.as_ref()).context(format!("cannot load file {}", filename))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let result = part1(input);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part2() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let result = part2(input);
        assert_eq!(result, 9);
    }
}
