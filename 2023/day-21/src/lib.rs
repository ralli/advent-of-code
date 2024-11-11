use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

pub fn part1(input: &str) -> anyhow::Result<i64> {
    let grid: Grid = input.parse()?;
    let result = num_tiles(&grid, 64);
    Ok(result)
}

pub fn part2(input: &str) -> anyhow::Result<i64> {
    Ok(0)
}

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn start_pos(&self) -> Option<(usize, usize)> {
        self.cells.iter()
            .enumerate()
            .find_map(|(row, line)|
                line
                    .iter()
                    .enumerate()
                    .find(|(_, ch)| **ch == 'S').map(|(col, _)| (row, col)))
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells: Vec<Vec<char>> = s.lines().filter(|s| !s.is_empty()).map(|line| line.chars().collect()).collect();
        let height = cells.len();
        let width = cells.first().map(|line| line.len()).unwrap_or_default();
        Ok(Grid { cells, width, height })
    }
}

fn num_tiles(grid: &Grid, max_steps: i32) -> i64 {
    let (start_row, start_col) = grid.start_pos().unwrap();
    let mut points = vec![(start_row, start_col)];
    let mut grid = grid.clone();
    grid.cells[start_row][start_col] = '.';

    for round in 0..max_steps {
        let mut next_points: Vec<(usize, usize)> = Vec::new();

        for &(row, col) in points.iter() {
            if row > 0 && grid.cells[row - 1][col] == '.' {
                grid.cells[row - 1][col] = 'O';
                next_points.push((row - 1, col));
            }

            if row + 1 < grid.height && grid.cells[row + 1][col] == '.' {
                grid.cells[row + 1][col] = 'O';
                next_points.push((row + 1, col));
            }

            if col > 0 && grid.cells[row][col - 1] == '.' {
                grid.cells[row][col - 1] = 'O';
                next_points.push((row, col - 1));
            }

            if col + 1 < grid.width && grid.cells[row][col + 1] == '.' {
                grid.cells[row][col + 1] = 'O';
                next_points.push((row, col + 1));
            }
        }

        points = next_points;
        println!("{} {}", round, points.len());

    }

    points.len() as i64
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in self.cells.iter() {
            for c in line {
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

    static INPUT: &str = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 16;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test1() -> anyhow::Result<()> {
        let grid: Grid = INPUT.parse()?;
        let result = num_tiles(&grid, 6);
        let expected = 16;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 167409079868000;
        assert_eq!(result, expected);
        Ok(())
    }
}
