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
    Ok(())
}

type Point = (u64, u64);
type Edge = (Point, Point);

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
    let mut horiz: Vec<Edge> = create_horiz_edges(&points);

    let mut vert: Vec<Edge> = create_vert_edges(&points);

    horiz.sort_unstable_by_key(|&((xmin, _ymin), (_xmax, _ymax))| xmin);
    vert.sort_unstable_by_key(|&((_xmin, ymin), (_xmax, _ymax))| ymin);

    let mut max_area = 0;
    for (i, &(x1, y1)) in points.iter().enumerate() {
        for (j, &(x2, y2)) in points.iter().enumerate().skip(i + 1) {
            if x1 == x2 && y1 == y2 {
                continue;
            }
            let left = x1.min(x2);
            let top = y1.min(y2);
            let right = x1.max(x2);
            let bottom = y1.max(y2);
            let top_left = (left, top);
            let top_right = (right, top);
            let bottom_left = (left, bottom);
            let bottom_right = (right, bottom);

            println!("{top_left:?} - {bottom_right:?}");
            // one point lies within the rectangle => false
            if points
                .iter()
                .any(|&(x, y)| left < x && x < right && top < y && y < bottom)
            {
                continue;
            }

            // one of the points of the rectangle is not inside the polygon => false
            if !is_inside(&top_left, &vert)
                || !is_inside(&top_right, &vert)
                || !is_inside(&bottom_left, &vert)
                || !is_inside(&bottom_right, &vert)
            {
                continue;
            }

            let a = area(&top_left, &bottom_right);
            if a > max_area {
                println!("({x1},{y1}), ({x2},{y2}) -> {a}");
                max_area = a;
            }
        }
    }

    Ok(max_area)
}

fn check_valid((x1, y1): Point, (x2, y2): Point, points: &[Point], vert: &[Edge]) -> bool {
    let left = x1.min(x2);
    let top = y1.min(y2);
    let right = x1.max(x2);
    let bottom = y1.max(y2);
    let top_left = (left, top);
    let top_right = (right, top);
    let bottom_left = (left, bottom);
    let bottom_right = (right, bottom);

    if points
        .iter()
        .any(|&(x, y)| left < x && x < right && top < y && y < bottom)
    {
        return false;
    }
    if !is_inside(&top_left, &vert)
        || !is_inside(&top_right, &vert)
        || !is_inside(&bottom_left, &vert)
        || !is_inside(&bottom_right, &vert)
    {
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
        .filter(|&((x1, y1), (x2, y2))| y1 == y2)
        .map(|((x1, y1), (x2, y2))| ((x1.min(x2), y1.min(y2)), (x1.max(x2), y1.max(y2))))
        .collect_vec();
    result.sort_unstable_by_key(|&((xmin, _ymin), (_xmax, _ymax))| xmin);
    result
}

fn create_vert_edges(points: &[Point]) -> Vec<Edge> {
    let last_point = points.last().copied().unwrap();
    let first_point = points.first().copied().unwrap();
    let mut result = points
        .windows(2)
        .map(|w| (w[0], w[1]))
        .chain(std::iter::once((last_point, first_point)))
        .filter(|&((x1, y1), (x2, y2))| x1 == x2)
        .map(|((x1, y1), (x2, y2))| ((x1.min(x2), y1.min(y2)), (x1.max(x2), y1.max(y2))))
        .collect_vec();
    result.sort_unstable_by_key(|&((xmin, ymin), (_xmax, _ymax))| xmin);
    result
}

fn is_inside((x, y): &Point, vert: &[Edge]) -> bool {
    let xcoords = vert
        .iter()
        .filter(|&((_, ymin), (_, ymax))| y >= ymin && y <= ymax)
        .map(|&((x, _), (_, _))| x)
        .collect_vec();

    for (i, (x1, x2)) in xcoords.iter().tuple_windows::<(_, _)>().enumerate() {
        if x >= x1 && x <= x2 {
            return i % 2 == 0;
        }
    }
    false
}

fn area((x1, y1): &Point, (x2, y2): &Point) -> u64 {
    (x1.abs_diff(*x2) + 1) * (y2.abs_diff(*y1) + 1)
}

fn parse_points(input: &mut &str) -> ModalResult<Vec<Point>> {
    separated(1.., parse_point, line_ending).parse_next(input)
}

fn parse_point(input: &mut &str) -> ModalResult<Point> {
    separated_pair(parse_number, ',', parse_number).parse_next(input)
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
        println!("{vert:?}");
        let (x1, y1) = (2, 3);
        let (x2, y2) = (9, 5);

        assert!(check_valid((x1, y1), (x2, y2), &points, &vert))
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 24);
    }
}
