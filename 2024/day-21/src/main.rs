use anyhow::anyhow;
use nom::character::complete::{line_ending, multispace0, one_of};
use nom::combinator::eof;
use nom::multi::{many1, separated_list0};
use nom::IResult;
use std::iter;

fn main() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("day-21/input.txt")?;

    let result = part1(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let (_, inputs) = parse_input(input).map_err(|e| anyhow!(e.to_string()))?;
    let mut result = 0;

    for input in inputs.iter() {
        let complexity = solve1(input.as_slice());
        let n = numeric_part(input.as_slice())?;
        println!("{} {}", complexity, n);
        result += n * complexity;
    }

    Ok(result)
}

fn numeric_part(input: &[char]) -> anyhow::Result<usize> {
    let s = input
        .iter()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>();
    let result = s.parse::<usize>()?;
    Ok(result)
}

fn solve1(input: &[char]) -> usize {
    let mut pos_num = position_for_numeric('A');
    let mut pos_a1 = position_for_arrow('A');
    let mut pos_a2 = position_for_arrow('A');
    let mut result = 0;

    // print!("{}: ", input.iter().collect::<String>());

    for c in input.iter() {
        let target_num = position_for_numeric(*c);
        let (dr_num, dc_num) = move_to(&pos_num, &target_num);
        pos_num = target_num;

        let input_a1 = to_key_sequence(dr_num, dc_num);

        // print!("{}", input_a1.iter().collect::<String>());
        for c in input_a1.iter() {
            let target_a1 = position_for_arrow(*c);
            let (dr_a1, dc_a1) = move_to(&pos_a1, &target_a1);
            pos_a1 = target_a1;

            let input_a2 = to_key_sequence(dr_a1, dc_a1);

            // print!("{}", input_a2.iter().collect::<String>());
            for c in input_a2.iter() {
                let target_a2 = position_for_arrow(*c);
                let (dr_a2, dc_a2) = move_to(&pos_a2, &target_a2);
                pos_a2 = target_a2;

                let input_a3 = to_key_sequence(dr_a2, dc_a2);

                // print!("{}", input_a3.iter().collect::<String>());
                result += input_a3.len();
            }
        }
    }

    // println!();

    result
}

type Position = (isize, isize);

fn move_to(start: &Position, end: &Position) -> (isize, isize) {
    let (start_row, start_col) = *start;
    let (end_row, end_col) = *end;
    let (dr, dc) = (end_row - start_row, end_col - start_col);
    // moving over the empty space is not an issue here.
    (dr, dc)
}

fn to_key_sequence(dr: isize, dc: isize) -> Vec<char> {
    let ir = if dr > 0 {
        iter::repeat('v').take(dr.unsigned_abs())
    } else {
        iter::repeat('^').take(dr.unsigned_abs())
    };

    let ic = if dc > 0 {
        iter::repeat('>').take(dc.unsigned_abs())
    } else {
        iter::repeat('<').take(dc.unsigned_abs())
    };

    let it = match (dr.signum(), dc.signum()) {
        (1, 1) => ic.chain(ir),
        (-1, -1) => ir.chain(ic),
        _ => ir.chain(ic),
    };

    let it = it.chain(iter::once('A'));

    it.collect()
}

fn position_for_numeric(c: char) -> Position {
    match c {
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),

        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),

        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),

        '0' => (3, 1),
        'A' => (3, 2),
        _ => unreachable!("invalid code {}", c),
    }
}

fn position_for_arrow(c: char) -> Position {
    match c {
        '^' => (0, 1),
        'A' => (0, 2),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        _ => unreachable!("invalid code {}", c),
    }
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
        println!("{result}");
        Ok(())
    }

    #[test]
    fn test1() {
        let input = "379A".chars().collect::<Vec<_>>();
        let result = solve1(&input);
        println!("{result}");
    }
}
