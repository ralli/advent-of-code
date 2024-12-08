use anyhow::Context;
use std::collections::BTreeSet;
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Antenna {
    row: isize,
    col: isize,
    freq: char,
}

#[derive(Debug, Clone)]
struct Grid {
    antennas: Vec<Antenna>,
    width: isize,
    height: isize,
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let grid = parse_grid(input)?;
    let freqs: BTreeSet<char> = grid.antennas.iter().map(|a| a.freq).collect();
    let mut positions: BTreeSet<(isize, isize)> = BTreeSet::new();
    for freq in freqs.iter() {
        let freq_antennas: Vec<_> = grid.antennas.iter().filter(|a| a.freq == *freq).collect();
        for (i, a1) in freq_antennas.iter().enumerate() {
            for (j, a2) in freq_antennas.iter().enumerate() {
                if i != j {
                    let dr = a2.row - a1.row;
                    let dc = a2.col - a1.col;
                    let r = a1.row + dr * 2;
                    let c = a1.col + dc * 2;
                    if 0 <= r && grid.height > r && 0 <= c && grid.width > c {
                        positions.insert((r, c));
                        // println!("{:?}", (r, c, freq));
                    }
                }
            }
        }
    }
    Ok(positions.len())
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let grid = parse_grid(input)?;
    let freqs: BTreeSet<char> = grid.antennas.iter().map(|a| a.freq).collect();
    let mut positions: BTreeSet<(isize, isize)> = BTreeSet::new();
    for freq in freqs.iter() {
        let freq_antennas: Vec<_> = grid.antennas.iter().filter(|a| a.freq == *freq).collect();
        for (i, a1) in freq_antennas.iter().enumerate() {
            for (j, a2) in freq_antennas.iter().enumerate() {
                if i != j {
                    let dr = a2.row - a1.row;
                    let dc = a2.col - a1.col;
                    let mut r = a1.row + dr;
                    let mut c = a1.col + dc;
                    while 0 <= r && grid.height > r && 0 <= c && grid.width > c {
                        positions.insert((r, c));
                        r += dr;
                        c += dc;
                        // println!("{:?}", (r, c, freq));
                    }
                }
            }
        }
    }
    Ok(positions.len())
}

fn parse_grid(input: &str) -> anyhow::Result<Grid> {
    let mut antennas = Vec::new();
    let mut height = 0;
    let mut width = 0;
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                antennas.push(Antenna {
                    row: row as isize,
                    col: col as isize,
                    freq: c,
                })
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
