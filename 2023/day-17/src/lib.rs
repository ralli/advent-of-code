use anyhow::anyhow;
use std::str::FromStr;

use pathfinding::prelude::dijkstra;

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let grid: Grid = input.parse()?;
    let start = (0, 0);
    let goal = (grid.height as i32 - 1, grid.width as i32 - 1);
    let min = 1;
    let max = 3;
    find_solution(&grid, start, goal, min, max).ok_or_else(|| anyhow!("no solution found"))
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    let grid: Grid = input.parse()?;
    let start = (0, 0);
    let goal = (grid.height as i32 - 1, grid.width as i32 - 1);
    let min = 4;
    let max = 10;
    find_solution(&grid, start, goal, min, max).ok_or_else(|| anyhow!("no solution found"))
}

fn find_solution(
    grid: &Grid,
    start: (i32, i32),
    goal: (i32, i32),
    min: u32,
    max: u32,
) -> Option<usize> {
    let start_edge = Edge {
        pos: start,
        direction: Direction::Right,
        direction_count: 0,
    };
    dijkstra(
        &start_edge,
        |e| successors(grid, e, min, max),
        |e| e.pos == goal,
    )
    .map(|(_path, distance)| distance)
}

fn successors(grid: &Grid, edge: &Edge, min: u32, max: u32) -> Vec<(Edge, usize)> {
    static DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    DIRECTIONS
        .iter()
        .filter_map(|&direction| edge_in_direction(grid, edge, direction, min, max))
        .collect()
}

fn edge_in_direction(
    grid: &Grid,
    edge: &Edge,
    direction: Direction,
    min: u32,
    max: u32,
) -> Option<(Edge, usize)> {
    if direction == edge.direction.opposite() {
        return None;
    }
    let (dr, dc) = direction.delta();
    let mut distance = 0;
    let (mut row, mut col) = edge.pos;
    let mut direction_count = if direction == edge.direction {
        edge.direction_count
    } else {
        0
    };
    if direction_count < min {
        while direction_count < min {
            row += dr;
            col += dc;
            if row < 0 || row >= grid.height as i32 || col < 0 || col >= grid.width as i32 {
                return None;
            }
            distance += grid.cells[row as usize][col as usize];
            direction_count += 1;
        }
        return Some((
            Edge {
                pos: (row, col),
                direction,
                direction_count,
            },
            distance,
        ));
    }
    row += dr;
    col += dc;
    if row < 0 || row >= grid.height as i32 || col < 0 || col >= grid.width as i32 {
        return None;
    }
    distance += grid.cells[row as usize][col as usize];
    direction_count += 1;
    if direction_count > max {
        return None;
    }
    Some((
        Edge {
            pos: (row, col),
            direction,
            direction_count,
        },
        distance,
    ))
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
