use std::{fmt, fs};
use std::fmt::Formatter;

use anyhow::{anyhow, Context};
use nom::branch::alt;
use nom::character::complete::{char, line_ending};
use nom::IResult;
use nom::multi::{many1, separated_list0, separated_list1};
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
    let mut col_reflections: Vec<usize> = Vec::new();
    let mut row_reflections: Vec<usize> = Vec::new();
    for g in grids.iter() {
        if let Some(c) = g.col_reflection() {
            col_reflections.push(c)
        } else {
            let bla = g.row_reflection();
            if let Some(bla) = bla {
                row_reflections.push(bla);
            }
        }
    }
    // assert_eq!(col_reflections.len() + row_reflections.len(), grids.len());
    println!("{col_reflections:?} {row_reflections:?}");
    let ncol: usize = col_reflections.iter().sum();
    let nrow: usize = row_reflections.iter().sum::<usize>() * 100;
    // 7705 too low
    Ok(ncol + nrow)
}

fn part2(input: &str) -> anyhow::Result<i64> {
    Ok(0)
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<CellType>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn col_reflection(&self) -> Option<usize> {
        if self.width < 2 {
            return None;
        }
        (0..(self.width - 2)).find(|&col|
            self.reflects_at_col(col)
        ).map(|i| i + 1)
    }

    fn row_reflection(&self) -> Option<usize> {
        if self.height == 0 {
            return None;
        }
        (0..(self.height - 2)).find(|&row| {
            let result = self.reflects_at_row(row);
            println!("{row} {result}");
            result
        }
        ).map(|i| i + 1)
    }

    fn reflects_at_col(&self, start_col: usize) -> bool {
        (0..self.height).all(|row| self.reflects_row_at_col(row, start_col))
    }

    fn reflects_row_at_col(&self, row: usize, start_col: usize) -> bool {
        let mut i1 = start_col;
        let mut i2 = start_col + 1;
        while i2 < self.width {
            if self.cells[row][i1] != self.cells[row][i2] {
                return false;
            }
            if i1 == 0 {
                break;
            }
            i1 -= 1;
            i2 += 1;
        }
        true
    }

    fn reflects_at_row(&self, start_row: usize) -> bool {
        (0..self.width).all(|col| dbg!(self.reflects_col_at_row(col, start_row)))
    }

    fn reflects_col_at_row(&self, col: usize, start_row: usize) -> bool {
        let mut i1 = start_row;
        let mut i2 = start_row + 1;
        while i2 < self.height {
            if self.cells[i1][col] != self.cells[i2][col] {
                return false;
            }
            if i1 == 0 {
                break;
            }
            i1 -= 1;
            i2 += 1;
        }
        true
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
    Ok((input, Grid { cells, height, width }))
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
        let result = grids[0].col_reflection();
        let expected = Some(1);
        assert_eq!(result, expected);
        let result = grids[0].row_reflection();
        let expected = None;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test2() -> anyhow::Result<()> {
        let grids = parse_input(INPUT)?;
        let result = grids[1].col_reflection();
        let expected = None;
        assert_eq!(result, expected);
        let result = grids[1].row_reflection();
        let expected = Some(1);
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
        let expected = 525152;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_10() -> anyhow::Result<()> {
        let input = r#"#.##.######.##.##
###...####...####
....##.##.##.....
#..#.#....#.#..##
.....##..##......
#.#.###..###.#.##
.##.#.####.#.#...
.#..#......#..#..
.####.####.####..
###...####...####
#.##........##.##
.#....#..#....#..
..###.####.###...
"#;
        let (_, grid) = parse_grid(input).map_err(|e| anyhow!(e.to_string()))?;
        dbg!(grid.reflects_col_at_row(7,1));
      //  dbg!(grid.reflects_at_row(0));
        // let bla = grid.row_reflection();
        // let expected = Some(8);
        // assert_eq!(bla, expected);
        Ok(())
    }
}
