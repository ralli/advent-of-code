use anyhow::anyhow;
use std::fs;
use winnow::ModalResult;
use winnow::Parser;
use winnow::ascii::{digit1, line_ending, multispace0};
use winnow::combinator::{eof, separated, terminated};

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("day-03.txt")?;

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut inp = input;
    let banks = terminated(parse_banks, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|e| anyhow!("{e}"))?;
    let sum = banks.iter().map(|bank| get_max(bank, 2)).sum::<usize>();
    Ok(sum)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut inp = input;
    let banks = terminated(parse_banks, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|e| anyhow!("{e}"))?;
    let sum = banks.iter().map(|bank| get_max(bank, 12)).sum::<usize>();
    Ok(sum)
}

fn parse_banks(input: &mut &str) -> ModalResult<Vec<Vec<u32>>> {
    separated(1.., parse_digits, line_ending).parse_next(input)
}

fn parse_digits(input: &mut &str) -> ModalResult<Vec<u32>> {
    digit1
        .map(|digits: &str| {
            digits
                .chars()
                .map(|digit| digit.to_digit(10).unwrap_or(0))
                .collect::<Vec<_>>()
        })
        .parse_next(input)
}

fn get_max(bank: &[u32], count: usize) -> usize {
    let mut result = 0;
    let mut index = 0;

    for size in (1..=count).rev() {
        let mut found_digit = 0;

        for i in index..=(bank.len() - size) {
            let digit = bank[i];
            if digit > found_digit {
                found_digit = digit;
                index = i + 1;
            }
        }

        result = result * 10 + found_digit as usize;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 3121910778619);
    }
}
