use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use anyhow::anyhow;
use anyhow::Context;
use nom::sequence::delimited;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace0, space0},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

fn main() -> anyhow::Result<()> {
    let filename = "day-24.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i64> {
    let (_, hailstones) = parse_state(input).map_err(|e| anyhow!(e.to_string()))?;
    let minpos = 200000000000000.0;
    let maxpos = 400000000000000.0;

    let result = count_possible_intersections(&hailstones, minpos, maxpos);

    Ok(result)
}

//
// Did not manage to find a solution on my own. So i implemented one, i found most funny.
//
// This approach finds the correct answer by guessing the velocity vcector and the starting position.
//
// Finding the velocity vector is done by brute force over some interval checking if the coordinate differences are
// dividable without remainder.
//
// finding the starting point is done by calculating the x and y coordinates and estimating the z coordinate.
// since this yields many different results (either because of bad timing or rounding errors and the like) the correct
// results will be the one with the most hits over all hailstones :-)
//
// more or less copied this solution: https://github.com/ayoubzulfiqar/advent-of-code/tree/main/Go/Day24
//
// hyperneutrinos solution is more elegant and deterministic by solving an equation system using a computer algebra library:
// https://github.com/hyper-neutrino/advent-of-code/blob/main/2023/day24p2.py
//
fn part2(input: &str) -> anyhow::Result<i64> {
    let (_, hailstones) = parse_state(input).map_err(|e| anyhow!(e.to_string()))?;
    let vxx: BTreeMap<i64, Vec<i64>> = hailstones.iter().fold(BTreeMap::new(), |mut m, h| {
        let e = m.entry(h.vel.x).or_default();
        e.push(h.pos.x);
        m
    });
    let vyy: BTreeMap<i64, Vec<i64>> = hailstones.iter().fold(BTreeMap::new(), |mut m, h| {
        let e = m.entry(h.vel.y).or_default();
        e.push(h.pos.y);
        m
    });
    let vzz: BTreeMap<i64, Vec<i64>> = hailstones.iter().fold(BTreeMap::new(), |mut m, h| {
        let e = m.entry(h.vel.z).or_default();
        e.push(h.pos.z);
        m
    });

    //
    // guessing the velocity vector:
    //   the vector is built from integer numbers, so it is relatively easy to guess
    // just go from an interval (lets say -1000..1000) for x, y, z and choose the x, y, z that divides all
    // points in x1, x2 for a given vx without remainder. Doing this for many points yields a single answer if you
    // have enough points to check.
    //
    let rvx = get_rock_velocity(&vxx);
    let rvy = get_rock_velocity(&vyy);
    let rvz = get_rock_velocity(&vzz);

    // to find the starting point:
    //
    // calculate the x and y coordinate of the starting vector and
    // estimate the z coordinate from the x velocity.
    //
    // this will will yield many different results. The correct one will be hit by far most...
    //
    // build a histogram and choose the result hit most.
    //
    let mut results: BTreeMap<i64, i64> = BTreeMap::new();
    for stone_a in hailstones.iter() {
        let div = stone_a.vel.x - rvx;
        if div == 0 {
            continue;
        }
        for stone_b in hailstones.iter() {
            let ma = (stone_a.vel.y - rvy) as f64 / (stone_a.vel.x - rvx) as f64;
            let mb = (stone_b.vel.y - rvy) as f64 / (stone_b.vel.x - rvx) as f64;

            let ca = (stone_a.pos.y as f64) - ma * (stone_a.pos.x as f64);
            let cb = (stone_b.pos.y as f64) - mb * (stone_b.pos.x as f64);

            let rpx = ((cb - ca) / (ma - mb)) as i64;
            let rpy = (ma * (rpx as f64) + ca) as i64;

            let time = (rpx - stone_a.pos.x) / div;
            let rpz = stone_a.pos.z + (stone_a.vel.z - rvz) * time;

            let result = rpx + rpy + rpz;
            let e = results.entry(result).or_default();
            *e += 1;
        }
    }
    // println!("{results:?}");
    let result = results
        .keys()
        .max_by(|a, b| results.get(a).unwrap().cmp(results.get(b).unwrap()))
        .copied()
        .unwrap();
    Ok(result)
}

