use std::{fmt, fs};
use std::fmt::Formatter;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, newline};
use nom::combinator::{eof, map};
use nom::IResult;
use nom::multi::{many1, separated_list1};

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let result = part1(&input)?;
    println!("{result}");
    let result = part2(&input)?;
    println!("{result}");
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<u64> {
    let (_, grid) = parse_grid(input).map_err(|e| anyhow::Error::msg(e.to_string()))?;
    Ok(treecount_for_slope(&grid, 3, 1))
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let (_, grid) = parse_grid(input).map_err(|e| anyhow::Error::msg(e.to_string()))?;
    let count = slopes.into_iter().map(|(dx, dy)| treecount_for_slope(&grid, dx, dy)).product();
    Ok(count)
}


fn treecount_for_slope(grid: &Grid, dx: usize, dy: usize) -> u64 {
    let width = grid.width();
    let height = grid.height();
    if width < dx || height < dy {
        return 0;
    }
    let mut r = 0;
    let mut c = 0;
    let mut count = 0;
    while r < height - dy {
        r += dy;
        c += dx;
        c %= width;
        if let Value::Tree = grid.lines[r][c] {
            count += 1;
        }
    }
    count
}

#[derive(Debug)]
struct Grid {
    lines: Vec<Vec<Value>>,
}

impl Grid {
    fn width(&self) -> usize {
        self.lines.first().unwrap().len()
    }

    fn height(&self) -> usize {
        self.lines.len()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for r in self.lines.iter() {
            for v in r.iter() {
                write!(f, "{v}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Empty,
    Tree,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Value::Empty => '.',
            Value::Tree => '#'
        };
        write!(f, "{ch}")
    }
}

fn parse_grid(input: &str) -> IResult<&str, Grid> {
    let (input, lines) = parse_lines(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = eof(input)?;
    Ok((input, Grid { lines }))
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<Value>>> {
    separated_list1(newline, parse_values)(input)
}

fn parse_values(input: &str) -> IResult<&str, Vec<Value>> {
    many1(parse_value)(input)
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    let empty = map(tag("."), |_| Value::Empty);
    let tree = map(tag("#"), |_| Value::Tree);
    alt((empty, tree))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        assert_eq!(result, 7);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        assert_eq!(result, 336);
        Ok(())
    }

}