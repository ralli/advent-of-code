use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Formatter;

use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, tuple};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let input = read_file("./day-14/input.txt")?;
    let result = part1(&input);

    println!("{}", result);

    let result = part2(&input);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> usize {
    let (_, line_segments) = line_segments(input).unwrap();
    let mut grid = Grid::new(&line_segments);

    println!("{}", &grid);

    let mut count = 0;
    while grid.drop_sand() {
        count += 1;
    }
    println!("Step: {}\n\n{}", count, &grid);
    count
}

fn part2(input: &str) -> usize {
    let (_, line_segments) = line_segments(input).unwrap();
    let mut grid = Grid::new(&line_segments);

    let mut count = 0;
    while grid.drop_sand2() {
        count += 1;
        // println!("Step: {}\n\n{}", count, &grid);
    }
    println!("Step: {}\n\n{}", count + 1, &grid);
    count + 1
}

#[derive(Debug, Copy, Clone)]
enum Material {
    Air,
    Sand,
    Rock,
    Source,
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Material::*;

        let c = match self {
            Air => '.',
            Sand => 'o',
            Rock => '#',
            Source => '+',
        };

        write!(f, "{}", c)
    }
}

impl Material {
    fn is_solid(&self) -> bool {
        !matches!(self, Material::Air)
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::Air
    }
}

#[derive(Debug)]
struct Grid {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    elements: BTreeMap<(i32, i32), Material>,
}

impl Grid {
    fn new(line_segments: &[LineSegment]) -> Self {
        let min_points: Vec<_> = line_segments.iter().map(|p| p.min()).collect();
        let max_points: Vec<_> = line_segments.iter().map(|p| p.max()).collect();

        let x1 = min_points.iter().map(|(x, _)| x).min().copied().unwrap();
        let y1 = min_points
            .iter()
            .map(|(_, y)| y)
            .min()
            .copied()
            .unwrap()
            .min(0);
        let x2 = max_points.iter().map(|(x, _)| x).max().copied().unwrap();
        let y2 = max_points.iter().map(|(_, y)| y).max().copied().unwrap();

        let mut elements = BTreeMap::new();
        for line_segment in line_segments.iter() {
            for w in line_segment.points.windows(2) {
                let (x1, y1) = w[0];
                let (x2, y2) = w[1];
                if x1 == x2 {
                    for y in y1.min(y2)..=y1.max(y2) {
                        elements.insert((x1, y), Material::Rock);
                    }
                } else {
                    for x in x1.min(x2)..=x1.max(x2) {
                        elements.insert((x, y1), Material::Rock);
                    }
                }
            }
        }
        elements.insert((500, 0), Material::Source);
        Grid {
            x1,
            y1,
            x2,
            y2,
            elements,
        }
    }

    fn get(&self, x: i32, y: i32) -> Material {
        self.elements.get(&(x, y)).copied().unwrap_or_default()
    }

    fn get2(&self, x: i32, y: i32) -> Material {
        if y == self.y2 + 2 {
            return Material::Rock;
        }
        self.elements.get(&(x, y)).copied().unwrap_or_default()
    }

    fn drop_sand(&mut self) -> bool {
        let mut x = 500;
        let mut y = 0;

        while let Some((next_x, next_y)) = self.next_pos(x, y) {
            x = next_x;
            y = next_y;
        }

        self.elements.insert((x, y), Material::Sand);
        y < self.y2
    }

    fn drop_sand2(&mut self) -> bool {
        let mut x = 500;
        let mut y = 0;

        while let Some((next_x, next_y)) = self.next_pos2(x, y) {
            x = next_x;
            y = next_y;
        }

        self.elements.insert((x, y), Material::Sand);

        !(y == 0 && x == 500)
    }

    fn next_pos(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        if y >= self.y2 {
            return None;
        }

        if !self.get(x, y + 1).is_solid() {
            return Some((x, y + 1));
        }

        if !self.get(x - 1, y + 1).is_solid() {
            return Some((x - 1, y + 1));
        }

        if !self.get(x + 1, y + 1).is_solid() {
            return Some((x + 1, y + 1));
        }

        None
    }

    fn next_pos2(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        if !self.get2(x, y + 1).is_solid() {
            return Some((x, y + 1));
        }

        if !self.get2(x - 1, y + 1).is_solid() {
            return Some((x - 1, y + 1));
        }

        if !self.get2(x + 1, y + 1).is_solid() {
            return Some((x + 1, y + 1));
        }

        None
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let x1 = self.x1;
        let y1 = self.y1;
        let x2 = self.x2;
        let y2 = self.y2;

        for y in y1..=y2 {
            for x in x1..=x2 {
                write!(f, "{}", self.get(x, y))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct LineSegment {
    points: Vec<(i32, i32)>,
}

impl LineSegment {
    fn min(&self) -> (i32, i32) {
        let x = self.points.iter().map(|p| p.0).min().unwrap();
        let y = self.points.iter().map(|p| p.1).min().unwrap();
        (x, y)
    }
    fn max(&self) -> (i32, i32) {
        let x = self.points.iter().map(|p| p.0).max().unwrap();
        let y = self.points.iter().map(|p| p.1).max().unwrap();
        (x, y)
    }
}

fn line_segments(input: &str) -> IResult<&str, Vec<LineSegment>> {
    separated_list1(line_ending, line_segment)(input)
}

fn line_segment(input: &str) -> IResult<&str, LineSegment> {
    map(
        separated_list1(tuple((space1, tag("->"), space1)), point),
        |points| LineSegment { points },
    )(input)
}

fn point(input: &str) -> IResult<&str, (i32, i32)> {
    use nom::character::complete::i32 as i32_parser;
    separated_pair(i32_parser, tag(","), i32_parser)(input)
}

fn read_file(filename: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 24;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 93;
        assert_eq!(result, expected);
    }
}