fn get_rock_velocity(velocities: &BTreeMap<i64, Vec<i64>>) -> i64 {
    let mut possibilities: Vec<i64> = (-1000..1000).collect();
    for (vel, values) in velocities.iter() {
        if values.len() < 2 {
            continue;
        }
        possibilities.retain(|possible| {
            (*possible - vel) != 0 && (values[0] - values[1]).rem_euclid(*possible - vel) == 0
        });
    }
    possibilities.first().copied().unwrap()
}

fn count_possible_intersections(hailstones: &[Hailstone], minpos: f64, maxpos: f64) -> i64 {
    let result: usize = hailstones
        .iter()
        .enumerate()
        .map(|(i, a)| {
            hailstones[i + 1..]
                .iter()
                .filter_map(|b| a.intersection_with(b).map(|s| (b, s)))
                .filter(|&(_b, (x, y))| minpos <= x && maxpos >= x && minpos <= y && maxpos >= y)
                .count()
        })
        .sum();

    result as i64
}

#[derive(Debug)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct Hailstone {
    pos: Vec3,
    vel: Vec3,
}

impl Hailstone {
    fn intersection_with(&self, other: &Hailstone) -> Option<(f64, f64)> {
        let x1 = self.pos.x as f64;
        let y1 = self.pos.y as f64;
        let xv1 = self.vel.x as f64;
        let yv1 = self.vel.y as f64;
        let x2 = other.pos.x as f64;
        let y2 = other.pos.y as f64;
        let xv2 = other.vel.x as f64;
        let yv2 = other.vel.y as f64;

        let a1 = yv1;
        let b1 = -xv1;
        let c1 = yv1 * x1 - xv1 * y1;

        let a2 = yv2;
        let b2 = -xv2;
        let c2 = yv2 * x2 - xv2 * y2;

        if (a1 * b2 - b1 * a2).abs() < 1e-5 {
            return None;
        }

        let x = (c1 * b2 - c2 * b1) / (a1 * b2 - a2 * b1);
        let y = (c2 * a1 - c1 * a2) / (a1 * b2 - a2 * b1);

        if (x - x1) * xv1 >= 0.0
            && (y - y1) * yv1 >= 0.0
            && (x - x2) * xv2 >= 0.0
            && (y - y2) * yv2 >= 0.0
        {
            Some((x, y))
        } else {
            None
        }
    }
}

fn parse_state(input: &str) -> IResult<&str, Vec<Hailstone>> {
    all_consuming(terminated(
        separated_list1(line_ending, parse_hailstone),
        multispace0,
    ))(input)
}

fn parse_hailstone(input: &str) -> IResult<&str, Hailstone> {
    let (input, pos) = parse_vec3(input)?;
    let (input, _) = delimited(space0, tag("@"), space0)(input)?;
    let (input, dir) = parse_vec3(input)?;

    Ok((input, Hailstone { pos, vel: dir }))
}

fn parse_vec3(input: &str) -> IResult<&str, Vec3> {
    let (input, x) = complete::i64(input)?;
    let (input, _) = terminated(tag(","), space0)(input)?;
    let (input, y) = complete::i64(input)?;
    let (input, _) = terminated(tag(","), space0)(input)?;
    let (input, z) = complete::i64(input)?;

    Ok((input, Vec3 { x, y, z }))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let (_, hailstones) = parse_state(INPUT).map_err(|e| anyhow!(e.to_string()))?;
        let minpos = 7.0;
        let maxpos = 27.0;
        let result = count_possible_intersections(&hailstones, minpos, maxpos);
        let expected = 2;
        assert_eq!(result, expected);
        Ok(())
    }

    /*  #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 47;
        assert_eq!(result, expected);
        Ok(())
    }*/
}
