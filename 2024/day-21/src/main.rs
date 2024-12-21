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

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let (_, inputs) = parse_input(input).map_err(|e| anyhow!(e.to_string()))?;
    let mut solver = Solver::new();

    let result = inputs
        .iter()
        .map(|input| {
            let length = solver.solve(input, 2);
            let n = numeric_part(input).unwrap();
            (n, length)
        })
        // .inspect(|(n, length)| {
        //     println!("{length} {n}");
        // })
        .map(|(n, length)| n * length)
        .sum();

    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let (_, inputs) = parse_input(input).map_err(|e| anyhow!(e.to_string()))?;
    let mut solver = Solver::new();

    let result = inputs
        .iter()
        .map(|input| {
            let length = solver.solve(input, 25);
            let n = numeric_part(input).unwrap();
            (n, length)
        })
        // .inspect(|(n, length)| {
        //     println!("{length} {n}");
        // })
        .map(|(n, length)| n * length)
        .sum();

    Ok(result)
}

type Keyboard = HashMap<char, Position>;
type Position = (isize, isize);

///
/// First simulated a series of keyboards to solve part1.
/// This was too inefficient to solve part2.
///
/// Observations:
/// - Each keyboard ends up in the same state with its robot arm pointing to 'A'.
/// - The lengths for each move on the numeric keyboard can be calculated independently.
/// - The lengths of each move of each of the keyboards can be cached with a key (n, from, to) where
///   n is some ID of the keyboard.
///  
struct Solver {
    cache: HashMap<(char, char, usize), usize>,
    dir_answers: HashMap<(char, char), Vec<Vec<char>>>,
    num_answers: HashMap<(char, char), Vec<Vec<char>>>,
}

impl Solver {
    fn new() -> Self {
        let cache = HashMap::new();
        let num_keypad = create_num_keyboard();
        let dir_keypad = create_directional_keyboard();
        let dir_answers = create_answers(&dir_keypad);
        let num_answers = create_answers(&num_keypad);
        Self {
            cache,
            dir_answers,
            num_answers,
        }
    }

    fn solve(&mut self, input: &[char], depth: usize) -> usize {
        moves_for(input)
            .iter()
            .map(|&(a, b)| {
                let inputs = self.num_answers.get(&(a, b)).cloned().unwrap();
                inputs
                    .iter()
                    .map(|input| self.int_solve(input, depth))
                    .min()
                    .unwrap_or_default()
            })
            .sum()
    }

    ///
    /// Takes a series of inputs on the directional keyboard and  
    /// calculates the number of moves required to perform up to directional keyboard
    /// directly before the numeric keyboard.
    ///
    fn int_solve(&mut self, input: &[char], depth: usize) -> usize {
        moves_for(input)
            .iter()
            .map(|&(from, to)| self.calc_length(from, to, depth))
            .sum()
    }

    ///
    /// calculates the number of moves required for a single move on this directional
    /// keyboard identified by `depth` up to the first directional keyboard (the one which you type on).
    ///
    /// Caches (memoizes) the intermediate results.
    ///
    fn calc_length(&mut self, from: char, to: char, depth: usize) -> usize {
        if let Some(&count) = self.cache.get(&(from, to, depth)) {
            return count;
        }
        if depth == 1 {
            // this is the number of moves on the first keyboard
            // to move the robot arm from the first to the second position
            // and press the 'A' button.
            return self
                .dir_answers
                .get(&(from, to))
                .map(|v| v.first().map(|w| w.len()).unwrap_or_default())
                .unwrap_or_default();
        }

        //
        // calculate the minimum number of moves required one level up the chain...
        //
        let inputs = self.dir_answers.get(&(from, to)).cloned().unwrap();
        let min_length = inputs
            .iter()
            .map(|input| self.int_solve(input, depth - 1))
            .min()
            .unwrap_or_default();

        self.cache.insert((from, to, depth), min_length);

        min_length
    }
}

fn numeric_part(input: &[char]) -> anyhow::Result<usize> {
    let s = input
        .iter()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>();
    let result = s.parse::<usize>()?;
    Ok(result)
}

fn create_directional_keyboard() -> Keyboard {
    HashMap::from([
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ])
}

fn moves_for(input: &[char]) -> Vec<(char, char)> {
    let mut from = 'A';
    let mut result = Vec::new();
    for &to in input.iter() {
        result.push((from, to));
        from = to;
    }
    result
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

fn create_answers(keyboard: &Keyboard) -> HashMap<(char, char), Vec<Vec<char>>> {
    let valid_positions: HashSet<Position> = keyboard.values().copied().collect();
    let mut result = HashMap::new();
    for (from_key, from_position) in keyboard.iter() {
        for (to_key, to_position) in keyboard.iter() {
            result.insert(
                (*from_key, *to_key),
                paths_for(from_position, to_position, &valid_positions),
            );
        }
    }
    result
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
            let mut result = path.clone();
            result.push('A');
            results.push(result);
            continue;
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
    fn test_solve() {
        let input = "029A".chars().collect::<Vec<_>>();
        let mut solver = Solver::new();
        let result = solver.solve(&input, 25);
        println!("{result}");
    }
}
