use std::fmt;
use std::fmt::Formatter;
use winnow::ModalResult;
use winnow::Parser;
use winnow::ascii::{line_ending, multispace0};
use winnow::combinator::{eof, repeat, separated, terminated};
use winnow::token::one_of;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day-04.txt")?;
    let result = part1(&input)?;
    println!("{result}");
    let result = part2(&input)?;
    println!("{result}");
    Ok(())
}

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get(&self, row: isize, col: isize) -> char {
        if row < 0 || row >= self.height as isize || col < 0 || col >= self.width as isize {
            '.'
        } else {
            self.cells[row as usize][col as usize]
        }
    }

    fn set(&mut self, row: isize, col: isize, value: char) {
        if row < 0 || row >= self.height as isize || col < 0 || col >= self.width as isize {
            return;
        }
        self.cells[row as usize][col as usize] = value;
    }

    fn count_adjacent(&self, row: isize, col: isize) -> usize {
        let mut count = 0;
        for r in row - 1..=row + 1 {
            for c in col - 1..=col + 1 {
                if r == row && c == col {
                    continue;
                }
                let c = self.get(r, c);
                if c == '@' || c == 'x' {
                    count += 1;
                }
            }
        }
        count
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                write!(f, "{}", self.cells[row][col])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1(input: &str) -> anyhow::Result<u64> {
    let mut inp = input;
    let grid = terminated(parse_grid, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    let mut count = 0;
    for row in 0..grid.height as isize {
        for col in 0..grid.width as isize {
            if grid.get(row, col) == '@' && grid.count_adjacent(row, col) < 4 {
                count += 1;
            }
        }
    }
    Ok(count)
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let mut inp = input;
    let mut grid = terminated(parse_grid, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    let mut count = 0;
    //println!("{}", grid);
    loop {
        let mut found = 0;
        for row in 0..grid.height as isize {
            for col in 0..grid.width as isize {
                if grid.get(row, col) == '@' && grid.count_adjacent(row, col) < 4 {
                    count += 1;
                    found += 1;
                    grid.set(row, col, 'x');
                }
            }
        }
        if found == 0 {
            break;
        }
        //println!("found: {found}");
        // println!("{}", grid);
        for row in 0..grid.height as isize {
            for col in 0..grid.width as isize {
                if grid.get(row, col) == 'x' {
                    grid.set(row, col, '.');
                }
            }
        }
    }
    Ok(count)
}

fn parse_grid(input: &mut &str) -> ModalResult<Grid> {
    let cells: Vec<_> = separated(1.., parse_row, line_ending).parse_next(input)?;
    let width = cells.len();
    let height: usize = cells.first().map(|row| row.len()).unwrap_or(0);
    Ok(Grid {
        cells,
        width,
        height,
    })
}

fn parse_row(input: &mut &str) -> ModalResult<Vec<char>> {
    repeat(1.., parse_cell).parse_next(input)
}

fn parse_cell(input: &mut &str) -> ModalResult<char> {
    one_of(['.', '@']).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 43);
    }
}
