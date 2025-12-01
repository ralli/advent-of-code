use std::fs;
use anyhow::anyhow;
use winnow::ModalResult;
use winnow::Parser;
use winnow::ascii::{digit1, line_ending, multispace0};
use winnow::combinator::{alt, eof, separated, terminated};

fn main() -> anyhow::Result<()>{
    let input = fs::read_to_string("day-1.txt")?;
    let result = part1(&input)?;
    println!("{result}");
    let result = part2(&input)?;
    println!("{result}");
    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Command {
    direction: Direction,
    distance: i32,
}

fn part1(input: &str) -> anyhow::Result<i32> {
    let commands = terminated(parse_input, (multispace0, eof))
        .parse(input)
        .map_err(|e| anyhow!("{e}"))?;
    let mut dial = 50;
    let mut count = 0;
    for cmd in commands.iter() {
        match cmd.direction {
            Direction::Left => dial -= cmd.distance,
            Direction::Right => dial += cmd.distance,
        }
        let dial = dial.rem_euclid(100);
        if dial == 0 {
            count += 1;
        }
    }
    Ok(count)
}

fn part2(input: &str) -> anyhow::Result<i32> {
    let commands = terminated(parse_input, (multispace0, eof))
        .parse(input)
        .map_err(|e| anyhow!("{e}"))?;
    let mut dial: i32 = 50;
    let mut count = 0;
    for cmd in commands.iter() {
        let delta = match cmd.direction {
            Direction::Left => -1,
            Direction::Right => 1
        };
        for _ in 0..cmd.distance {
            dial = (dial + delta).rem_euclid(100);
            if dial == 0 {
                count += 1;
            }
        }
    }
    Ok(count)
}


fn parse_input(input: &mut &str) -> ModalResult<Vec<Command>> {
    separated(1.., parse_command, line_ending).parse_next(input)
}

fn parse_command(input: &mut &str) -> ModalResult<Command> {
    (parse_direction, parse_distance)
        .map(|(direction, distance)| Command {
            direction,
            distance,
        })
        .parse_next(input)
}

fn parse_direction(input: &mut &str) -> ModalResult<Direction> {
    alt(('L'.value(Direction::Left), 'R'.value(Direction::Right))).parse_next(input)
}

fn parse_distance(input: &mut &str) -> ModalResult<i32> {
    digit1.parse_to::<i32>().parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;
        assert_eq!(part1(input).unwrap(), 3);
    }

    #[test]
    fn test_part2() {
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;
        assert_eq!(part2(input).unwrap(), 6);
    }
}