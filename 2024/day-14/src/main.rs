use anyhow::{anyhow, Context};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, space1};
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let filename = "day-14/input.txt";
    let input = std::fs::read_to_string(filename).context(format!("Error reading {filename}"))?;
    let (width, height) = (101, 103);
    let result = part1(&input, width, height)?;
    println!("{result}");

    let result = part2(&input, width, height)?;
    println!("{result}");

    Ok(())
}

type Point = (i64, i64);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Robot {
    p: Point,
    v: Point,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Bathroom {
    width: i64,
    height: i64,
    robots: Vec<Robot>,
}

fn part1(input: &str, width: i64, height: i64) -> anyhow::Result<i64> {
    let steps = 100;
    let (_, bathroom) = parse_bathroom(input, width, height).map_err(|e| anyhow!("{e}"))?;
    let mut quadrants = vec![0; 4];

    for robot in bathroom.robots.iter() {
        let (x, y) = robot.p;
        let (dx, dy) = robot.v;
        let np = (
            (x + dx * steps).rem_euclid(bathroom.width),
            (y + dy * steps).rem_euclid(bathroom.height),
        );
        let quadrant_attempt = point_quadrant(np, width, height);
        if let Some(quadrant) = quadrant_attempt {
            quadrants[quadrant as usize] += 1;
        }
    }

    Ok(quadrants.iter().product::<i64>())
}

fn part2(input: &str, width: i64, height: i64) -> anyhow::Result<i64> {
    let (_, bathroom) = parse_bathroom(input, width, height).map_err(|e| anyhow!("{e}"))?;
    let mut min_sf = usize::MAX;
    let mut min_round = -1;
    let max_steps = width * height;

    for steps in 1..=max_steps {
        let mut grid = vec![false; (width * height) as usize];
        for robot in bathroom.robots.iter() {
            let (x, y) = robot.p;
            let (dx, dy) = robot.v;
            let (x, y) = (
                (x + dx * steps).rem_euclid(bathroom.width),
                (y + dy * steps).rem_euclid(bathroom.height),
            );
            grid[(y * width + x) as usize] = true;
        }
        let sf = grid.iter().filter(|&x| !x).count();
        if sf < min_sf {
            min_sf = sf;
            min_round = steps;
        }
    }
    #[cfg(test)]
    print_state(&bathroom, min_round);
    Ok(min_round)
}

#[cfg(test)]
fn print_state(bathroom: &Bathroom, steps: i64) {
    let mut grid = vec![false; (bathroom.width * bathroom.height) as usize];
    for robot in bathroom.robots.iter() {
        let (x, y) = robot.p;
        let (dx, dy) = robot.v;
        let (x, y) = (
            (x + dx * steps).rem_euclid(bathroom.width),
            (y + dy * steps).rem_euclid(bathroom.height),
        );
        grid[(y * bathroom.width + x) as usize] = true;
    }
    print_grid(&grid, bathroom.width, bathroom.height);
}

#[cfg(test)]
fn print_grid(grid: &[bool], width: i64, height: i64) {
    for y in 0..height {
        for x in 0..width {
            let c = if grid[(y * width + x) as usize] {
                '*'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!();
    }
}
fn parse_bathroom(input: &str, width: i64, height: i64) -> IResult<&str, Bathroom> {
    let (rest, robots) = separated_list0(line_ending, parse_robot)(input)?;
    Ok((
        rest,
        Bathroom {
            width,
            height,
            robots,
        },
    ))
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    let (rest, _) = tag("p=")(input)?;
    let (rest, p) = parse_point(rest)?;
    let (rest, _) = space1(rest)?;
    let (rest, _) = tag("v=")(rest)?;
    let (rest, v) = parse_point(rest)?;
    Ok((rest, Robot { p, v }))
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    separated_pair(complete::i64, complete::char(','), complete::i64)(input)
}

fn point_quadrant(p: Point, width: i64, height: i64) -> Option<i64> {
    let (x, y) = p;
    let x = coordinate_section(x, width);
    let y = coordinate_section(y, height);
    match (x, y) {
        (Some(0), Some(0)) => Some(0),
        (Some(0), Some(1)) => Some(1),
        (Some(1), Some(0)) => Some(2),
        (Some(1), Some(1)) => Some(3),
        _ => None,
    }
}

fn coordinate_section(c: i64, length: i64) -> Option<i64> {
    assert!(length > 0);
    let m = length / 2;
    if c < m {
        Some(0)
    } else if c > m {
        Some(1)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    const INPUT: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    #[test]
    fn test_parse_bathroom() -> anyhow::Result<()> {
        let (rest, bathroom) = parse_bathroom(INPUT, 11, 7).map_err(|e| anyhow!("{e}"))?;
        assert_eq!(rest, "");
        println!("{bathroom:?}");
        Ok(())
    }

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT, 11, 7)?;
        assert_eq!(result, 12);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT, 11, 7)?;
        assert_eq!(result, 1);
        Ok(())
    }

    #[test]
    fn test_coordinate_section() {
        assert_eq!(Some(0), coordinate_section(0, 7));
        assert_eq!(Some(0), coordinate_section(1, 7));
        assert_eq!(Some(0), coordinate_section(2, 7));
        assert!(coordinate_section(3, 7).is_none());
        assert_eq!(Some(1), coordinate_section(4, 7));
        assert_eq!(Some(1), coordinate_section(5, 7));
        assert_eq!(Some(1), coordinate_section(6, 7));

        assert!(coordinate_section(5, 11).is_none());
        assert_eq!(Some(0), coordinate_section(1, 11));
        assert_eq!(Some(1), coordinate_section(6, 11));
        assert_eq!(Some(1), coordinate_section(10, 11));
    }
}
