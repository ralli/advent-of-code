use std::fs::File;
use std::io::Read;

use anyhow::Context;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::sequence::separated_pair;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let filename = "./day-17/input.txt";
    let content = read_file(filename).context(filename)?;

    let result = part1(&content);
    println!("{}", result);

    let result = part2(&content);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> i32 {
    let (_, bounds) = input_data(input).unwrap();

    (1..=bounds.xmax)
        .cartesian_product(1..bounds.ymin.abs())
        .filter_map(|(dx, dy)| simulate(dx, dy, &bounds))
        .max()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let (_, bounds) = input_data(input).unwrap();

    (1..=bounds.xmax)
        .cartesian_product(bounds.ymin..bounds.ymin.abs())
        .filter_map(|(dx, dy)| simulate(dx, dy, &bounds))
        .count()
}

fn simulate(initial_dx: i32, initial_dy: i32, bounds: &Bounds) -> Option<i32> {
    let mut x = 0;
    let mut y = 0;
    let mut dx = initial_dx;
    let mut dy = initial_dy;
    let mut max_y = i32::MIN;

    while check_not_out_of_bounds(x, y, bounds) {
        x += dx;
        y += dy;
        max_y = y.max(max_y);
        if check_in_bounds(x, y, bounds) {
            return Some(max_y);
        }
        // dx > 0 => dx -= 1
        dx -= dx.signum();
        dy -= 1;
    }

    None
}

fn check_not_out_of_bounds(x: i32, y: i32, bounds: &Bounds) -> bool {
    x <= bounds.xmax && y >= bounds.ymin
}

fn check_in_bounds(x: i32, y: i32, bounds: &Bounds) -> bool {
    (bounds.xmin..=bounds.xmax).contains(&x) && (bounds.ymin..=bounds.ymax).contains(&y)
}

#[derive(Debug)]
struct Bounds {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

fn input_data(input: &str) -> IResult<&str, Bounds> {
    let (input, _) = tag("target area: x=")(input)?;
    let (input, (xmin, xmax)) = separated_pair(complete::i32, tag(".."), complete::i32)(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, (ymin, ymax)) = separated_pair(complete::i32, tag(".."), complete::i32)(input)?;

    Ok((
        input,
        Bounds {
            xmin,
            xmax,
            ymin,
            ymax,
        },
    ))
}
fn read_file(name: &str) -> anyhow::Result<String> {
    let mut f = File::open(name)?;
    let mut result = String::new();
    f.read_to_string(&mut result)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 45;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 112;
        assert_eq!(result, expected);
    }
}
