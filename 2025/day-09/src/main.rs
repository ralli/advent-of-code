use anyhow::anyhow;
use itertools::Itertools;
use winnow::ModalResult;
use winnow::Parser;
use winnow::ascii::{digit1, line_ending, multispace0};
use winnow::combinator::{eof, separated, separated_pair, terminated};

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day-09.txt")?;
    let result = part1(&input)?;
    println!("{result}");
    let result = part2(&input)?;
    println!("{result}");
    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Edge {
    min: Point,
    max: Point,
}

fn part1(input: &str) -> anyhow::Result<u64> {
    let mut inp = input;
    let points = terminated(parse_points, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|e| anyhow!("{e}"))?;
    let max_area = points
        .iter()
        .combinations(2)
        .map(|c| area(&c[0], &c[1]))
        .max()
        .unwrap_or_default();
    Ok(max_area)
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let mut inp = input;
    let points = terminated(parse_points, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|e| anyhow!("{e}"))?;
    let horiz: Vec<Edge> = create_horiz_edges(&points);
    let vert: Vec<Edge> = create_vert_edges(&points);
    let mut max_area = 0;

    for (i, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i + 1) {
            if !check_valid(*p1, *p2, &points, &horiz, &vert) {
                continue;
            }
            let a = area(p1, p2);
            if a > max_area {
                max_area = a;
            }
        }
    }

    Ok(max_area)
}

fn check_valid(p1: Point, p2: Point, points: &[Point], horiz: &[Edge], vert: &[Edge]) -> bool {
    let left = p1.x.min(p2.x);
    let top = p1.y.min(p2.y);
    let right = p1.x.max(p2.x);
    let bottom = p1.y.max(p2.y);
    let top_left = Point { x: left, y: top };
    let top_right = Point { x: right, y: top };
    let bottom_left = Point { x: left, y: bottom };
    let bottom_right = Point {
        x: right,
        y: bottom,
    };

    // one of the polygons points is inside the rectangle
    if points
        .iter()
        .any(|p| left < p.x && p.x < right && top < p.y && p.y < bottom)
    {
        return false;
    }

    // one of the rectangles corners is outside the polygon
    if !is_inside(top_left, horiz, vert)
        || !is_inside(top_right, horiz, vert)
        || !is_inside(bottom_left, horiz, vert)
        || !is_inside(bottom_right, horiz, vert)
    {
        return false;
    }

    // one of the polygons edges crosses the rectangle
    if horiz.iter().any(|e| {
        top < e.min.y
            && bottom > e.min.y
            && e.min.x.min(e.max.x) < right
            && e.min.x.max(e.max.x) > left
    }) {
        return false;
    }

    if vert.iter().any(|e| {
        left < e.min.x
            && right > e.min.x
            && e.min.y.min(e.max.y) < bottom
            && e.min.y.max(e.max.y) > top
    }) {
        return false;
    }

    true
}

fn create_horiz_edges(points: &[Point]) -> Vec<Edge> {
    let last_point = points.last().copied().unwrap();
    let first_point = points.first().copied().unwrap();
    let mut result = points
        .windows(2)
        .map(|w| (w[0], w[1]))
        .chain(std::iter::once((last_point, first_point)))
        .filter(|(p1, p2)| p1.y == p2.y)
        .map(|(p1, p2)| Edge {
            min: p1.min(p2),
            max: p1.max(p2),
        })
        .collect_vec();
    result.sort_unstable_by_key(|e| e.min.y);
    result
}

fn create_vert_edges(points: &[Point]) -> Vec<Edge> {
    let last_point = points.last().copied().unwrap();
    let first_point = points.first().copied().unwrap();
    let mut result = points
        .windows(2)
        .map(|w| (w[0], w[1]))
        .chain(std::iter::once((last_point, first_point)))
        .filter(|(p1, p2)| p1.x == p2.x)
        .map(|(p1, p2)| Edge {
            min: p1.min(p2),
            max: p1.max(p2),
        })
        .collect_vec();
    result.sort_unstable_by_key(|e| e.min.x);
    result
}

fn is_inside(p: Point, hedges: &[Edge], vedges: &[Edge]) -> bool {
    let mut left_edges = 0;
    let mut right_edges = 0;
    let mut crossed_edges = 0;

    let mut i = hedges.partition_point(|e| e.min.y < p.y);
    while i < hedges.len() && hedges[i].min.y == p.y {
        if (hedges[i].min.x..=hedges[i].max.x).contains(&p.x) {
            // we've hit a horizontal edge, so we're inside the polygon
            return true;
        }
        i += 1;
    }
    if i == hedges.len() {
        return false;
    }

    let mut j = vedges.partition_point(|e| e.min.x < p.x);
    while j < vedges.len() && vedges[j].min.x == p.x {
        if (vedges[j].min.y..=vedges[j].max.y).contains(&p.y) {
            // we've hit a horizontal edge, so we're inside the polygon
            return true;
        }
        j += 1;
    }
    if j == vedges.len() {
        return false;
    }

    for e in hedges.iter().skip(i) {
        if (e.min.x..=e.max.x).contains(&p.x) {
            if p.x == e.min.x {
                // hit a corner
                if e.max.x > p.x {
                    right_edges += 1;
                } else {
                    left_edges += 1;
                }
                if right_edges == left_edges {
                    // We've crossed as many right-pointing as left-pointing
                    // edges. Increase the total number of edges crossed.
                    crossed_edges += 1;
                }
            } else if p.x == e.max.x {
                // hit a corner
                if e.min.x > p.x {
                    right_edges += 1;
                } else {
                    left_edges += 1;
                }
                if right_edges == left_edges {
                    // We've crossed as many right-pointing as left-pointing
                    // edges. Increase the total number of edges crossed.
                    crossed_edges += 1;
                }
            } else {
                // hit the inside of the edge
                crossed_edges += 1;
            }
        }
        i += 1;
    }

    // we're inside the polygon if we've crossed an odd number of edges
    crossed_edges % 2 != 0
}

fn area(p1: &Point, p2: &Point) -> u64 {
    (p1.x.abs_diff(p2.x) + 1) * (p1.y.abs_diff(p2.y) + 1)
}

fn parse_points(input: &mut &str) -> ModalResult<Vec<Point>> {
    separated(1.., parse_point, line_ending).parse_next(input)
}

fn parse_point(input: &mut &str) -> ModalResult<Point> {
    separated_pair(parse_number, ',', parse_number)
        .map(|(x, y)| Point { x, y })
        .parse_next(input)
}

fn parse_number(input: &mut &str) -> ModalResult<u64> {
    digit1.parse_to::<u64>().parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 50);
    }

    #[test]
    fn test_is_inside() {
        let mut inp = INPUT;
        let points = parse_points(&mut inp).unwrap();
        let vert = create_vert_edges(&points);
        let horiz = create_horiz_edges(&points);
        println!("{vert:?}");
        let p1 = Point { x: 2, y: 3 };
        let p2 = Point { x: 9, y: 5 };

        assert!(check_valid(p1, p2, &points, &horiz, &vert))
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 24);
    }
}
