use std::fs;

use anyhow::Context;
use once_cell::sync::Lazy;

fn main() -> anyhow::Result<()> {
    let filename = "day-1.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;
    let result = part1(&input)?;
    println!("{result}");
    let result = part2(&input)?;
    println!("{result}");
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i32> {
    let result: Result<Vec<i32>, _> = input
        .lines()
        .map(|line| line.chars().filter(|c| c.is_digit(10)).collect())
        .map(|line: Vec<char>| format!("{}{}", line.first().unwrap_or(&'0'), line.last().unwrap_or(&'0')))
        .map(|s: String| s.parse())
        .collect();
    let result = result?;
    Ok(result.iter().copied().sum())
}

fn part2(input: &str) -> anyhow::Result<u32> {
    let lines = parse_input(input);
    let result = lines.into_iter().map(|(s, line)| {
        let result = line.first().copied().unwrap() * 10 + line.last().copied().unwrap();
        result
    })
        .sum();
    Ok(result)
}

fn parse_input(input: &str) -> Vec<(String, Vec<u32>)> {
    input.lines().map(parse_line).collect()
}

fn parse_line(input: &str) -> (String, Vec<u32>) {
    let size = input.len();
    let v = (0..size).filter_map(|idx| parse_digit(&input[idx..])).collect();
    (input.to_string(), v)
}

fn parse_digit(input: &str) -> Option<u32> {
    static DIGIT_WORDS: Lazy<Vec<(&str, u32)>> = Lazy::new(|| vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        // ("zero", 0),
    ]);

    if let Some(true) = input.chars().next().map(|c| c.is_digit(10)) {
        return Some(input.chars().next().unwrap() as u32 - '0' as u32);
    }

    for (prefix, digit) in DIGIT_WORDS.iter() {
        if input.starts_with(prefix) {
            return Some(*digit);
        }
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 142;
        assert_eq!(result, expected);
        Ok(())
    }


    static INPUT2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT2)?;
        let expected = 281;
        assert_eq!(result, expected);
        Ok(())
    }
}

