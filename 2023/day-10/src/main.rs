use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt::Formatter;
use std::{fmt, fs, iter};

use anyhow::{anyhow, Context};

fn main() -> anyhow::Result<()> {
    let filename = "day-10.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<isize> {
    let grid = parse_input(input)?;
    let start_position = grid.find_start_pos();
    let mut distances = BTreeMap::from([(start_position, vec![0])]);
    let mut q: VecDeque<(isize, isize, isize, isize, isize)> =
        VecDeque::from([(start_position.0, start_position.1, -1, -1, 0)]);
    let mut visited: BTreeSet<(isize, isize, isize, isize)> = BTreeSet::new();
    while let Some((row, col, from_row, from_col, distance)) = q.pop_front() {
        let current_pipe = grid.pipes[row as usize][col as usize];
        let next_moves: Vec<(isize, isize)> = grid
            .valid_moves(row, col)
            .into_iter()
            .filter(|(r, c)| *r != from_row || *c != from_col)
            .collect();
        // println!("{} {} {} {} {:?}", row, col, from_row, from_col, next_moves);
        for (next_row, next_col) in next_moves {
            if visited.insert((row, col, next_row, next_col)) {
                q.push_back((next_row, next_col, row, col, distance + 1));
                distances
                    .entry((next_row, next_col))
                    .or_default()
                    .push(distance + 1);
            }
        }
    }
    // println!("{distances:?}",);
    // println!("{grid}");
    // println!();
    //
    // for row in 0..grid.height as usize {
    //     for col in (0..grid.width as usize) {
    //         if let Some(v) = distances.get(&(row as isize, col as isize)) {
    //             let m = v.iter().min().copied().unwrap_or_default();
    //             print!("{m:2}");
    //         } else {
    //             print!(" .");
    //         }
    //     }
    //     println!();
    // }

    let result = distances
        .values()
        .map(|v| v.iter().min().copied().unwrap_or_default())
        .max()
        .unwrap_or_default();
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<isize> {
    let grid = parse_input(input)?;
    let start_position = grid.find_start_pos();
    let mut q: VecDeque<(isize, isize, isize, isize, Vec<(isize, isize)>)> =
        VecDeque::from([(start_position.0, start_position.1, -1, -1, Vec::new())]);
    // let mut visited: BTreeSet<(isize, isize, isize, isize)> = BTreeSet::new();
    let mut result = Vec::new();
    while let Some((row, col, from_row, from_col, path)) = q.pop_front() {
        let next_path: Vec<_> = path.iter().copied().chain(iter::once((row, col))).collect();
        let next_moves: Vec<(isize, isize)> = grid
            .valid_moves(row, col)
            .into_iter()
            .filter(|(r, c)| *r != from_row || *c != from_col)
            .collect();
        for (next_row, next_col) in next_moves {
            q.push_back((next_row, next_col, row, col, next_path.clone()));
        }
        let current_pipe = grid.pipes[row as usize][col as usize];

        if current_pipe == PipeType::Start && !path.is_empty() {
            result = next_path;
            break;
        }
    }

    let mut positions = BTreeSet::from_iter(result.into_iter());

    let mut result = 0;
    let mut found = BTreeSet::new();

    for row in (0..grid.height as usize) {
        let mut even_odd = 0;
        let mut last = PipeType::Empty;
        for col in (0..grid.width as usize).rev() {
            let current = grid.pipes[row][col];
            if !positions.contains(&(row as isize, col as isize)) {
                if even_odd % 2 == 1 {
                    result += 1;
                    found.insert((row as isize, col as isize));
                }
            } else {
                match (last, current) {
                    (PipeType::SouthToWest, PipeType::SouthToEast) => {
                        even_odd += 1;
                    }
                    (PipeType::SouthToWest, PipeType::NorthToEast) => {
                        // ignore
                    }
                    (PipeType::NorthToWest, PipeType::NorthToEast) => {
                        even_odd += 1;
                    }
                    (PipeType::NorthToWest, PipeType::SouthToEast) => {
                        // ignore
                    }
                    _ => {
                        if current != PipeType::Horizontal {
                            last = current;
                            even_odd += 1;
                        }
                    }
                };
            }
        }
    }
    // for row in (0..grid.height as usize) {
    //     for col in 0..grid.width as usize {
    //         if found.contains(&(row as isize, col as isize)) {
    //             print!("I");
    //             // print!("{}", grid.pipes[row][col])
    //         } else if positions.contains(&(row as isize, col as isize)) {
    //             print!("{}", grid.pipes[row][col])
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }
    // println!("{found:?}");
    Ok(found.len() as isize)
}

