use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let grid: Grid = input.parse()?;
    let mut q = BinaryHeap::from([Edge {
        distance: 0,
        pos: (0, 0),
        direction: Direction::Right, // first direction does not matter (direction_count = 0)
        direction_count: 0,
    }]);

    let mut distances: BTreeMap<(i32, i32), u32> = BTreeMap::from([((0, 0), 0)]);
    let mut pred: BTreeMap<(i32, i32), Edge> = BTreeMap::new();

    while let Some(current) = q.pop() {
        let (row, col) = current.pos;
        // if row + 1 == grid.width as i32 && col + 1 == grid.height as i32 {
        //     break;
        // }
        let mut next: Vec<Edge> = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .filter(|&&d| d != current.direction.opposite())
        .filter_map(|&d| {
            let (dr, dc) = d.delta();
            let direction_count = if d == current.direction {
                current.direction_count + 1
            } else {
                1
            };
            if direction_count > 3 {
                return None;
            }
            let next_row = row + dr;
            if next_row < 0 || next_row >= grid.height as i32 {
                return None;
            }

            let next_col = col + dc;
            if next_col < 0 || next_col >= grid.width as i32 {
                return None;
            }

            let edge_distance = grid.cells[next_row as usize][next_col as usize];
            let distance = current.distance + edge_distance;
            let bla = distances.entry((next_row, next_col)).or_insert(u32::MAX);

            if distance < *bla {
                *bla = distance;
                let e = Edge {
                    distance,
                    pos: (next_row, next_col),
                    direction: d,
                    direction_count,
                };
                Some(e)
            } else {
                None
            }
        })
        .collect();
        for e in next.iter() {
            pred.insert(e.pos, current);
        }
        while let Some(e) = q.pop() {
            if !next.iter().any(|x| x.pos == e.pos) {
                next.push(e);
            }
        }
        q.extend(next.into_iter());
        // println!("{current:?}:  {q:?}");
    }

    // dbg!(&distances);
    print_grid(&grid, &pred, &distances);
    let bottom_right = ((grid.height - 1) as i32, (grid.width - 1) as i32);
    let result = distances.get(&bottom_right).copied().unwrap_or_default();
    Ok(result as usize)
}

fn path(
    grid: &Grid,
    pred: &BTreeMap<(i32, i32), Edge>,
    distances: &BTreeMap<(i32, i32), u32>,
) -> Vec<((i32, i32), (i32, i32), Direction, u32)> {
    let mut pos = (grid.height as i32 - 1, grid.width as i32 - 1);
    let mut result = Vec::new();
    while pos != (0, 0) {
        let e = pred.get(&pos).unwrap();
        let d = distances[&pos];
        result.push((e.pos, pos, e.direction, d));
        pos = e.pos;
    }
    result
}
fn print_grid(
    grid: &Grid,
    pred: &BTreeMap<(i32, i32), Edge>,
    distances: &BTreeMap<(i32, i32), u32>,
) {
    let mut map = grid
        .cells
        .iter()
        .map(|r| {
            r.iter()
                .map(|c| (c + '0' as u32) as u8 as char)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let path = path(grid, pred, distances);

    for (from, to, dir, dist) in path.into_iter().rev() {
        println!("{from:?} {to:?} {dir:?} {dist}");
        let (row, col) = to;
        map[row as usize][col as usize] = dir.to_char();
    }
    println!();
    for line in map.into_iter() {
        for c in line.into_iter() {
            print!("{c}");
        }
        println!();
    }
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Edge {
    distance: u32,
    pos: (i32, i32),
    direction: Direction,
    direction_count: u32,
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
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
        let cells: Vec<Vec<u32>> = s
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
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
    fn test1() {
        let e1 = Edge {
            distance: 0,
            pos: (1, 1),
            direction: Direction::Down,
            direction_count: 10,
        };
        let e2 = Edge {
            distance: 1,
            pos: (0, 0),
            direction: Direction::Up,
            direction_count: 0,
        };
        assert!(e1 < e2);
    }
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
        let expected = 51;
        assert_eq!(result, expected);
        Ok(())
    }
}
