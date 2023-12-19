use anyhow::anyhow;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete;
use nom::character::complete::{line_ending, one_of, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::IResult;

pub fn part1(input: &str) -> anyhow::Result<i64> {
    let commands = parse_input(input)?;
    let mut start = Position::new(0, 0);
    let mut internal_area = 0;
    for command in commands.iter() {
        let (dx, dy) = command.direction.delta();
        let end = Position::new(
            start.x + dx * command.distance,
            start.y + dy * command.distance,
        );
        internal_area += start.x * end.y - start.y * end.x;
        start = end;
    }

    let internal_points = internal_area.abs() / 2;
    let num_edge_points: i64 = commands.iter().map(|c| c.distance).sum::<i64>() / 2 + 1;
    Ok(internal_points + num_edge_points)
}

pub fn part2(input: &str) -> anyhow::Result<i64> {
    let commands = parse_input(input)?;
    let mut start = Position::new(0, 0);
    let mut internal_area = 0;

    for command in commands.iter() {
        let distance = command.color >> 4;
        let direction = match command.color & 0xf {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => unreachable!("{:x}", command.color),
        };
        let (dx, dy) = direction.delta();
        let end = Position::new(start.x + dx * distance, start.y + dy * distance);
        // shoelace formula (without division by 2)
        internal_area += start.x * end.y - start.y * end.x;
        start = end;
    }

    // shoelace formula (division by 2)
    let internal_points = internal_area.abs() / 2;

    // pick's theorem is A = I + R/2 - 1
    // A is the area of the polygon (calculated above)
    // R is the number of points on the border = sum of all distances
    // I is the number of internal points. (not relevant here)
    //
    // R/2 - 1 is the inside area taken by cubes of the the edge within the polygon.
    //
    // The "A" we calculate does not cover all "cube areas" on the edge.
    // To get the total Area, we will have to add the missing area of edge-cubes lying on the outside of the polygon.
    //
    // This is where the R/2 + 1 comes from (R = (R/2) - 1 + (R/2) + 1) :-)

    let num_edge_points: i64 = commands.iter().map(|c| c.color >> 4).sum::<i64>() / 2 + 1;
    Ok(internal_points + num_edge_points)
}

#[derive(Debug, Copy, Clone)]
struct Command {
    direction: Direction,
    distance: i64,
    color: i64,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(&self) -> (i64, i64) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(row: i64, col: i64) -> Self {
        Self { x: row, y: col }
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Command>> {
    let (_, commands) = parse_commands(input).map_err(|e| anyhow!(e.to_string()))?;
    Ok(commands)
}

fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(line_ending, parse_command)(input)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, direction) = parse_direction(input)?;
    let (input, _) = space1(input)?;
    let (input, distance) = complete::u32(input)?;
    let (input, _) = space1(input)?;
    let (input, color) = delimited(tag("("), hex_color, tag(")"))(input)?;
    Ok((
        input,
        Command {
            direction,
            distance: distance as i64,
            color: color as i64,
        },
    ))
}

fn from_hex(input: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn hex_primary(input: &str) -> IResult<&str, u32> {
    map_res(take_while_m_n(6, 6, is_hex_digit), from_hex)(input)
}

fn hex_color(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("#")(input)?;
    let (input, color) = hex_primary(input)?;

    Ok((input, color))
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, c) = one_of("UDLR")(input)?;
    let dir = match c {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => unreachable!("no direction"),
    };
    Ok((input, dir))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 62;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected: i64 = 952408144115;
        assert_eq!(result, expected);
        Ok(())
    }

    static INPUT2: &str = r#"R 3 (#000000)
D 2 (#000000)
R 5 (#000000)
U 2 (#000000)
R 3 (#000000)
D 4 (#000000)
L 11 (#000000)
U 4 (#000000)"#;

    #[test]
    fn test1() -> anyhow::Result<()> {
        let result = part1(INPUT2)?;
        let expected = 52;
        assert_eq!(result, expected);
        Ok(())
    }
}