fn parse_input(input: &str) -> anyhow::Result<Grid> {
    let pipes: anyhow::Result<Vec<Vec<PipeType>>> = input.lines().map(parse_line).collect();
    let pipes = pipes?;
    let height = pipes.len() as isize;
    let width = pipes.first().map(|p| p.len()).unwrap_or_default() as isize;
    Ok(Grid {
        pipes,
        width,
        height,
    })
}

fn parse_line(input: &str) -> anyhow::Result<Vec<PipeType>> {
    input.chars().map(|c| PipeType::try_from(c)).collect()
}

#[derive(Debug)]
struct Grid {
    pipes: Vec<Vec<PipeType>>,
    width: isize,
    height: isize,
}

impl Grid {
    fn find_start_pos(&self) -> (isize, isize) {
        for row in 0..self.height {
            for col in 0..self.width {
                if self.pipes[row as usize][col as usize] == PipeType::Start {
                    return (row, col);
                }
            }
        }
        unreachable!("no start position");
    }

    // returns positions, not deltas!
    fn valid_moves(&self, row: isize, col: isize) -> Vec<(isize, isize)> {
        DELTAS
            .iter()
            .filter(|(dr, dc)| self.is_valid_move(row, col, *dr, *dc))
            .map(|(dr, dc)| (row + *dr, col + *dc))
            .collect()
    }

