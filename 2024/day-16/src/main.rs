use anyhow::{anyhow, Context};
use nom::character::complete::{line_ending, one_of};
use nom::multi::{many1, separated_list0};
use nom::IResult;
use pathfinding::prelude::dijkstra;
use std::fmt::Formatter;
use std::{fmt, fs};

fn main() -> anyhow::Result<()> {
    let filename = "day-16/input.txt";
    let content = fs::read_to_string(filename).context(format!("cannot load {filename}"))?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn as_vector(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let (_, grid) = parse_grid(input).map_err(|e| anyhow!("{e}"))?;
    // println!("{grid}");
    let (sr, sc) = grid.start_pos;
    let (er, ec) = grid.end_pos;
    let result_attempt = dijkstra(
        &(sr, sc, Direction::East),
        |p| successors(&grid, p),
        |(r, c, _)| *r == er && *c == ec,
    );
    // println!("{result_attempt:?}");
    let Some((_path, cost)) = result_attempt else {
        return Err(anyhow!("no path found"));
    };
    Ok(cost)
}

fn part2(_input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

fn turn_cost(dir1: &Direction, dir2: &Direction) -> Option<usize> {
    if dir1 == dir2 {
        return Some(0);
    }
    let (dx1, dy1) = dir1.as_vector();
    let (dx2, dy2) = dir2.as_vector();
    if dx1 == -dx2 && dy1 == -dy2 {
        return None;
    }
    Some(1000)
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

fn successors(grid: &Grid, pos: &PosDir) -> Vec<(PosDir, usize)> {
    let mut result: Vec<(PosDir, usize)> = Vec::with_capacity(3);
    let (row, col, dir) = *pos;
    for next_dir in DIRECTIONS.iter() {
        if let Some(tc) = turn_cost(&dir, next_dir) {
            let (dr, dc) = next_dir.as_vector();
            let (next_row, next_col) = ((row as i32 + dr) as usize, (col as i32 + dc) as usize);
            if grid.cells[next_row][next_col] != '#' {
                let next_p = ((next_row, next_col, *next_dir), tc + 1);
                // println!("{pos:?} -> {next_p:?} {}", grid.cells[next_row][next_col]);
                result.push(next_p)
            }
        }
    }
    result
}

type Position = (usize, usize);
type PosDir = (usize, usize, Direction);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<char>>,
    start_pos: Position,
    end_pos: Position,
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

fn parse_grid(input: &str) -> IResult<&str, Grid> {
    let (rest, cells) = separated_list0(line_ending, parse_grid_line)(input)?;
    let width = cells.first().map(|r| r.len()).unwrap_or_default();
    let height = cells.len();
    let mut start_row = 0;
    let mut start_col = 0;
    let mut end_row = 0;
    let mut end_col = 0;
    for (row_idx, row) in cells.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if *col == 'S' {
                (start_row, start_col) = (row_idx, col_idx);
            }
            if *col == 'E' {
                (end_row, end_col) = (row_idx, col_idx);
            }
        }
    }
    Ok((
        rest,
        Grid {
            width,
            height,
            cells,
            start_pos: (start_row, start_col),
            end_pos: (end_row, end_col),
        },
    ))
}

fn parse_grid_line(input: &str) -> IResult<&str, Vec<char>> {
    many1(one_of(".#SE"))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        assert_eq!(result, 7036);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        // let result = part2(INPUT)?;
        // assert_eq!(result, 9021);
        Ok(())
    }
}
