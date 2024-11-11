use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt::Formatter;
use std::str::FromStr;
use std::{fmt, fs, iter};

use anyhow::{anyhow, Context};

fn main() -> anyhow::Result<()> {
    let filename = "day-23.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i64> {
    let grid: Grid = input.parse()?;
    let start = grid.find_start().ok_or_else(|| anyhow!("no start"))?;
    let mut q = VecDeque::from([(start, Vec::new())]);

    let mut result = 0;
    while let Some(((row, col), path)) = q.pop_front() {
        if row + 1 == grid.height {
            result = result.max(path.len());
        }
        let directions = grid.next_directions(row, col);
        for &(dr, dc) in directions.iter() {
            let (next_row, next_col) = (row + dr, col + dc);
            if next_row >= 0
                && next_row < grid.height
                && next_col >= 0
                && next_col < grid.width
                && grid.cells[next_row as usize][next_col as usize] != '#'
                && !path.contains(&(next_row, next_col))
            {
                let next_path = path
                    .iter()
                    .copied()
                    .chain(iter::once((next_row, next_col)))
                    .collect();
                q.push_back(((next_row, next_col), next_path));
            }
        }
        //
    }
    Ok(result as i64)
}

fn part2(input: &str) -> anyhow::Result<i64> {
    let grid: Grid = input.parse()?;
    let start = grid
        .find_start()
        .ok_or_else(|| anyhow!("no start position"))?;
    let end = grid.find_end().ok_or_else(|| anyhow!("no end position"))?;
    // longest path of a graph is NP-complete (see: https://en.wikipedia.org/wiki/Longest_path_problem)
    // this graph has many nodes without branches. You can get a feasible runtime by compressing all such consecutive edges
    // into a single one.
    let graph = build_compressed_graph(&grid, &start, &end);
    let mut visited = BTreeSet::new();
    let result = dfs(&graph, &start, &end, &mut visited);
    Ok(result)
}

fn build_compressed_graph(
    grid: &Grid,
    start: &Point,
    end: &Point,
) -> BTreeMap<Point, Vec<(Point, i64)>> {
    let mut graph: BTreeMap<Point, Vec<(Point, i64)>> = BTreeMap::new();
    // these are all nodes within the grid with more than two branches (where i come from + more than a single choice to move on)
    let points = find_points_with_branches(grid, start, end);

    //
    // for each "branching point" find edges to the directly adjacent "branching points"
    // the resulting graph is compressed, but contains silly cycles which are filtered out in the DFS step later on...
    // see the attached mermaid diagram for the test input in the project
    //
    for &(start_row, start_col) in points.iter() {
        let mut q: VecDeque<(Point, i64)> = VecDeque::from([((start_row, start_col), 0)]);
        let mut visited = BTreeSet::from([(start_row, start_col)]);

        while let Some(((row, col), distance)) = q.pop_front() {
            // distance != 0 means "not start node"
            if distance != 0 && points.contains(&(row, col)) {
                graph
                    .entry((start_row, start_col))
                    .or_default()
                    .push(((row, col), distance));
            } else {
                for (dr, dc) in DIRECTIONS.iter() {
                    let (next_row, next_col) = (row + dr, col + dc);
                    if next_row >= 0
                        && next_row < grid.height
                        && next_col >= 0
                        && next_col < grid.width
                        && grid.get(next_row, next_col) != '#'
                        && visited.insert((next_row, next_col))
                    {
                        q.push_back(((next_row, next_col), distance + 1));
                    }
                }
            }
        }
    }

    graph
}

fn find_points_with_branches(grid: &Grid, start: &Point, end: &Point) -> Vec<Point> {
    let mut points = vec![*start, *end];

    for (row, line) in grid.cells.iter().enumerate() {
        let row = row as isize;
        for (col, _) in line.iter().enumerate().filter(|(_, c)| **c != '#') {
            let col = col as isize;
            let direction_count = DIRECTIONS
                .iter()
                .filter(|(dr, dc)| {
                    let next_row = row + dr;
                    let next_col = col + dc;
                    next_row >= 0
                        && next_row < grid.height
                        && next_col >= 0
                        && next_col <= grid.width
                        && grid.get(next_row, next_col) != '#'
                })
                .count();
            if direction_count > 2 {
                points.push((row, col))
            }
        }
    }
    points
}

// must do it recurively since i do not know when to insert/remove the nodes from
// visited otherwise...
fn dfs(
    graph: &BTreeMap<Point, Vec<(Point, i64)>>,
    pos: &Point,
    end: &Point,
    visited: &mut BTreeSet<Point>,
) -> i64 {
    if pos == end {
        0
    } else {
        visited.insert(*pos);
        let edges = graph.get(pos).unwrap();
        let mut m = i64::MIN;
        for (p, d) in edges.iter() {
            if !visited.contains(p) {
                m = m.max(dfs(graph, p, end, visited) + *d);
            }
        }
        visited.remove(pos);
        m
    }
}

fn print_mermaid_graph(graph: &BTreeMap<Point, Vec<(Point, i64)>>) {
    println!("flowchart");
    for ((sr, sc), edges) in graph.iter() {
        for ((dr, dc), distance) in edges.iter() {
            println!("  {sr}_{sc}[{sr},{sc}] --{distance}--> {dr}_{dc}[{dr},{dc}]");
        }
    }
}

static DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

type Point = (isize, isize);
type State = (Point, Point, i64);
#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<char>>,
    width: isize,
    height: isize,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        let height = cells.len() as isize;
        let width = cells
            .first()
            .filter(|line| !line.is_empty())
            .map(|line| line.len())
            .unwrap_or_default() as isize;
        Ok(Grid {
            cells,
            width,
            height,
        })
    }
}

impl Grid {
    fn get(&self, row: isize, col: isize) -> char {
        self.cells[row as usize][col as usize]
    }

    fn find_start(&self) -> Option<(isize, isize)> {
        self.cells.first().and_then(|line| {
            line.iter()
                .enumerate()
                .find(|(col, c)| **c == '.')
                .map(|(col, _)| (0isize, col as isize))
        })
    }

    fn find_end(&self) -> Option<(isize, isize)> {
        self.cells.last().and_then(|line| {
            line.iter()
                .enumerate()
                .find(|(col, c)| **c == '.')
                .map(|(col, _)| (self.height - 1, col as isize))
        })
    }

    fn next_directions(&self, row: isize, col: isize) -> &'static [(isize, isize)] {
        match self.cells[row as usize][col as usize] {
            '^' => [(-1, 0)].as_slice(),
            'v' => [(1, 0)].as_slice(),
            '<' => [(0, -1)].as_slice(),
            '>' => [(0, 1)].as_slice(),
            _ => [(-1, 0), (1, 0), (0, -1), (0, 1)].as_slice(),
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in self.cells.iter() {
            for ch in line.iter() {
                write!(f, "{ch}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 94;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 154;
        assert_eq!(result, expected);
        Ok(())
    }
}
