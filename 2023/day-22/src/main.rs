use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::ops::Deref;
use anyhow::{anyhow, Context};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, multispace0};
use nom::combinator::all_consuming;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::terminated;

fn main() -> anyhow::Result<()> {
    let filename = "day-22.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot read file {}", filename))?;

    let result = part1(&input)?;
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i64> {
    let (_, mut bricks) = parse_bricks(input).map_err(|e| anyhow!(e.to_string()))?;

    assert!(bricks.iter().all(|brick| brick.p1.x <= brick.p2.x && brick.p1.y <= brick.p2.y && brick.p1.z <= brick.p2.z));

    bricks.sort_by(|a, b| a.p1.z.cmp(&b.p1.z));

    for i in (0..bricks.len()).rev() {
        let a = &bricks[i];
        let z2 = bricks.iter().filter(|b| a.p1.z > b.p2.z && a.overlaps(b)).map(|b| b.p2.z).max().unwrap_or_default();
        let delta = a.p1.z - z2;
        assert!(delta > 0);
        bricks[i].p1.z -= delta - 1;
        bricks[i].p2.z -= delta - 1;

        assert_eq!(z2 + 1, bricks[i].p1.z);
    }

    let overlappings = (0..bricks.len()).map(|i| {
        let a = &bricks[i];
        let supporting: Vec<&Brick> = bricks.iter().filter(|b| a.p1.z == b.p2.z + 1).filter(|b| a.overlaps(b)).collect();
        (a, supporting)
    });
    let brick_map = BTreeMap::from_iter(overlappings);

    println!("{:?}", brick_map);

    let required: BTreeSet<&Brick> = brick_map
        .values()
        .filter_map(|supporting| if supporting.len() == 1 { supporting.first() } else { None })
        .copied()
        .collect();
    let result = bricks.iter().filter(|brick| !required.contains(brick)).count();
    Ok(result as i64)
}


fn parse_bricks(input: &str) -> IResult<&str, Vec<Brick>> {
    let (input, mut bricks) = all_consuming(
        terminated(
            separated_list1(line_ending, parse_brick),
            multispace0,
        )
    )(input)?;
    bricks.sort_by(|a, b| a.p1.z.cmp(&b.p1.z));
    Ok((input, bricks))
}

fn parse_brick(input: &str) -> IResult<&str, Brick> {
    let (input, p1) = parse_point3(input)?;
    let (input, _) = tag("~")(input)?;
    let (input, p2) = parse_point3(input)?;
    Ok((input, Brick { p1, p2 }))
}

fn parse_point3(input: &str) -> IResult<&str, Point3> {
    let (input, x) = complete::i64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = complete::i64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, z) = complete::i64(input)?;
    Ok((input, Point3 { x, y, z }))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct Brick {
    p1: Point3,
    p2: Point3,
}

impl Brick {
    fn overlaps(&self, other: &Brick) -> bool {
        self.p2.x >= other.p1.x && self.p1.x <= other.p2.x &&
            self.p2.y >= other.p1.y && self.p1.y <= other.p2.y
    }

    fn move_by(&mut self, z: i64) {
        self.p1.z += z;
        self.p2.z += z;
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
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
}