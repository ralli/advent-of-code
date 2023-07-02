use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char, line_ending};
use nom::multi::{many1, separated_list1};
use nom::IResult;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::fmt::Formatter;
use std::ops::RangeInclusive;

fn main() -> anyhow::Result<()> {
    let input = read_file("./day-23/input.txt")?;

    let result = part1(&input);
    println!("{}", result);

    let result = part2(&input);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> usize {
    let (_, mut board) = board(input).unwrap();

    for _step in 0..10 {
        board.step();
    }

    board.num_empty_fields()
}

fn part2(input: &str) -> i64 {
    let (_, mut board) = board(input).unwrap();
    let mut result = 0;

    loop {
        result += 1;
        if board.step() == 0 {
            break;
        }
    }

    result
}

fn read_file(filename: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}

type Point = (i64, i64);
struct Board {
    elves: HashSet<Point>,
    directions: VecDeque<MoveDirection>,
}

const DIRECTIONS: [(i64, i64); 8] = [
    (-1, 1),
    (0, 1),
    (1, 1),
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
];

#[derive(Debug, Copy, Clone)]
enum MoveDirection {
    North,
    East,
    South,
    West,
}

impl Board {
    fn step(&mut self) -> usize {
        let initial: HashSet<Point> = self
            .elves
            .iter()
            .filter(|(x, y)| {
                !DIRECTIONS
                    .iter()
                    .all(|(dx, dy)| !self.elves.contains(&(x + dx, y + dy)))
            })
            .copied()
            .collect();

        let mut moves = HashMap::new();

        for direction in self.directions.iter() {
            match direction {
                MoveDirection::North => {
                    for &(x, y) in initial.iter() {
                        if !self.elves.contains(&(x - 1, y + 1))
                            && !self.elves.contains(&(x, y + 1))
                            && !self.elves.contains(&(x + 1, y + 1))
                        {
                            moves.entry((x, y)).or_insert((x, y + 1));
                        }
                    }
                }
                MoveDirection::South => {
                    for &(x, y) in initial.iter() {
                        if !self.elves.contains(&(x - 1, y - 1))
                            && !self.elves.contains(&(x, y - 1))
                            && !self.elves.contains(&(x + 1, y - 1))
                        {
                            moves.entry((x, y)).or_insert((x, y - 1));
                        }
                    }
                }
                MoveDirection::West => {
                    for &(x, y) in initial.iter() {
                        if !self.elves.contains(&(x - 1, y - 1))
                            && !self.elves.contains(&(x - 1, y))
                            && !self.elves.contains(&(x - 1, y + 1))
                        {
                            moves.entry((x, y)).or_insert((x - 1, y));
                        }
                    }
                }
                MoveDirection::East => {
                    for &(x, y) in initial.iter() {
                        if !self.elves.contains(&(x + 1, y - 1))
                            && !self.elves.contains(&(x + 1, y))
                            && !self.elves.contains(&(x + 1, y + 1))
                        {
                            moves.entry((x, y)).or_insert((x + 1, y));
                        }
                    }
                }
            }
        }

        self.directions.rotate_left(1);

        let mut hist = HashMap::new();
        for p in moves.values() {
            let entry = hist.entry(*p).or_insert(0);
            *entry += 1;
        }

        let mut n = 0;
        for (p1, p2) in moves.iter() {
            let count = hist.get(p2).copied().unwrap();
            if count == 1 {
                n += 1;
                self.elves.remove(p1);
                self.elves.insert(*p2);
            }
        }

        n
    }

    fn num_empty_fields(&self) -> usize {
        self.xrange()
            .cartesian_product(self.yrange())
            .filter(|p| !self.elves.contains(p))
            .count()
    }

    fn xrange(&self) -> RangeInclusive<i64> {
        let xmin = self.xmin();
        let xmax = self.xmax();

        xmin..=xmax
    }

    fn yrange(&self) -> RangeInclusive<i64> {
        let ymin = self.ymin();
        let ymax = self.ymax();

        ymin..=ymax
    }

    fn xmin(&self) -> i64 {
        self.elves.iter().map(|(x, _)| x).min().copied().unwrap()
    }

    fn xmax(&self) -> i64 {
        self.elves.iter().map(|(x, _)| x).max().copied().unwrap()
    }

    fn ymin(&self) -> i64 {
        self.elves.iter().map(|(_, y)| y).min().copied().unwrap()
    }

    fn ymax(&self) -> i64 {
        self.elves.iter().map(|(_, y)| y).max().copied().unwrap()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let xmin = self.xmin() - 2;
        let xmax = self.xmax() + 2;
        let yr = (self.ymin() - 2)..=(self.ymax() + 2);

        for y in yr.rev() {
            for x in xmin..=xmax {
                write!(
                    f,
                    "{}",
                    if self.elves.contains(&(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn board(input: &str) -> IResult<&str, Board> {
    let (input, lines) = separated_list1(line_ending, many1(alt((char('.'), char('#')))))(input)?;
    let mut elves = HashSet::new();

    for (y, line) in lines.iter().rev().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == '#' {
                elves.insert((x as i64, y as i64));
            }
        }
    }

    Ok((
        input,
        Board {
            elves,
            directions: VecDeque::from([
                MoveDirection::North,
                MoveDirection::South,
                MoveDirection::West,
                MoveDirection::East,
            ]),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 110;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 20;
        assert_eq!(result, expected);
    }
}
