use std::collections::HashSet;

use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space0};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, tuple};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let input = read_file("./day-15/input.txt")?;
    let result = part1(&input, 2_000_000);

    println!("{}", result);

    let result = part2(&input, 4_000_000);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str, ypos: i32) -> usize {
    let (_, entries) = lines(input).unwrap();
    let beacons: HashSet<_> = entries
        .iter()
        .map(|e| e.beacon_pos)
        .filter(|p| p.1 == ypos)
        .collect();
    let beacon_count = beacons.len();
    let sensors_and_distances: Vec<_> = entries
        .iter()
        .map(|e| {
            let d = manhattan_distance(
                e.sensor_pos.0,
                e.sensor_pos.1,
                e.beacon_pos.0,
                e.beacon_pos.1,
            );
            (e.sensor_pos.0, e.sensor_pos.1, d)
        })
        .filter(|(_x, y, d)| y + d >= ypos && y - d <= ypos)
        .collect();
    let mut marked = HashSet::new();

    for (xs, ys, d) in sensors_and_distances {
        for x in xs - d..=xs + d {
            if manhattan_distance(xs, ys, x, ypos) <= d {
                marked.insert(x);
            }
        }
    }
    marked.len() - beacon_count
}

fn manhattan_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn part2(input: &str, limit: i32) -> usize {
    let (_, entries) = lines(input).unwrap();

    let sensors_and_distances: Vec<_> = entries
        .iter()
        .map(|e| {
            let d = manhattan_distance(
                e.sensor_pos.0,
                e.sensor_pos.1,
                e.beacon_pos.0,
                e.beacon_pos.1,
            );
            (e.sensor_pos.0, e.sensor_pos.1, d)
        })
        .collect();

    let p = (0..=limit)
        .find_map(|y| {
            let ranges = x_ranges(&sensors_and_distances, y, limit);
            if ranges.len() == 1 && (ranges[0].0 > 0 || ranges[0].1 < limit) {
                if ranges[0].0 > 0 {
                    Some((0, y))
                } else {
                    Some((limit, y))
                }
            } else if ranges.len() == 2 {
                Some((ranges[0].1, y))
            } else {
                None
            }
        })
        .unwrap();
    let x = p.0 as usize;
    let y = p.1 as usize;

    x * 4_000_000 + y
}

/**
 Generate all x-intervals on ypos and merge them.
 - if the merged result has only one x-interval then (0, ypos) or (limit,ypos) might be a result.
 - if there are two intervals. The result is the end of the first x-interval (y, i.0).
*/
fn x_ranges(sensors_and_distances: &[(i32, i32, i32)], ypos: i32, limit: i32) -> Vec<(i32, i32)> {
    let result: Vec<(i32, i32)> = sensors_and_distances
        .iter()
        .filter_map(|(xs, ys, d)| {
            let dy = (ys - ypos).abs();
            let dx = d - dy;
            if dx >= 0 {
                Some(((xs - dx).max(0), (xs + dx + 1).min(limit)))
            } else {
                None
            }
        })
        .collect();

    merge_intervals(&result)
}

fn merge_intervals(a: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let n = a.len();

    if n == 0 {
        return Vec::new();
    }

    let mut arr = a.to_vec();
    let mut s = Vec::new();

    arr.sort_by(|a, b| a.0.cmp(&b.0));

    s.push(arr[0]);

    for ai in arr[1..].iter() {
        let top = s.last_mut().unwrap();
        if top.1 < ai.0 {
            s.push(*ai);
        } else if top.1 < ai.1 {
            *top = (top.0, ai.1)
        }
    }

    s
}

#[derive(Debug, Clone, Copy)]
struct Entry {
    sensor_pos: (i32, i32),
    beacon_pos: (i32, i32),
}

fn lines(input: &str) -> IResult<&str, Vec<Entry>> {
    separated_list1(line_ending, line)(input)
}

fn line(input: &str) -> IResult<&str, Entry> {
    map(
        separated_pair(
            preceded(tag("Sensor at "), point),
            tuple((tag(":"), space0)),
            preceded(tag("closest beacon is at "), point),
        ),
        |(p1, p2)| Entry {
            sensor_pos: p1,
            beacon_pos: p2,
        },
    )(input)
}

fn point(input: &str) -> IResult<&str, (i32, i32)> {
    use nom::character::complete::i32 as i32_parser;
    separated_pair(
        preceded(tag("x="), i32_parser),
        tuple((tag(","), space0)),
        preceded(tag("y="), i32_parser),
    )(input)
}

fn read_file(filename: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part1_works() {
        let result = part1(INPUT, 10);
        let expected = 26;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT, 20);
        let expected = 56000011;
        assert_eq!(result, expected);
    }

    #[test]
    fn merge_intervals_works() {
        let input = vec![(1, 3), (2, 6), (6, 9), (9, 10)];
        let expected = vec![(1, 10)];
        let result = merge_intervals(&input);
        assert_eq!(result, expected);
    }
}
