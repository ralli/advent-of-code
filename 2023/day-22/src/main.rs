use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt::Formatter;
use std::{fmt, fs};

use anyhow::{anyhow, Context};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, multispace0};
use nom::combinator::all_consuming;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let filename = "day-22.txt";
    let input = fs::read_to_string(filename).with_context(|| anyhow!("cannot load {filename}"))?;

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i64> {
    let (_, mut bricks) = all_consuming(terminated(
        separated_list1(line_ending, parse_brick),
        multispace0,
    ))(input)
    .map_err(|e| anyhow!(e.to_string()))?;

    assert!(bricks
        .iter()
        .all(|b| b.p1.x <= b.p2.x && b.p1.y <= b.p2.y && b.p1.z <= b.p2.z));

    bricks.sort_by(|a, b| a.p1.z.cmp(&b.p1.z));

    for i in 0..bricks.len() {
        let a = &bricks[i];
        let z2 = bricks
            .iter()
            .enumerate()
            .filter(|(j, b)| i != *j && a.p1.z > b.p2.z && a.overlaps(b))
            .map(|(_, b)| b.p2.z)
            .max()
            .unwrap_or_default();
        let delta = a.p1.z - z2 - 1;
        let a = &mut bricks[i];
        a.p1.z -= delta;
        a.p2.z -= delta;
        assert!(a.p1.z > 0);
        assert!(a.p2.z > 0);
    }

    let depends_on: BTreeMap<&Brick, Vec<&Brick>> = (0..bricks.len())
        .map(|i| {
            let a = &bricks[i];
            let depends: Vec<&Brick> = bricks
                .iter()
                .filter(|b| a.p1.z == b.p2.z + 1 && a.overlaps(b))
                .collect();
            (a, depends)
        })
        .collect();

    let required: BTreeSet<&Brick> = depends_on
        .values()
        .filter_map(|depends| {
            if depends.len() == 1 {
                depends.first()
            } else {
                None
            }
        })
        .copied()
        .collect();

    let result = bricks
        .iter()
        .filter(|brick| !required.contains(brick))
        .count();

    Ok(result as i64)
}

fn part2(input: &str) -> anyhow::Result<i64> {
    let (_, mut bricks) = all_consuming(terminated(
        separated_list1(line_ending, parse_brick),
        multispace0,
    ))(input)
    .map_err(|e| anyhow!(e.to_string()))?;

    bricks.sort_by(|a, b| a.p1.z.cmp(&b.p1.z));

    for i in 0..bricks.len() {
        let a = &bricks[i];
        let z2 = bricks
            .iter()
            .enumerate()
            .filter(|(j, b)| i != *j && a.p1.z > b.p2.z && a.overlaps(b))
            .map(|(_, b)| b.p2.z)
            .max()
            .unwrap_or_default();
        let delta = a.p1.z - z2 - 1;
        let a = &mut bricks[i];
        a.p1.z -= delta;
        a.p2.z -= delta;
        assert!(a.p1.z > 0);
        assert!(a.p2.z > 0);
    }

    let mut depends_on: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    let mut supports: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();

    for (i, a) in bricks.iter().enumerate() {
        for (j, b) in bricks.iter().enumerate() {
            if i != j && a.p1.z == b.p2.z + 1 && a.overlaps(b) {
                depends_on.entry(i).or_default().insert(j);
                supports.entry(j).or_default().insert(i);
            }
        }
    }

    let starts: BTreeSet<usize> = depends_on
        .values()
        .filter_map(|depends| {
            if depends.len() == 1 {
                depends.first()
            } else {
                None
            }
        })
        .copied()
        .collect();

    let result: i64 = starts
        .iter()
        .map(|start| count_bricks(&depends_on, &supports, *start))
        .sum();
    Ok(result)
}

fn count_bricks(
    depends_on: &BTreeMap<usize, BTreeSet<usize>>,
    supports: &BTreeMap<usize, BTreeSet<usize>>,
    start: usize,
) -> i64 {
    let mut q = VecDeque::from([start]);
    let mut visited = BTreeSet::new();

    while let Some(current) = q.pop_front() {
        visited.insert(current);
        if let Some(edges) = supports.get(&current) {
            for next in edges.iter() {
                // next will fall if all bricks it depends on have already fallen
                if let Some(deps) = depends_on.get(next) {
                    if deps.is_subset(&visited) {
                        q.push_back(*next);
                    }
                }
            }
        }
    }

    (visited.len() - 1) as i64
}

fn parse_brick(input: &str) -> IResult<&str, Brick> {
    let (input, p1) = parse_point(input)?;
    let (input, _) = tag("~")(input)?;
    let (input, p2) = parse_point(input)?;
    Ok((input, Brick { p1, p2 }))
}
fn parse_point(input: &str) -> IResult<&str, Point3> {
    let (input, x) = complete::i64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = complete::i64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, z) = complete::i64(input)?;
    Ok((input, Point3 { x, y, z }))
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
struct Brick {
    p1: Point3,
    p2: Point3,
}

impl Brick {
    fn overlaps(&self, other: &Brick) -> bool {
        self.p1.x <= other.p2.x
            && other.p1.x <= self.p2.x
            && self.p1.y <= other.p2.y
            && other.p1.y <= self.p2.y
    }
}

impl fmt::Display for Brick {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{},{},{}~{},{},{}",
            self.p1.x, self.p1.y, self.p1.z, self.p2.x, self.p2.y, self.p2.z
        )
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
struct Point3 {
    x: i64,
    y: i64,
    z: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 5;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 7;
        assert_eq!(result, expected);
        Ok(())
    }
}
