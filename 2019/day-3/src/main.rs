use anyhow::anyhow;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{newline, one_of};
use nom::multi::separated_list0;
use nom::IResult;
use std::fs;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let input = read_file("day-3/day-3.txt")?;
    //     let input = r#"R8,U5,L5,D3
    // U7,R6,D4,L4"#;

    let result = part1(&input)?;
    println!("{}", result);

    let result = part2(&input)?;
    println!("{}", result);
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i32> {
    let (_, wires) = parse_input(&input).map_err(|e| anyhow!(e.to_string()))?;
    let (horiz1, vert1) = create_line_segments(&wires[0]);
    let (horiz2, vert2) = create_line_segments(&wires[1]);
    let mut intersections = find_horiz_intersections(&horiz1, &vert2);
    intersections.extend(find_horiz_intersections(&horiz2, &vert1).into_iter());
    let result = intersections.into_iter().map(|(x, y)| x.abs() + y.abs()).min().unwrap_or(0);
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<i32> {
    let (_, wires) = parse_input(&input).map_err(|e| anyhow!(e.to_string()))?;
    let (horiz1, vert1) = create_line_segments(&wires[0]);
    let (horiz2, vert2) = create_line_segments(&wires[1]);
    let mut intersections = find_horiz_intersections(&horiz1, &vert2);
    intersections.extend(find_horiz_intersections(&horiz2, &vert1).into_iter());
    let distances1 = intersections.iter().map(|p| distances_to_intersection(&wires[0], *p).unwrap()).collect::<Vec<_>>();
    let distances2 = intersections.iter().map(|p| distances_to_intersection(&wires[1], *p).unwrap()).collect::<Vec<_>>();
    Ok(distances1.iter().zip(distances2.iter()).map(|(a, b)| *a + *b).min().unwrap_or(0))
}

fn distances_to_intersection(wire: &[Move], (x, y): (i32, i32)) -> Option<i32> {
    let (mut curr_x, mut curr_y) = (0, 0);
    let mut distance = 0;
    for m in wire.iter() {
        let (dx, dy) = move_delta(m);
        match m {
            Move::Up(_) | Move::Down(_) => {
                if curr_x == x && ((curr_y < y && curr_y + dy > y) || (curr_y > y && curr_y + dy < y)) {
                    return Some(distance + (curr_y - y).abs());
                }
            }
            Move::Left(_) | Move::Right(_) => {
                if curr_y == y && ((curr_x < x && curr_x + dx > x) || (curr_x > x && curr_x + dx < x)) {
                    return Some(distance + (curr_x - x).abs());
                }
            }
        }
        curr_x += dx;
        curr_y += dy;
        distance += dx.abs() + dy.abs();
    }
    None
}

fn move_delta(m: &Move) -> (i32, i32) {
    match m {
        Move::Up(distance) => (0, *distance),
        Move::Down(distance) => (0, -*distance),
        Move::Left(distance) => (-*distance, 0),
        Move::Right(distance) => (*distance, 0),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

type Wire = Vec<Move>;

fn find_horiz_intersections(horiz: &[(i32, i32, i32)], vert: &[(i32, i32, i32)]) -> Vec<(i32, i32)> {
    let mut result = Vec::new();

    for (y, x1, x2) in horiz.iter() {
        let px1 = x1.min(x2);
        let px2 = x1.max(x2);
        for (x, y1, y2) in vert.iter() {
            let py1 = y1.min(y2);
            let py2 = y1.max(y2);
            if py1 < y && py2 > y && px1 < x && px2 > x {
                result.push((*x, *y));
            }
        }
    }
    result
}

fn create_line_segments(wire: &[Move]) -> (Vec<(i32, i32, i32)>, Vec<(i32, i32, i32)>) {
    let (mut x, mut y) = (0, 0);
    let mut horiz = Vec::new();
    let mut vert = Vec::new();
    for m in wire.iter() {
        let (dx, dy) = move_delta(m);
        match m {
            Move::Up(_) | Move::Down(_) => vert.push((x, y, y + dy)),
            Move::Left(_) | Move::Right(_) => horiz.push((y, x, x + dx)),
        };
        x += dx;
        y += dy;
    }
    (horiz, vert)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Wire>> {
    separated_list0(newline, parse_wire)(input)
}

fn parse_wire(input: &str) -> IResult<&str, Wire> {
    separated_list0(tag(","), parse_move)(input)
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (rest, ch) = one_of("UDLR")(input)?;
    let (rest, distance) = complete::i32(rest)?;
    let m = match ch {
        'U' => Move::Up(distance),
        'D' => Move::Down(distance),
        'L' => Move::Left(distance),
        'R' => Move::Right(distance),
        _ => unreachable!()
    };
    Ok((rest, m))
}

fn read_file(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let s = fs::read_to_string(&path)?;
    Ok(s)
}
