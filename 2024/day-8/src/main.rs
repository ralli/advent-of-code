use anyhow::Context;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;

fn main() -> anyhow::Result<()> {
    let filename = "day-8/input.txt";
    let content = fs::read_to_string(filename).context(format!("cannot load {filename}"))?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

#[derive(Debug, Clone)]
struct Grid {
    antennas: BTreeMap<char, Vec<(isize, isize)>>,
    width: isize,
    height: isize,
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let grid = parse_grid(input)?;
    let mut positions: BTreeSet<(isize, isize)> = BTreeSet::new();
    for freq_antennas in grid.antennas.values() {
        for (i, (row1, col1)) in freq_antennas.iter().enumerate() {
            for (j, (row2, col2)) in freq_antennas.iter().enumerate() {
                if i != j {
                    let dr = row2 - row1;
                    let dc = col2 - col1;
                    let r = row1 + dr * 2;
                    let c = col1 + dc * 2;
                    if 0 <= r && grid.height > r && 0 <= c && grid.width > c {
                        positions.insert((r, c));
                    }
                }
            }
        }
    }
    Ok(positions.len())
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let grid = parse_grid(input)?;
    let mut positions: BTreeSet<(isize, isize)> = BTreeSet::new();
    for freq_antennas in grid.antennas.values() {
        for (i, (row1, col1)) in freq_antennas.iter().enumerate() {
            for (j, (row2, col2)) in freq_antennas.iter().enumerate() {
                if i != j {
                    let dr = row2 - row1;
                    let dc = col2 - col1;
                    let mut r = row1 + dr;
                    let mut c = col1 + dc;
                    while 0 <= r && grid.height > r && 0 <= c && grid.width > c {
                        positions.insert((r, c));
                        r += dr;
                        c += dc;
                    }
                }
            }
        }
    }
    Ok(positions.len())
}

fn parse_grid(input: &str) -> anyhow::Result<Grid> {
    let mut antennas: BTreeMap<char, Vec<(isize, isize)>> = BTreeMap::new();
    let mut height = 0;
    let mut width = 0;
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                let e = antennas.entry(c).or_default();
                e.push((row as isize, col as isize));
            }
        }
        if line.chars().any(|c| c == '.' || c.is_alphanumeric()) {
            height += 1;
            width = width.max(line.len() as isize);
        }
    }

    Ok(Grid {
        antennas,
        width,
        height,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    const INPUT2: &str = r#"T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
.........."#;

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        assert_eq!(result, 14);
        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let result = part2(INPUT2)?;
        assert_eq!(result, 9);
        Ok(())
    }

    #[test]
    fn test_part2_1() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        assert_eq!(result, 34);
        Ok(())
    }
}
