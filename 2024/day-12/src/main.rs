use anyhow::Context;
use std::collections::{BTreeSet, VecDeque};
use std::fs;

fn main() -> anyhow::Result<()> {
    let filename = "day-12/input.txt";
    let content = fs::read_to_string(filename).context(format!("cannot load {filename}"))?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let grid = parse_grid(input)?;
    let mut visited = BTreeSet::new();
    let mut total_price = 0;
    for row in 0..grid.height {
        for col in 0..grid.width {
            if !visited.contains(&(row, col)) {
                let (area, perimeter) = area_and_perimiter(&grid, row, col, &mut visited);
                // println!(
                //     "type={}, area={}, perimeter={}, price={}",
                //     grid.get(row, col),
                //     area,
                //     perimeter,
                //     area * perimeter
                // );
                total_price += area * perimeter;
            }
        }
    }
    Ok(total_price)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let grid = parse_grid(input)?;
    let mut visited = BTreeSet::new();
    let mut total_price = 0;
    for row in 0..grid.height {
        for col in 0..grid.width {
            if !visited.contains(&(row, col)) {
                let (area, perimeter) = area_and_perimiter2(&grid, row, col, &mut visited);
                // println!(
                //     "type={}, area={}, perimeter={}, price={}",
                //     grid.get(row, col),
                //     area,
                //     perimeter,
                //     area * perimeter
                // );
                total_price += area * perimeter;
            }
        }
    }
    Ok(total_price)
}

type Position = (isize, isize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

impl Side {
    fn direction(&self) -> Position {
        match self {
            Side::Top => (-1, 0),
            Side::Right => (0, 1),
            Side::Bottom => (1, 0),
            Side::Left => (0, -1),
        }
    }

    fn edge_direction(&self) -> Position {
        match self {
            Side::Top | Side::Bottom => (0, 1),
            Side::Left | Side::Right => (1, 0),
        }
    }
}

type PositionAndSide = (isize, isize, Side);

#[derive(Debug)]
struct Grid {
    width: isize,
    height: isize,
    cells: Vec<Vec<char>>,
}

impl Grid {
    fn get(&self, row: isize, col: isize) -> char {
        if self.is_in_bounds(row, col) {
            self.cells[row as usize][col as usize]
        } else {
            '.'
        }
    }

    fn is_in_bounds(&self, row: isize, col: isize) -> bool {
        row >= 0 && row < self.height && col >= 0 && col < self.width
    }
}

const DIRS: [Position; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn area_and_perimiter(
    grid: &Grid,
    start_row: isize,
    start_col: isize,
    visited: &mut BTreeSet<Position>,
) -> (usize, usize) {
    let plant_type = grid.get(start_row, start_col);
    let mut q = VecDeque::from([(start_row, start_col)]);
    let mut area = 0;
    let mut perimeter = 0;
    while let Some((row, col)) = q.pop_front() {
        if visited.contains(&(row, col)) {
            continue;
        }
        visited.insert((row, col));
        area += 1;
        for &(dr, dc) in DIRS.iter() {
            let (next_row, next_col) = (row + dr, col + dc);
            if grid.get(next_row, next_col) == plant_type {
                q.push_back((next_row, next_col));
            } else {
                perimeter += 1;
            }
        }
    }
    (area, perimeter)
}

fn area_and_perimiter2(
    grid: &Grid,
    start_row: isize,
    start_col: isize,
    visited: &mut BTreeSet<Position>,
) -> (usize, usize) {
    let plant_type = grid.get(start_row, start_col);
    let mut q = VecDeque::from([(start_row, start_col)]);
    let mut area = 0;
    let mut perimeter = 0;
    let mut visited_edges = BTreeSet::new();

    while let Some((row, col)) = q.pop_front() {
        if visited.contains(&(row, col)) {
            continue;
        }
        visited.insert((row, col));
        area += 1;
        for &(dr, dc) in DIRS.iter() {
            let (next_row, next_col) = (row + dr, col + dc);
            if grid.get(next_row, next_col) == plant_type {
                q.push_back((next_row, next_col));
            }
        }
        perimeter += visit_edge(grid, row, col, Side::Top, &mut visited_edges);
        perimeter += visit_edge(grid, row, col, Side::Right, &mut visited_edges);
        perimeter += visit_edge(grid, row, col, Side::Bottom, &mut visited_edges);
        perimeter += visit_edge(grid, row, col, Side::Left, &mut visited_edges);
    }
    (area, perimeter)
}

fn visit_edge(
    grid: &Grid,
    start_row: isize,
    start_col: isize,
    side: Side,
    visited_edges: &mut BTreeSet<PositionAndSide>,
) -> usize {
    let (side_dr, side_dc) = side.direction();
    let plant_type = grid.get(start_row, start_col);

    if grid.get(start_row + side_dr, start_col + side_dc) == plant_type {
        return 0;
    }

    if visited_edges.contains(&(start_row, start_col, side)) {
        return 0;
    };

    let (dr, dc) = side.edge_direction();

    let (mut row, mut col) = (start_row, start_col);
    while grid.get(row, col) == plant_type && grid.get(row + side_dr, col + side_dc) != plant_type {
        visited_edges.insert((row, col, side));
        row += dr;
        col += dc;
    }

    let (mut row, mut col) = (start_row, start_col);
    while grid.get(row, col) == plant_type && grid.get(row + side_dr, col + side_dc) != plant_type {
        visited_edges.insert((row, col, side));
        row -= dr;
        col -= dc;
    }

    1
}

fn parse_grid(input: &str) -> anyhow::Result<Grid> {
    let cells: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.chars().filter(|c| c.is_ascii_alphabetic()).collect())
        .collect();
    let width = cells.first().map(|r| r.len() as isize).unwrap_or_default();
    let height = cells.len() as isize;
    Ok(Grid {
        width,
        height,
        cells,
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    #[test]
    fn test_area_and_perimiter() -> anyhow::Result<()> {
        let grid = parse_grid(INPUT)?;
        let mut visited = BTreeSet::new();
        let row = 0;
        let col = 0;
        let (area, perimeter) = area_and_perimiter(&grid, row, col, &mut visited);
        println!(
            "type={}, area={}, perimeter={}, price={}",
            grid.get(row, col),
            area,
            perimeter,
            area * perimeter
        );
        Ok(())
    }

    #[test]
    fn test_area_and_perimiter2() -> anyhow::Result<()> {
        let grid = parse_grid(INPUT)?;
        let mut visited = BTreeSet::new();
        let row = 0;
        let col = 0;
        let (area, perimeter) = area_and_perimiter2(&grid, row, col, &mut visited);
        println!(
            "type={}, area={}, perimeter={}, price={}",
            grid.get(row, col),
            area,
            perimeter,
            area * perimeter
        );
        Ok(())
    }

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        assert_eq!(result, 1930);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        assert_eq!(result, 1206);
        Ok(())
    }
}
