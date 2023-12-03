use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Formatter;
use std::{fmt, fs};

use anyhow::Context;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let filename = "day-3.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;
    let result = part1(&input)?;
    println!("{result}");
    let result = part2(&input)?;
    println!("{result}");
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i32> {
    let grid = Grid::new(input);
    let numbers = grid.scan_numbers();
    let result = numbers.iter().sum();
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<i32> {
    let grid = Grid::new(input);
    let numbers = grid.scan_numbers_and_positions();
    let mut stars_to_numbers: BTreeMap<Point, Vec<i32>> = BTreeMap::new();
    for np in numbers.into_iter() {
        for pos in np.positions.into_iter() {
            let e = stars_to_numbers.entry(pos).or_default();
            e.push(np.number);
        }
    }
    let result = stars_to_numbers
        .into_iter()
        .filter(|(_, numbers)| numbers.len() == 2)
        .map(|(_, numbers)| numbers.into_iter().product::<i32>())
        .sum();
    Ok(result)
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    fields: Vec<Vec<char>>,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let fields: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        Grid {
            width: fields
                .iter()
                .next()
                .map(|field| field.len())
                .unwrap_or_default(),
            height: fields.len(),
            fields,
        }
    }

    fn scan_numbers(&self) -> Vec<i32> {
        (0..self.width)
            .cartesian_product(0..self.height)
            .filter_map(|(row, col)| self.scan_number(row as i32, col as i32))
            .collect()
    }

    fn scan_numbers_and_positions(&self) -> Vec<NumberAndPositions> {
        (0..self.width)
            .cartesian_product(0..self.height)
            .filter_map(|(row, col)| self.scan_number_and_positions(row as i32, col as i32))
            .collect()
    }

    fn scan_number(&self, row: i32, col: i32) -> Option<i32> {
        if !self.is_start_of_number(row, col) {
            return None;
        }

        let mut result = 0;
        let mut symbol_found = false;
        let mut c = col;

        while self.is_digit(row, c) {
            result *= 10;
            result += (self.get(row, c) as i32) - '0' as i32;
            symbol_found |= self.has_adjacent_symbol(row, c);
            c += 1;
        }

        if symbol_found {
            Some(result)
        } else {
            None
        }
    }

    fn scan_number_and_positions(&self, row: i32, col: i32) -> Option<NumberAndPositions> {
        if !self.is_start_of_number(row, col) {
            return None;
        }

        let mut result = 0;
        let mut positions = BTreeSet::new();
        let mut c = col;

        while self.is_digit(row, c) {
            result *= 10;
            result += (self.get(row, c) as i32) - '0' as i32;
            positions = positions
                .union(&self.adjacent_stars(row, c))
                .copied()
                .collect();
            c += 1;
        }

        if positions.is_empty() {
            None
        } else {
            Some(NumberAndPositions {
                number: result,
                positions,
            })
        }
    }

    fn get(&self, row: i32, col: i32) -> char {
        if row < 0 || row as usize >= self.height || col < 0 || col as usize >= self.width {
            return '.';
        }
        self.fields[row as usize][col as usize]
    }

    fn has_adjacent_symbol(&self, row: i32, col: i32) -> bool {
        DELTAS
            .iter()
            .any(|(dr, dc)| self.is_symbol(row + dr, col + dc))
    }

    fn adjacent_stars(&self, row: i32, col: i32) -> BTreeSet<Point> {
        let mut result = BTreeSet::new();
        DELTAS.iter().for_each(|(dr, dc)| {
            if self.is_star(row + dr, col + dc) {
                result.insert(Point {
                    row: row + *dr,
                    col: col + *dc,
                });
            }
        });
        result
    }

    fn is_digit(&self, row: i32, col: i32) -> bool {
        self.get(row, col).is_ascii_digit()
    }

    fn is_symbol(&self, row: i32, col: i32) -> bool {
        let c = self.get(row, col);
        !(c == '.' || c.is_ascii_digit())
    }

    fn is_star(&self, row: i32, col: i32) -> bool {
        let c = self.get(row, col);
        c == '*'
    }

    fn is_start_of_number(&self, row: i32, col: i32) -> bool {
        self.is_digit(row, col) && !self.is_digit(row, col - 1)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                write!(f, "{}", self.get(row as i32, col as i32))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
static DELTAS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug)]
struct NumberAndPositions {
    number: i32,
    positions: BTreeSet<Point>,
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Copy, Clone)]
struct Point {
    row: i32,
    col: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 4361;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 467835;
        assert_eq!(result, expected);
        Ok(())
    }
}
