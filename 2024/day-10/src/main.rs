use std::collections::{BTreeMap, VecDeque};
use std::fmt;
use std::fmt::Formatter;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Grid {
    width: isize,
    height: isize,
    grid: Vec<Vec<i32>>,
}

impl Grid {
    fn get(&self, row: isize, col: isize) -> i32 {
        if row < 0 || row >= self.height || col < 0 || col >= self.width {
            return -1;
        }
        self.grid[row as usize][col as usize]
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let n = self.get(row, col);
                if n < 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "{n}")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

const DIRS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn walk_grid(grid: &Grid) -> BTreeMap<(isize, isize), usize> {
    let mut scores = BTreeMap::new();
    let mut q = VecDeque::new();

    for row in 0..grid.height {
        for col in 0..grid.width {
            if grid.get(row, col) == 0 {
                q.push_back((row, col, 0));
            }
        }
    }
    println!("{:?}", q);
    while let Some((row, col, height)) = q.pop_front() {
        if height == 9 {
            let entry = scores.entry((row, col)).or_default();
            *entry += 1;
        }
        for (dr, dc) in DIRS.iter() {
            let (next_row, next_col) = (row + dr, col + dc);
            if grid.get(next_row, next_col) == height + 1 {
                q.push_back((next_row, next_col, height + 1));
            }
        }
        println!("{:?} {:?}", (row, col, height), q);
    }
    scores
}

fn parse_grid(input: &str) -> Grid {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_digit(10) || *c == '.')
                .map(|c| {
                    if c == '.' {
                        -1
                    } else {
                        c.to_digit(10).unwrap() as i32
                    }
                })
                .collect()
        })
        .collect();
    let width = grid.iter().next().map(|r| r.len()).unwrap_or_default() as isize;
    let height = grid.len() as isize;
    Grid {
        width,
        height,
        grid,
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_grid, walk_grid};

    const INPUT: &str = r#"10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01"#;

    const INPUT2: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
    #[test]
    fn test_parse_grid() {
        let grid = parse_grid(INPUT2);
        println!("{}", grid);
        let scores = walk_grid(&grid);
        println!("{scores:#?}");
    }
}
