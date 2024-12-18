use anyhow::{anyhow, Context};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, multispace0};
use nom::combinator::{eof, map};
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::{HashSet, VecDeque};
use std::fmt::Formatter;
use std::str::FromStr;
use std::{fmt, fs};

fn main() -> anyhow::Result<()> {
    let filename = "day-18/input.txt";
    let content = fs::read_to_string(filename).context(format!("cannot load {filename}"))?;

    let result = part1(&content, 1024)?;
    println!("{result}");

    let (x, y) = part2(&content, 1024)?;
    println!("{x},{y}");

    Ok(())
}

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn part1(input: &str, size: usize) -> anyhow::Result<usize> {
    let grid: Grid = input.parse()?;
    bfs_with_len(&grid, size).ok_or_else(|| anyhow!("no solution found"))
}

fn part2(input: &str, min_size: usize) -> anyhow::Result<Point> {
    let grid: Grid = input.parse()?;
    let max_size = grid.points.iter().len();
    for i in min_size..max_size {
        if bfs_with_len(&grid, i).is_none() {
            return Ok(grid.points[i - 1]);
        }
    }
    Err(anyhow!("no solution found"))
}

fn bfs_with_len(grid: &Grid, size: usize) -> Option<usize> {
    let grid = grid.truncate(size);
    // println!("{}", grid);
    let points: HashSet<Point> = HashSet::from_iter(grid.points.clone().into_iter());
    let mut q = VecDeque::from([(0, 0, 0)]);
    let goal = (grid.width, grid.height);
    let mut visited = HashSet::new();

    while let Some((x, y, cost)) = q.pop_front() {
        // println!("({},{},{}) q={:?}", x, y, cost, q);
        if (x, y) == goal {
            return Some(cost);
        }
        if !visited.insert((x, y)) {
            continue;
        }
        for &(dx, dy) in DIRS.iter() {
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 0
                && nx <= grid.width
                && ny >= 0
                && ny <= grid.height
                && !points.contains(&(nx, ny))
            {
                let np = (nx, ny, cost + 1);
                q.push_back(np);
            }
        }
    }
    None
}

type Point = (isize, isize);

#[derive(Debug)]
struct Grid {
    width: isize,
    height: isize,
    points: Vec<Point>,
}

impl Grid {
    fn get(&self, p: &Point) -> char {
        let (x, y) = *p;
        if x < 0 || y < 0 || x > self.width || y > self.height {
            return '#';
        }
        if self.points.contains(&p) {
            '#'
        } else {
            '.'
        }
    }

    fn truncate(&self, size: usize) -> Grid {
        Self {
            width: self.width,
            height: self.height,
            points: self.points.iter().take(size).copied().collect(),
        }
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (_, grid) = parse_grid(input).map_err(|e| anyhow!("{e}"))?;
        Ok(grid)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in 0..=self.height {
            for x in 0..=self.width {
                write!(f, "{}", self.get(&(x, y)))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_grid(input: &str) -> IResult<&str, Grid> {
    let parse_point = map(
        separated_pair(complete::i32, tag(","), complete::i32),
        |(x, y)| (x as isize, y as isize),
    );
    let (input, points) = separated_list0(line_ending, parse_point)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = eof(input)?;
    let width = points.iter().map(|(x, _)| *x).max().unwrap_or_default();
    let height = points.iter().map(|(_, y)| *y).max().unwrap_or_default();
    Ok((
        input,
        Grid {
            width,
            height,
            points,
        },
    ))
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT, 12)?;
        assert_eq!(result, 22);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT, 12)?;
        assert_eq!(result, (6, 1));
        Ok(())
    }
}
