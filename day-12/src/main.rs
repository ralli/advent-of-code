use nom::character::complete::{alpha1, line_ending};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::{HashSet, VecDeque};

type Grid = Vec<Vec<char>>;

fn main() -> anyhow::Result<()> {
    let input = read_file("./day-12/input.txt")?;

    let result = part1(&input).unwrap();
    println!("{}", result);

    let result = part2(&input).unwrap();
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> Option<usize> {
    let (_, grid) = grid(input).unwrap();
    let start = find_value(&grid, 'S')?;
    find_distance(&grid, start)
}

fn part2(input: &str) -> Option<usize> {
    let (_, grid) = grid(input).unwrap();
    let mut results = Vec::new();
    let height = grid.len();
    let width = grid[0].len();

    for r in 0..height {
        for c in 0..width {
            if grid[r][c] == 'a' || grid[r][c] == 'S' {
                results.push(find_distance(&grid, (r, c)));
            }
        }
    }

    results.iter().flatten().min().copied()
}

fn find_distance(grid: &Grid, start: (usize, usize)) -> Option<usize> {
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();

    visited.insert(start);
    let initial_moves = possible_moves(grid, 1, start);
    for m in initial_moves {
        if !visited.contains(&(m.0, m.1)) {
            q.push_back(m);
            visited.insert((m.0, m.1));
        }
    }

    while !q.is_empty() {
        let (row, col, distance) = q.pop_front().unwrap();
        if grid[row][col] == 'E' {
            return Some(distance);
        }
        let moves = possible_moves(&grid, distance + 1, (row, col));
        for m in moves.into_iter() {
            if !visited.contains(&(m.0, m.1)) {
                q.push_back(m);
                visited.insert((m.0, m.1));
            }
        }
    }

    None
}

fn find_value(grid: &Grid, value: char) -> Option<(usize, usize)> {
    let height = grid.len();
    let width = grid[0].len();

    for r in 0..height {
        for c in 0..width {
            if grid[r][c] == value {
                return Some((r, c));
            }
        }
    }

    None
}

fn possible_moves(
    grid: &Grid,
    distance: usize,
    from: (usize, usize),
) -> Vec<(usize, usize, usize)> {
    let height = grid.len();
    let width = grid[0].len();
    let mut edges: Vec<(usize, usize, usize)> = Vec::new();
    let (r, c) = from;

    if r > 0 && is_valid(grid[r][c], grid[r - 1][c]) {
        edges.push((r - 1, c, distance));
    }

    if r + 1 < height && is_valid(grid[r][c], grid[r + 1][c]) {
        edges.push((r + 1, c, distance))
    }

    if c > 0 && is_valid(grid[r][c], grid[r][c - 1]) {
        edges.push((r, c - 1, distance))
    }

    if c + 1 < width && is_valid(grid[r][c], grid[r][c + 1]) {
        edges.push((r, c + 1, distance))
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
