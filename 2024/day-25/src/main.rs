use anyhow::anyhow;
use nom::character::complete::{line_ending, one_of};
use nom::multi::{many1, separated_list0, separated_list1};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("day-25/input.txt")?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let (_, grids) = parse_grids(input).map_err(|e| anyhow!("{e}"))?;
    let height = grids.first().map(|g| g.len()).unwrap_or_default();
    // let width = grids.first().map(|g| g.first().map(|r| r.len()).unwrap_or_default()).unwrap_or_default();
    let (locks, keys): (Vec<Grid>, Vec<Grid>) = grids.into_iter().partition(is_lock);
    let lock_heights: Vec<_> = locks.iter().map(|g| get_lock_heights(g, '#')).collect();
    let key_heights: Vec<_> = keys.iter().map(|g| get_key_heights(g, '#')).collect();
    let mut result = 0;
    for lh in lock_heights.iter() {
        for kh in key_heights.iter() {
            if matches(lh, kh, height-1) {
                result += 1;
            }
        }
    }
    Ok(result)
}

fn part2(_input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

fn matches(lock_heights: &[usize], key_heights: &[usize], height: usize) -> bool {
    lock_heights
        .iter()
        .zip(key_heights.iter())
        .all(|(lock_height, key_height)| lock_height + key_height < height)
}

fn get_lock_heights(g: &Grid, m: char) -> Vec<usize> {
    let width = g.first().map(|r| r.len()).unwrap_or_default();
    (0..width)
        .map(|col| get_lock_height_for_col(g, m, col))
        .collect()
}

fn get_lock_height_for_col(g: &Grid, m: char, col: usize) -> usize {
    (1..g.len())
        .map(|r| g[r].as_slice())
        .take_while(|row| row[col] == m)
        .count()
}

fn get_key_heights(g: &Grid, m: char) -> Vec<usize> {
    let width = g.first().map(|r| r.len()).unwrap_or_default();
    (0..width)
        .map(|col| get_key_height_for_col(g, m, col))
        .collect()
}

fn get_key_height_for_col(g: &Grid, m: char, col: usize) -> usize {
    let height = g.len();
    (0..height)
        .rev()
        .map(|r| g[r].as_slice())
        .take_while(|row| row[col] == m)
        .count()
        - 1
}

fn is_lock(grid: &Grid) -> bool {
    grid.first()
        .map(|f| f.first().map(|f| *f == '#').unwrap_or(false))
        .unwrap_or(false)
}

type Grid = Vec<Vec<char>>;

fn parse_grids(input: &str) -> IResult<&str, Vec<Grid>> {
    separated_list0(many1(line_ending), parse_grid)(input)
}

fn parse_grid(input: &str) -> IResult<&str, Grid> {
    separated_list1(line_ending, parse_grid_line)(input)
}

fn parse_grid_line(input: &str) -> IResult<&str, Vec<char>> {
    many1(one_of(".#"))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        assert_eq!(result, 3);
        Ok(())
    }
}
