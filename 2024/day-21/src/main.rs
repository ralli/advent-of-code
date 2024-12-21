use anyhow::anyhow;
use nom::character::complete::{line_ending, multispace0, one_of};
use nom::combinator::eof;
use nom::multi::{many1, separated_list0};
use nom::IResult;
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter;

fn main() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("day-21/input.txt")?;

    let result = part1(&content)?;
    println!("{result}");

    Ok(())
}

type Keyboard = HashMap<char, Position>;
type Position = (isize, isize);

fn part1(input: &str) -> anyhow::Result<usize> {
    let (_, inputs) = parse_input(input).map_err(|e| anyhow!(e.to_string()))?;
    let mut result = 0;

    for input in inputs.iter() {
        let complexity = solve1(input);
        let n = numeric_part(input)?;
        result += n * complexity;
    }

    Ok(result)
}

fn solve1(input: &[char]) -> usize {
    let num_keyboard = create_num_keyboard();
    let arrow_keyboard = create_arrow_keyboard();

    let robot1 = solve(input, &num_keyboard);

    let robot2: Vec<Vec<char>> = robot1
        .iter()
        .flat_map(|input| solve(input, &arrow_keyboard))
        .collect();

    let min_len = robot2
        .iter()
        .map(|positions| positions.len())
        .min()
        .unwrap_or_default();

    let robot3: Vec<Vec<char>> = robot2
        .iter()
        .filter(|r| r.len() == min_len)
        .flat_map(|input| solve(input, &arrow_keyboard))
        .collect();

    robot3
        .iter()
        .map(|positions| positions.len())
        .min()
        .unwrap_or_default()
}

fn numeric_part(input: &[char]) -> anyhow::Result<usize> {
    let s = input
        .iter()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>();
    let result = s.parse::<usize>()?;
    Ok(result)
}

fn solve(input: &[char], keyboard: &Keyboard) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    let valid_positions: HashSet<Position> = keyboard.values().copied().collect();
    let pos = position_for('A', keyboard).unwrap();
    let mut cache = HashMap::new();
    let mut q: VecDeque<(&[char], Position, Vec<char>)> =
        VecDeque::from([(input, pos, Vec::new())]);

    while let Some((input, pos, path)) = q.pop_front() {
        if input.is_empty() {
            result.push(path);
            continue;
        }
        let c = input.first().unwrap();
        let input = &input[1..];
        let next_pos = position_for(*c, keyboard).unwrap();
        let inputs = cached_paths_for(&mut cache, &pos, &next_pos, &valid_positions);
        for bla in inputs.iter() {
            let next_path = path
                .iter()
                .copied()
                .chain(bla.iter().copied())
                .chain(iter::once('A'))
                .collect();
            q.push_back((input, next_pos, next_path));
        }
    }
    result
}

fn create_arrow_keyboard() -> Keyboard {
    HashMap::from([
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ])
}

fn create_num_keyboard() -> Keyboard {
    HashMap::from([
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('0', (3, 1)),
        ('A', (3, 2)),
    ])
}

fn cached_paths_for<'a>(
    cache: &'a mut HashMap<(Position, Position), Vec<Vec<char>>>,
    from: &Position,
    to: &Position,
    valid_positions: &HashSet<Position>,
) -> &'a Vec<Vec<char>> {
    cache
        .entry((*from, *to))
        .or_insert_with(|| paths_for(from, to, valid_positions))
}

fn paths_for(
    from: &Position,
    to: &Position,
    valid_positions: &HashSet<Position>,
) -> Vec<Vec<char>> {
    const DIRS: [(isize, isize, char); 4] = [(1, 0, 'v'), (-1, 0, '^'), (0, 1, '>'), (0, -1, '<')];
    let mut q: VecDeque<(Position, Vec<char>)> = VecDeque::from([(*from, Vec::new())]);
    let mut min_len = usize::MAX;
    let mut results = Vec::new();

    while let Some(((row, col), path)) = q.pop_front() {
        if !is_valid(row, col, valid_positions) {
            continue;
        };
        if (row, col) == *to {
            min_len = path.len();
            results.push(path.clone());
        }
        if path.len() >= min_len {
            continue;
        }
        for &(dr, dc, c) in DIRS.iter() {
            let (next_row, next_col) = (row + dr, col + dc);
            if !is_valid(next_row, next_col, valid_positions) {
                continue;
            }
            let next_path = path.iter().copied().chain(iter::once(c)).collect();
            q.push_back(((next_row, next_col), next_path));
        }
    }

    results
}

fn is_valid(row: isize, col: isize, valid_positions: &HashSet<Position>) -> bool {
    valid_positions.contains(&(row, col))
}

fn position_for(c: char, keyboard: &Keyboard) -> Option<Position> {
    keyboard.get(&c).copied()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let parse_line = many1(one_of("0123456789A"));
    let (input, lines) = separated_list0(line_ending, parse_line)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = eof(input)?;
    Ok((input, lines))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"029A
980A
179A
456A
379A"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        assert_eq!(result, 126384);
        Ok(())
    }

    #[test]
    fn test_paths_for() {
        let num_keyboard = create_num_keyboard();
        let num_positions: HashSet<Position> = num_keyboard.values().copied().collect();

        let from = position_for('A', &num_keyboard).unwrap();
        let to = position_for('8', &num_keyboard).unwrap();
        let paths = paths_for(&from, &to, &num_positions);
        println!("{paths:?}");
    }

    #[test]
    fn test_solve() {
        let num_keyboard = create_num_keyboard();
        let input = "029A".chars().collect::<Vec<_>>();
        let results = solve(&input, &num_keyboard);
        assert_eq!(results.len(), 3);
    }
}
