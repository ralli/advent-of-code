use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let filename = "day-14.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut grid: Grid = input.parse()?;
    grid.move_dishes_north();
    let result = grid.total_load();
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut grid: Grid = input.parse()?;
    let num_steps = 1_000_000_000;
    let mut visited: HashMap<Grid, Vec<i32>> = HashMap::new();
    for cycle in 1..=num_steps {
        grid.move_dishes_north();
        grid.move_dishes_west();
        grid.move_dishes_south();
        grid.move_dishes_east();
        let key = grid.clone();
        let e = visited.entry(key).or_default();
        e.push(cycle);
        if e.len() >= 2 {
            let delta = e[1] - e[0];
            if (num_steps - e[0]) % delta == 0 {
                break;
            }
        }
    }
    let result = grid.total_load();
    Ok(result)
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Grid {
    cells: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn move_dishes_north(&mut self) {
        for row in 1..self.height {
            for col in 0..self.width {
                if self.cells[row][col] == 'O' {
                    let mut r = row;
                    while r > 0 && self.cells[r - 1][col] == '.' {
                        self.cells[r - 1][col] = 'O';
                        self.cells[r][col] = '.';
                        r -= 1;
                    }
                }
            }
        }
    }

    fn move_dishes_south(&mut self) {
        for row in (0..self.height - 1).rev() {
            for col in 0..self.width {
                if self.cells[row][col] == 'O' {
                    let mut r = row;
                    while r + 1 < self.height && self.cells[r + 1][col] == '.' {
                        self.cells[r + 1][col] = 'O';
                        self.cells[r][col] = '.';
                        r += 1;
                    }
                }
            }
        }
    }

    fn move_dishes_west(&mut self) {
        for col in 1..self.width {
            for row in 0..self.height {
                if self.cells[row][col] == 'O' {
                    let mut c = col;
                    while c > 0 && self.cells[row][c - 1] == '.' {
                        self.cells[row][c - 1] = 'O';
                        self.cells[row][c] = '.';
                        c -= 1;
                    }
                }
            }
        }
    }

    fn move_dishes_east(&mut self) {
        for col in (0..self.width - 1).rev() {
            for row in 0..self.height {
                if self.cells[row][col] == 'O' {
                    let mut c = col;
                    while c + 1 < self.width && self.cells[row][c + 1] == '.' {
                        self.cells[row][c + 1] = 'O';
                        self.cells[row][c] = '.';
                        c += 1;
                    }
                }
            }
        }
    }

    fn total_load(&self) -> usize {
        let mut result = 0;

        for row in 0..self.height {
            for col in 0..self.width {
                if self.cells[row][col] == 'O' {
                    result += self.height - row;
                }
            }
        }
        result
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let cells: Vec<Vec<char>> = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        let height = cells.len();
        let width = cells.first().map(|c| c.len()).unwrap_or_default();
        Ok(Grid {
            cells,
            width,
            height,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 136;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 64;
        assert_eq!(result, expected);
        Ok(())
    }
}
