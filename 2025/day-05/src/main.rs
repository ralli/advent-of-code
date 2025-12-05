use anyhow::anyhow;
use std::ops::RangeInclusive;
use winnow::ascii::{digit1, line_ending, multispace0, multispace1};
use winnow::combinator::{eof, separated, separated_pair, terminated};
use winnow::{ModalResult, Parser};

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day-05.txt")?;

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

#[derive(Debug, Clone)]
struct Input {
    ranges: Vec<(u64, u64)>,
    numbers: Vec<u64>,
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut inp = input;
    let data = terminated(parse_input, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|e| anyhow!("{e}"))?;
    let result = data
        .numbers
        .iter()
        .filter(|number| data.ranges.iter().any(|(a, b)| a <= number && number <= &b))
        .count();
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let mut inp = input;
    let data = terminated(parse_input, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|e| anyhow!("{e}"))?;
    let mut ranges = data.ranges;
    ranges.sort_by_key(|(a, _b)| *a);
    let mut count = 0;

    let mut start = 0;
    while start < ranges.len() {
        let mut end = start + 1;
        let mut current = ranges[start];
        while end < ranges.len() && current.0 <= ranges[end].1 && current.1 >= ranges[end].0 {
            current = (current.0.min(ranges[end].0), current.1.max(ranges[end].1));
            end += 1;
        }
        count += current.1 - current.0 + 1;
        start = end;
    }

    Ok(count)
}

fn parse_input(input: &mut &str) -> ModalResult<Input> {
    separated_pair(parse_ranges, multispace1, parse_numbers)
        .parse_next(input)
        .map(|(ranges, numbers)| Input { ranges, numbers })
}

fn parse_ranges(input: &mut &str) -> ModalResult<Vec<(u64, u64)>> {
    separated(1.., parse_range, line_ending).parse_next(input)
}

fn parse_numbers(input: &mut &str) -> ModalResult<Vec<u64>> {
    separated(1.., parse_number, line_ending).parse_next(input)
}

fn parse_range(input: &mut &str) -> ModalResult<(u64, u64)> {
    separated_pair(parse_number, "-", parse_number).parse_next(input)
}

fn parse_number(input: &mut &str) -> ModalResult<u64> {
    digit1.parse_to::<u64>().parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 14);
    }
}
