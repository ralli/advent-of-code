use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

use anyhow::Context;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space0, space1};
use nom::multi::{many1, separated_list1};
use nom::sequence::preceded;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let filename = "./day-4/input.txt";
    let content = read_file(filename).context(filename)?;

    let result = part1(&content).unwrap();
    println!("{}", result);

    let result = part2(&content).unwrap();
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> Option<i32> {
    let (_, input) = parse_input(input).unwrap();
    let mut numbers = HashSet::new();

    if input.numbers.len() < 5 {
        return None;
    }

    for &number in input.numbers.iter() {
        numbers.insert(number);
        if let Some(max_score) = input
            .boards
            .iter()
            .flat_map(|b| board_score(b, &numbers))
            .max()
        {
            return Some(max_score * number);
        }
    }

    None
}

fn board_score(board: &Vec<Vec<i32>>, numbers: &HashSet<i32>) -> Option<i32> {
    if !is_solved(board, numbers) {
        None
    } else {
        let mut sum = 0;
        for row in 0..board.len() {
            for col in 0..board[row].len() {
                if !numbers.contains(&board[row][col]) {
                    sum += board[row][col];
                }
            }
        }
        Some(sum)
    }
}

fn is_solved(board: &Vec<Vec<i32>>, numbers: &HashSet<i32>) -> bool {
    board
        .iter()
        .any(|row| row.iter().all(|v| numbers.contains(v)))
        || (0..board[0].len())
            .any(|col| (0..board.len()).all(|row| numbers.contains(&board[row][col])))
}

fn part2(input: &str) -> Option<i32> {
    let (_, input) = parse_input(input).unwrap();
    let mut numbers = HashSet::new();
    let mut board_scores: HashMap<usize, (usize, i32)> = HashMap::new();

    if input.numbers.len() < 5 {
        return None;
    }

    for (round, &number) in input.numbers.iter().enumerate() {
        numbers.insert(number);
        for (i, b) in input.boards.iter().enumerate() {
            if !board_scores.contains_key(&i) {
                if let Some(score) = board_score(&b, &numbers) {
                    board_scores.insert(i, (round, score * number));
                }
            }
        }
    }

    let mut bla = board_scores.values().copied().collect::<Vec<_>>();
    bla.sort_by(|a, b| b.0.cmp(&a.0));
    let (_, score) = bla.into_iter().next().unwrap();
    Some(score)
}

#[derive(Debug)]
struct Input {
    numbers: Vec<i32>,
    boards: Vec<Vec<Vec<i32>>>,
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, numbers) = numbers(input)?;
    let (input, _) = many1(line_ending)(input)?;
    let (input, boards) = boards(input)?;
    Ok((input, Input { numbers, boards }))
}

fn numbers(input: &str) -> IResult<&str, Vec<i32>> {
    use nom::character::complete::i32 as i32_parser;
    separated_list1(tag(","), i32_parser)(input)
}

fn boards(input: &str) -> IResult<&str, Vec<Vec<Vec<i32>>>> {
    separated_list1(many1(line_ending), board)(input)
}

fn board(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(line_ending, line)(input)
}

fn line(input: &str) -> IResult<&str, Vec<i32>> {
    use nom::character::complete::i32 as i32_parser;
    preceded(space0, separated_list1(space1, i32_parser))(input)
}

fn read_file(name: &str) -> anyhow::Result<String> {
    let mut f = File::open(name)?;
    let mut result = String::new();
    f.read_to_string(&mut result)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn part1_works() {
        let result = part1(INPUT).unwrap();
        let expected = 4512;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT).unwrap();
        let expected = 1924;
        assert_eq!(result, expected);
    }
}
