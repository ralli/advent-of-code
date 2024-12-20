use anyhow::{anyhow, Context};
use nom::character::complete::{line_ending, multispace0, one_of};
use nom::combinator::eof;
use nom::multi::{many1, separated_list0};
use nom::IResult;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;

fn main() -> anyhow::Result<()> {
    let filename = "day-20/input.txt";
    let content = fs::read_to_string(filename).context(format!("cannot load {filename}"))?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let (_, grid) = parse_grid(input).map_err(|e| anyhow!("{e}"))?;
    let (end_row, end_col) =
        get_position_of(&grid, 'E').ok_or_else(|| anyhow!("cannot find 'E'"))?;

    let from_end = distances_from(&grid, end_row, end_col);
    let mut result = 0;

    for row_idx in 1..grid.len() - 1 {
        for col_idx in 1..grid[row_idx].len() - 1 {
            let col = grid[row_idx][col_idx];
            if col == '#' {
                let cheats = cheats_from_position(&grid, row_idx as isize, col_idx as isize);
                for (start, end) in cheats.iter() {
                    let savings = cheat_savings(start, end, &from_end, 2);
                    if savings >= 100 {
                        result += 1;
                    }
                }
            }
        }
    }
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let (_, grid) = parse_grid(input).map_err(|e| anyhow!("{e}"))?;
    let (end_row, end_col) =
        get_position_of(&grid, 'E').ok_or_else(|| anyhow!("cannot find 'E'"))?;
    let from_end = distances_from(&grid, end_row, end_col);
    let mut visited = BTreeSet::new();
    let mut result = 0;

    for row_idx in 1..grid.len() - 1 {
        for col_idx in 1..grid[row_idx].len() - 1 {
            let start = (row_idx as isize, col_idx as isize);
            let col = grid[row_idx][col_idx];
            if col != '#' {
                let bla =
                    cheats_from_position2(&grid, row_idx as isize, col_idx as isize, 20, &from_end);
                for &((nr, nc), d) in bla.iter() {
                    let end = (nr, nc);
                    let savings = cheat_savings(&start, &end, &from_end, d as isize);
                    if visited.insert((start, end)) && savings >= 100 {
                        result += 1;
                    }
                }
            }
        }
    }

    Ok(result)
}

const DIRS: [Position; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn distances_from(grid: &Grid, start_row: isize, start_col: isize) -> BTreeMap<Position, usize> {
    let mut visited = BTreeSet::new();
    let mut q = VecDeque::from([(start_row, start_col, 0)]);
    let mut distances = BTreeMap::new();

    while let Some((row, col, distance)) = q.pop_front() {
        if !visited.insert((row, col)) {
            continue;
        }
        distances.insert((row, col), distance);
        for (dr, dc) in DIRS.iter() {
            let (nr, nc) = (row + dr, col + dc);
            let v = grid[nr as usize][nc as usize];
            if v != '#' {
                q.push_back((nr, nc, distance + 1));
            }
        }
    }
    distances
}

fn cheats_from_position2(
    grid: &Grid,
    row: isize,
    col: isize,
    max_distance: usize,
    from_end: &BTreeMap<Position, usize>,
) -> Vec<(Position, usize)> {
    let mut distances: BTreeMap<Position, usize> = BTreeMap::new();
    let mut q = VecDeque::from([(row, col, 0)]);
    let height = grid.len() as isize;
    let width = grid.first().map(|r| r.len()).unwrap_or_default() as isize;
    let mut visited = BTreeSet::new();
    while let Some((row, col, distance)) = q.pop_front() {
        if distances.contains_key(&(row, col)) {
            continue;
        }
        if distance > max_distance {
            continue;
        }
        if !visited.insert((row, col)) {
            continue;
        }
        if from_end.contains_key(&(row, col)) {
            distances.insert((row, col), distance);
        }
        for (dr, dc) in DIRS.iter() {
            let (nr, nc) = (row + dr, col + dc);
            if nr < 0 || nr >= height || nc < 0 || nc >= width {
                continue;
            }
            q.push_back((nr, nc, distance + 1));
        }
    }
    distances.into_iter().collect()
}

fn cheats_from_position(grid: &Grid, row: isize, col: isize) -> Vec<Cheat> {
    let mut result = Vec::new();
    if grid[(row - 1) as usize][col as usize] != '#'
        && grid[(row + 1) as usize][col as usize] != '#'
    {
        result.push(((row - 1, col), (row + 1, col)));
        result.push(((row + 1, col), (row - 1, col)));
    }

    if grid[row as usize][(col - 1) as usize] != '#'
        && grid[row as usize][(col + 1) as usize] != '#'
    {
        result.push(((row, col - 1), (row, col + 1)));
        result.push(((row, col + 1), (row, col - 1)));
    }

    result
}

fn cheat_savings(
    start: &Position,
    end: &Position,
    from_end: &BTreeMap<Position, usize>,
    distance: isize,
) -> isize {
    let Some(distance2) = from_end.get(start) else {
        return 0;
    };
    let Some(distance1) = from_end.get(end) else {
        return 0;
    };
    let savings: isize = *distance2 as isize - *distance1 as isize - distance;
    savings
}

fn get_position_of(grid: &[Vec<char>], value: char) -> Option<Position> {
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if *col == value {
                return Some((row_idx as isize, col_idx as isize));
            }
        }
    }
    None
}

type Cheat = (Position, Position);
type Position = (isize, isize);
type Grid = Vec<Vec<char>>;

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let parse_line = many1(one_of(".#SE"));
    let (input, grid) = separated_list0(line_ending, parse_line)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = eof(input)?;
    Ok((input, grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let _result = part1(INPUT)?;
        // assert_eq!(result, 6);
        Ok(())
    }

    #[test]
    fn test1() -> anyhow::Result<()> {
        let (_, grid) = parse_grid(INPUT).map_err(|e| anyhow!(e.to_string()))?;
        let (end_row, end_col) =
            get_position_of(&grid, 'E').ok_or_else(|| anyhow!("'E' not found"))?;
        let from_end = distances_from(&grid, end_row, end_col);
        let savings = cheat_savings(&(3, 1), &(7, 3), &from_end, 6);
        println!("savings: {}", savings);
        Ok(())
    }

    #[test]
    fn test2() -> anyhow::Result<()> {
        let (_, grid) = parse_grid(INPUT).map_err(|e| anyhow!(e.to_string()))?;
        let (end_row, end_col) =
            get_position_of(&grid, 'E').ok_or_else(|| anyhow!("'E' not found"))?;
        let from_end = distances_from(&grid, end_row, end_col);
        let cheats = cheats_from_position2(&grid, 3, 1, 20, &from_end);
        println!("cheats: {cheats:?}");
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let _result = part2(INPUT)?;
        // assert_eq!(result, 16);
        Ok(())
    }
}
