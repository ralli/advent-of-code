use nom::character::complete::{line_ending, one_of, space1};
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let content = read_file("./day-9/input.txt")?;

    let result = part1(&content);
    println!("{}", result);

    let result = part2(&content);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let (_, directions) = directions(input).unwrap();
    let mut positions = HashSet::new();

    positions.insert(tail);

    for d in directions.into_iter() {
        head = next_head(&head, d);
        tail = next_tail(&head, &tail);
        positions.insert(tail);
    }
    positions.len()
}

fn part2(input: &str) -> usize {
    let mut tails = vec![(0, 0); 10];
    let (_, directions) = directions(input).unwrap();
    let mut positions = HashSet::new();

    positions.insert((0, 0));

    for d in directions.into_iter() {
        tails[0] = next_head(&tails[0], d);
        for i in 0..9 {
            tails[i + 1] = next_tail(&tails[i], &tails[i + 1]);
        }
        positions.insert(tails[9]);
    }
    positions.len()
}

fn next_head(head: &(i32, i32), direction: Direction) -> (i32, i32) {
    let (x, y) = *head;
    match direction {
        Direction::Up => (x, y + 1),
        Direction::Down => (x, y - 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    }
}

fn next_tail(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    if is_connected(head, tail) {
        return *tail;
    }

    let dx = (head.0 - tail.0).signum();
    let dy = (head.1 - tail.1).signum();

    (tail.0 + dx, tail.1 + dy)
}

fn is_connected(head: &(i32, i32), tail: &(i32, i32)) -> bool {
    (head.0 - 1) <= tail.0
        && (head.0 + 1) >= tail.0
        && (head.1 - 1) <= tail.1
        && (head.1 + 1) >= tail.1
}

fn read_file(filename: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}

fn directions(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, directions): (&str, Vec<Vec<Direction>>) =
        separated_list1(line_ending, moves)(input)?;
    Ok((input, directions.into_iter().flatten().collect()))
}

fn moves(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, c) = one_of("LRUD")(input)?;
    let (input, _) = space1(input)?;
    let (input, distance) = nom::character::complete::u32(input)?;

    let direction = match c {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        _ => Direction::Right,
    };

    let result: Vec<Direction> = vec![direction; distance as usize];
    Ok((input, result))
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 13;
        assert_eq!(result, expected);
    }
}
