use anyhow::anyhow;
use std::fs;
use winnow::ascii::{digit1, multispace0};
use winnow::combinator::{delimited, eof, separated, separated_pair, terminated};
use winnow::{ModalResult, Parser};

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("day-02.txt")?;
    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<u64> {
    let mut inp = input;
    let ranges = terminated(parse_ranges, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|e| anyhow!("{e}"))?;
    let mut sum = 0;

    for &(a, b) in ranges.iter() {
        for i in a..=b {
            if is_invalid(i) {
                sum += i;
            }
        }
    }

    Ok(sum)
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let mut inp = input;
    let ranges = terminated(parse_ranges, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|e| anyhow!("{e}"))?;
    let mut sum = 0;

    for &(a, b) in ranges.iter() {
        for i in a..=b {
            if is_invalid2(i) {
                sum += i;
            }
        }
    }

    Ok(sum)
}

fn is_invalid2(n: u64) -> bool {
    let s = n.to_string();
    let bs = s.as_bytes();
    let size = bs.len();
    for i in 1..size {
        if size % i == 0 {
            let chunks = bs.chunks_exact(i).collect::<Vec<&[u8]>>();
            if chunks.windows(2).all(|w| w[0] == w[1]) {
                return true;
            }
        }
    }
    false
}

fn is_invalid(n: u64) -> bool {
    let divisor = divisor(n);
    divisor != 0 && (n / divisor == n % divisor)
}

fn divisor(n: u64) -> u64 {
    let num_digits = n.ilog10() as u64 + 1;
    if num_digits % 2 != 0 {
        return 0;
    }
    let mut start: u64 = 1;
    let mut n: u64 = num_digits / 2;
    while n > 0 {
        n -= 1;
        start *= 10;
    }
    start
}

type Range = (u64, u64);

fn parse_ranges(input: &mut &str) -> ModalResult<Vec<Range>> {
    separated(1.., parse_range, delimited(multispace0, ',', multispace0)).parse_next(input)
}

fn parse_range(input: &mut &str) -> ModalResult<Range> {
    separated_pair(parse_int, "-", parse_int).parse_next(input)
}

fn parse_int(input: &mut &str) -> ModalResult<u64> {
    digit1.parse_to::<u64>().parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn test_is_invalid() {
        assert!(is_invalid(11));
        assert!(!is_invalid(12));
    }

    #[test]
    fn test_is_invalid2() {
        assert!(is_invalid2(11));
        // assert!(!is_invalid(12));
    }

    #[test]
    fn test_part1() {
        let result = part1(INPUT).unwrap();
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT).unwrap();
        assert_eq!(result, 4174379265);
    }
}
