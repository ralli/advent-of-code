use std::collections::HashMap;

use nom::character::complete::{alpha1, line_ending};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::IResult;
use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex};

type Grid = Vec<Vec<char>>;

fn main() -> anyhow::Result<()> {
    let input = read_file("./day-12/input.txt")?;

    let result = part1(&input).unwrap();
    println!("{}", result);

    let result = part2(&input).unwrap();
    println!("{}", result);

    Ok(())
}

fn create_graph(grid: &Grid) -> DiGraph<usize, (), usize> {
    let height = grid.len();
    let width = grid[0].len();
    let mut edges = Vec::new();

    for r in 0..height {
        for c in 0..width {
            let moves = possible_moves(grid, (r, c));
            let from = to_index(r, c, width);
            for m in moves {
                let to = to_index(m.0, m.1, width);
                edges.push((from, to));
            }
        }
    }

    DiGraph::from_edges(&edges)
}

fn part1(input: &str) -> Option<usize> {
    let (_, grid) = grid(input).unwrap();
    let g = create_graph(&grid);
    let start = find_value(&grid, 'S')?;
    let end = find_value(&grid, 'E')?;
    let node_map: HashMap<NodeIndex<usize>, usize> =
        dijkstra(&g, start.into(), Some(end.into()), |_| 1);
    let start_idx = NodeIndex::from(end);

    node_map.get(&start_idx).copied()
}

fn part2(input: &str) -> Option<usize> {
    let (_, grid) = grid(input).unwrap();
    let width = grid[0].len();
    let mut g = create_graph(&grid);
    let end = find_value(&grid, 'E')?;

    g.reverse();
    let node_map: HashMap<NodeIndex<usize>, usize> = dijkstra(&g, end.into(), None, |_| 1);

    node_map
        .iter()
        .filter_map(|(k, v)| {
            let idx = k.index();
            let r = idx / width;
            let c = idx % width;
            if grid[r][c] == 'a' || grid[r][c] == 'S' {
                Some(*v)
            } else {
                None
            }
        })
        .min()
}

fn find_value(grid: &Grid, value: char) -> Option<usize> {
    let width = grid[0].len();

    for (r, row) in grid.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            if v == value {
                return Some(to_index(r, c, width));
            }
        }
    }

    None
}

fn possible_moves(grid: &Grid, from: (usize, usize)) -> Vec<(usize, usize)> {
    let height = grid.len();
    let width = grid[0].len();
    let mut edges: Vec<(usize, usize)> = Vec::new();
    let (r, c) = from;

    if r > 0 && is_valid(grid[r][c], grid[r - 1][c]) {
        edges.push((r - 1, c));
    }

    if r + 1 < height && is_valid(grid[r][c], grid[r + 1][c]) {
        edges.push((r + 1, c))
    }

    if c > 0 && is_valid(grid[r][c], grid[r][c - 1]) {
        edges.push((r, c - 1))
    }

    if c + 1 < width && is_valid(grid[r][c], grid[r][c + 1]) {
        edges.push((r, c + 1))
    }

    edges
}

fn is_valid(from: char, to: char) -> bool {
    let from_val = to_code(from);
    let to_val = to_code(to);
    to_val - from_val <= 1
}

fn to_code(c: char) -> i32 {
    match c {
        'S' => 'a' as i32,
        'E' => 'z' as i32,
        _ => c as i32,
    }
}

fn to_index(row: usize, col: usize, width: usize) -> usize {
    row * width + col
}

fn grid(input: &str) -> IResult<&str, Grid> {
    separated_list1(line_ending, map(alpha1, |s: &str| s.chars().collect()))(input)
}

fn read_file(filename: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1_works() {
        let result = part1(INPUT).unwrap();
        let expected = 31;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT).unwrap();
        let expected = 29;
        assert_eq!(result, expected);
    }
}
