use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

use pathfinding::prelude::dijkstra;

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let grid: Grid = input.parse()?;
    let start = (0, 0);
    let goal = (grid.height as i32 - 1, grid.width as i32 - 1);
    let min = 0;
    let max = 3;
    let start_edge = Edge {
        pos: start,
        direction: Direction::Right,
        direction_count: 0,
    };
    let (_path, result) = dijkstra(
        &start_edge,
        |e| successors(&grid, e, min, max),
        |e| e.pos == goal,
    )
    .unwrap();
    Ok(result)
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    let grid: Grid = input.parse()?;
    let start = (0, 0);
    let goal = (grid.height as i32 - 1, grid.width as i32 - 1);
    let min = 4;
    let max = 10;
    let start_edge = Edge {
        pos: start,
        direction: Direction::Right,
        direction_count: 0,
    };
    let (_path, result) = dijkstra(
        &start_edge,
        |e| successors(&grid, e, min, max),
        |e| e.pos == goal,
    )
    .unwrap();
    Ok(result)
}

fn successors(grid: &Grid, edge: &Edge, min: u32, max: u32) -> Vec<(Edge, usize)> {
    let mut result = Vec::new();
    static DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    let opposite = edge.direction.opposite();
    let (row, col) = edge.pos;
    for &direction in DIRECTIONS.iter() {
        if direction == opposite {
            continue;
        }
        if direction != edge.direction && edge.direction_count < min {
            continue;
        }
        let (dr, dc) = direction.delta();
        let (next_row, next_col) = (row + dr, col + dc);
        if next_row < 0
            || next_row >= grid.height as i32
            || next_col < 0
            || next_col >= grid.width as i32
        {
            continue;
        }
        let next_direction_count = if direction == edge.direction {
            edge.direction_count + 1
        } else {
            1
        };
        if next_direction_count > max {
            continue;
        }

        let distance = grid.cells[next_row as usize][next_col as usize];
        result.push((
            Edge {
                pos: (next_row, next_col),
                direction,
                direction_count: next_direction_count,
            },
            distance,
        ));
    }
    // println!("{result:?}");
    result
}

#[derive(Debug)]
struct PrioQueue {
    edges: Vec<Edge>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Edge {
    pos: (i32, i32),
    direction: Direction,
    direction_count: u32,
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells: Vec<Vec<usize>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();
        let height = cells.len();
        let width = cells.first().map(|l| l.len()).unwrap_or_default();
        Ok(Grid {
            cells,
            width,
            height,
        })
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (row, line) in self.cells.iter().enumerate() {
            for (col, c) in line.iter().enumerate() {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 102;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 94;
        assert_eq!(result, expected);
        Ok(())
    }
}