    fn is_valid_move(&self, current_row: isize, current_col: isize, dr: isize, dc: isize) -> bool {
        let next_row = current_row + dr;
        let next_col = current_col + dc;
        if next_row < 0 || next_row >= self.height {
            return false;
        }
        if next_col < 0 || next_col >= self.width {
            return false;
        }
        let current_pipe = self.pipes[current_row as usize][current_col as usize];
        let next_pipe = self.pipes[next_row as usize][next_col as usize];
        VALID_MOVES.iter().any(|(cur, (xdr, xdc), nxt)| {
            current_pipe == *cur && dr == *xdr && dc == *xdc && next_pipe == *nxt
        })
    }
}
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in 0..self.height as usize {
            for col in 0..self.width as usize {
                write!(f, "{}", self.pipes[row][col])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum PipeType {
    Vertical,    // |
    Horizontal,  // -
    NorthToEast, // L
    NorthToWest, // J
    SouthToWest, // 7
    SouthToEast, // F
    Start,
    Empty,
}

static DELTAS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

static VALID_MOVES: [(PipeType, (isize, isize), PipeType); 60] = [
    (PipeType::Horizontal, (0, 1), PipeType::NorthToWest),
    (PipeType::Horizontal, (0, 1), PipeType::SouthToWest),
    (PipeType::Horizontal, (0, 1), PipeType::Horizontal),
    (PipeType::Horizontal, (0, 1), PipeType::Start),
    (PipeType::Horizontal, (0, -1), PipeType::NorthToEast),
    (PipeType::Horizontal, (0, -1), PipeType::SouthToEast),
    (PipeType::Horizontal, (0, -1), PipeType::Horizontal),
    (PipeType::Horizontal, (0, -1), PipeType::Start),
    //
    (PipeType::Vertical, (-1, 0), PipeType::SouthToEast),
    (PipeType::Vertical, (-1, 0), PipeType::SouthToWest),
    (PipeType::Vertical, (-1, 0), PipeType::Vertical),
    (PipeType::Vertical, (-1, 0), PipeType::Start),
    (PipeType::Vertical, (1, 0), PipeType::NorthToEast),
    (PipeType::Vertical, (1, 0), PipeType::NorthToWest),
    (PipeType::Vertical, (1, 0), PipeType::Vertical),
    (PipeType::Vertical, (1, 0), PipeType::Start),
    //
    (PipeType::NorthToEast, (0, 1), PipeType::Horizontal),
    (PipeType::NorthToEast, (0, 1), PipeType::NorthToWest),
    (PipeType::NorthToEast, (0, 1), PipeType::SouthToWest),
    (PipeType::NorthToEast, (0, 1), PipeType::Start),
    (PipeType::NorthToEast, (-1, 0), PipeType::Vertical),
    (PipeType::NorthToEast, (-1, 0), PipeType::SouthToEast),
    (PipeType::NorthToEast, (-1, 0), PipeType::SouthToWest),
    (PipeType::NorthToEast, (-1, 0), PipeType::Start),
    //
    (PipeType::NorthToWest, (0, -1), PipeType::Horizontal),
    (PipeType::NorthToWest, (0, -1), PipeType::NorthToEast),
    (PipeType::NorthToWest, (0, -1), PipeType::SouthToEast),
    (PipeType::NorthToWest, (0, -1), PipeType::Start),
    (PipeType::NorthToWest, (-1, 0), PipeType::Vertical),
    (PipeType::NorthToWest, (-1, 0), PipeType::SouthToWest),
    (PipeType::NorthToWest, (-1, 0), PipeType::SouthToEast),
    (PipeType::NorthToWest, (-1, 0), PipeType::Start),
    //
    (PipeType::SouthToEast, (0, 1), PipeType::Horizontal),
    (PipeType::SouthToEast, (0, 1), PipeType::NorthToWest),
    (PipeType::SouthToEast, (0, 1), PipeType::SouthToWest),
    (PipeType::SouthToEast, (0, 1), PipeType::Start),
    (PipeType::SouthToEast, (1, 0), PipeType::Vertical),
    (PipeType::SouthToEast, (1, 0), PipeType::NorthToEast),
    (PipeType::SouthToEast, (1, 0), PipeType::NorthToWest),
    (PipeType::SouthToEast, (1, 0), PipeType::Start),
    //
    (PipeType::SouthToWest, (0, -1), PipeType::Horizontal),
    (PipeType::SouthToWest, (0, -1), PipeType::NorthToEast),
    (PipeType::SouthToWest, (0, -1), PipeType::SouthToEast),
    (PipeType::SouthToWest, (0, -1), PipeType::Start),
    (PipeType::SouthToWest, (1, 0), PipeType::Vertical),
    (PipeType::SouthToWest, (1, 0), PipeType::NorthToEast),
    (PipeType::SouthToWest, (1, 0), PipeType::NorthToWest),
    (PipeType::SouthToWest, (1, 0), PipeType::Start),
    //
    (PipeType::Start, (-1, 0), PipeType::Vertical),
    (PipeType::Start, (-1, 0), PipeType::SouthToWest),
    (PipeType::Start, (-1, 0), PipeType::SouthToEast),
    (PipeType::Start, (0, 1), PipeType::Horizontal),
    (PipeType::Start, (0, 1), PipeType::NorthToWest),
    (PipeType::Start, (0, 1), PipeType::SouthToWest),
    (PipeType::Start, (1, 0), PipeType::Vertical),
    (PipeType::Start, (1, 0), PipeType::NorthToWest),
    (PipeType::Start, (1, 0), PipeType::NorthToEast),
    (PipeType::Start, (0, -1), PipeType::Horizontal),
    (PipeType::Start, (0, -1), PipeType::NorthToEast),
    (PipeType::Start, (0, -1), PipeType::SouthToEast),
];

impl TryFrom<char> for PipeType {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(PipeType::Vertical),
            '-' => Ok(PipeType::Horizontal),
            'L' => Ok(PipeType::NorthToEast),
            'J' => Ok(PipeType::NorthToWest),
            '7' => Ok(PipeType::SouthToWest),
            'F' => Ok(PipeType::SouthToEast),
            'S' => Ok(PipeType::Start),
            '.' => Ok(PipeType::Empty),
            _ => Err(anyhow!("unknown pipe type {}", value)),
        }
    }
}

impl fmt::Display for PipeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            PipeType::Vertical => '|',
            PipeType::Horizontal => '-',
            PipeType::NorthToEast => 'L',
            PipeType::NorthToWest => 'J',
            PipeType::SouthToWest => '7',
            PipeType::SouthToEast => 'F',
            PipeType::Start => 'S',
            PipeType::Empty => '.',
        };
        write!(f, "{c}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"-L|F7
7S-7|
L|7||
-L-J|
L|-JF"#;

    static INPUT2: &str = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#;

    #[test]
    fn part1_works_on_input1() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 4;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part1_works_on_input2() -> anyhow::Result<()> {
        let result = part1(INPUT2)?;
        let expected = 8;
        assert_eq!(result, expected);
        Ok(())
    }

    static INPUT3: &str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT3)?;
        let expected = 4;
        assert_eq!(result, expected);
        Ok(())
    }
}
