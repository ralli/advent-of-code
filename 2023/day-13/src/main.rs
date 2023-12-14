use std::fmt::Formatter;
use std::{fmt, fs};

use anyhow::{anyhow, Context};
use nom::branch::alt;
use nom::character::complete::{char, line_ending};
use nom::multi::{many1, separated_list0, separated_list1};
use nom::IResult;
use nom::Parser;

fn main() -> anyhow::Result<()> {
    let filename = "day-13.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let grids = parse_input(input)?;
    let folds: Vec<_> = grids.iter().map(|g| g.detect_fold()).collect();
    let result = folds
        .iter()
        .map(|fold| match fold {
            Fold::Vertical(n) => *n,
            Fold::Horizontal(n) => 100 * *n,
        })
        .sum();
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let grids = parse_input(input)?;
    let folds: Vec<_> = grids.iter().map(|g| g.detect_smudge_fold()).collect();
    let result = folds
        .iter()
        .map(|fold| match fold {
            Fold::Vertical(n) => *n,
            Fold::Horizontal(n) => 100 * *n,
        })
        .sum();
    Ok(result)
}

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<Vec<CellType>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn detect_fold(&self) -> Fold {
        self.rotated()
            .find_horizontal_fold()
            .map(Fold::Vertical)
            .or_else(|| self.find_horizontal_fold().map(Fold::Horizontal))
            .unwrap()
    }

    fn detect_fold_opt(&self) -> Vec<Fold> {
        self.rotated()
            .find_horizontal_folds()
            .into_iter()
            .map(Fold::Vertical)
            .chain(
                self.find_horizontal_folds()
                    .into_iter()
                    .map(Fold::Horizontal),
            )
            .collect()
    }

    fn detect_smudge_fold(&self) -> Fold {
        let mut g = self.clone();
        let old = self.detect_fold();
        for row in 0..self.height {
            for col in 0..self.width {
                g.flip(row, col);
                let maybes: Vec<Fold> = g
                    .detect_fold_opt()
                    .into_iter()
                    .filter(|n| *n != old)
                    .collect();
                g.flip(row, col);
                if !maybes.is_empty() {
                    return maybes[0];
                }
            }
        }
        unreachable!();
    }

    fn flip(&mut self, row: usize, col: usize) {
        self.cells[row][col] = self.cells[row][col].flipped();
    }

    fn find_horizontal_fold(&self) -> Option<usize> {
        (0..self.height - 1).find_map(|row| self.find_horizontal_fold_at_row(row))
    }

    fn find_horizontal_folds(&self) -> Vec<usize> {
        (0..self.height - 1)
            .filter_map(|row| self.find_horizontal_fold_at_row(row))
            .collect()
    }

    fn find_horizontal_fold_at_row(&self, row: usize) -> Option<usize> {
        (0..=row)
            .rev()
            .zip((row + 1)..self.height)
            .all(|(i, j)| self.cells[i] == self.cells[j])
            .then_some(row + 1)
    }

    fn rotated(&self) -> Grid {
        let cells: Vec<Vec<CellType>> = (0..self.width)
            .map(|col| {
                (0..self.height)
                    .map(|row| self.cells[row][col])
                    .collect::<Vec<_>>()
            })
            .collect();
        Grid {
            cells,
            height: self.width,
            width: self.height,
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for r in 0..self.height {
            for c in 0..self.width {
                let c = match self.cells[r][c] {
                    CellType::Ash => '.',
                    CellType::Rock => '#',
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum CellType {
    Ash,
    Rock,
}

impl CellType {
    fn flipped(&self) -> Self {
        match self {
            CellType::Ash => CellType::Rock,
            CellType::Rock => CellType::Ash,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Grid>> {
    let (_, grids) = parse_grids(input).map_err(|e| anyhow!(e.to_string()))?;
    Ok(grids)
}

fn parse_grids(input: &str) -> IResult<&str, Vec<Grid>> {
    separated_list0(many1(line_ending), parse_grid)(input)
}

fn parse_grid(input: &str) -> IResult<&str, Grid> {
    let row = many1(parse_cell_type);
    let (input, cells) = separated_list1(line_ending, row)(input)?;
    let height = cells.len();
    let width = cells.first().map(|r| r.len()).unwrap_or_default();
    Ok((
        input,
        Grid {
            cells,
            height,
            width,
        },
    ))
}

fn parse_cell_type(input: &str) -> IResult<&str, CellType> {
    let ash = char('.').map(|_| CellType::Ash);
    let rock = char('#').map(|_| CellType::Rock);
    alt((ash, rock))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    #[test]
    fn test1() -> anyhow::Result<()> {
        let grids = parse_input(INPUT)?;
        println!("{}", grids[0].rotated());
        let result = grids[0].detect_fold();

        let expected = Fold::Vertical(5);
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test2() -> anyhow::Result<()> {
        let grids = parse_input(INPUT)?;
        let result = grids[0].detect_smudge_fold();

        let expected = Fold::Horizontal(3);
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 405;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 400;
        assert_eq!(result, expected);
        Ok(())
    }
}
