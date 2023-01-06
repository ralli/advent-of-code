use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

use anyhow::Context;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, space1};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let filename = "./day-8/input.txt";
    let content = read_file(filename).context(filename)?;

    let result = part1(&content);
    println!("{}", result);

    let result = part2(&content);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> usize {
    let (_, entries) = entries(input).unwrap();
    let unique_segment_numbers = vec![2, 4, 3, 7];
    entries
        .into_iter()
        .map(|e| e.output_value)
        .map(|d| {
            d.iter()
                .filter(|d| unique_segment_numbers.contains(&d.len()))
                .count()
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    let (_, entries) = entries(input).unwrap();
    entries.iter().map(|entry| output(entry)).sum()
}

fn output(entry: &Entry) -> i32 {
    let digits = digits(entry);
    entry
        .output_value
        .iter()
        .fold(0, |x, digit| x * 10 + digits.get(digit).unwrap())
}

fn digits(entry: &Entry) -> BTreeMap<&Vec<char>, i32> {
    let one = entry.signal_patterns.iter().find(|e| e.len() == 2).unwrap();
    let four = entry.signal_patterns.iter().find(|e| e.len() == 4).unwrap();
    let seven = entry.signal_patterns.iter().find(|e| e.len() == 3).unwrap();
    let eight = entry.signal_patterns.iter().find(|e| e.len() == 7).unwrap();
    let six = entry
        .signal_patterns
        .iter()
        .find(|e| e.len() == 6 && !one.iter().all(|c| e.contains(c)))
        .unwrap();
    let nine = entry
        .signal_patterns
        .iter()
        .find(|e| e.len() == 6 && four.iter().all(|c| e.contains(c)))
        .unwrap();
    let zero = entry
        .signal_patterns
        .iter()
        .find(|e| {
            e.len() == 6 && one.iter().all(|c| e.contains(c)) && !four.iter().all(|c| e.contains(c))
        })
        .unwrap();

    let segment_c = one.iter().find(|&c| !six.contains(c)).unwrap();
    let segment_f = one.iter().find(|&c| six.contains(c)).unwrap();

    let two = entry
        .signal_patterns
        .iter()
        .find(|e| e.len() == 5 && e.contains(segment_c) && !e.contains(segment_f))
        .unwrap();
    let three = entry
        .signal_patterns
        .iter()
        .find(|e| e.len() == 5 && e.contains(segment_c) && e.contains(segment_f))
        .unwrap();
    let five = entry
        .signal_patterns
        .iter()
        .find(|e| e.len() == 5 && !e.contains(segment_c) && e.contains(segment_f))
        .unwrap();

    let digits = vec![
        (zero, 0),
        (one, 1),
        (two, 2),
        (three, 3),
        (four, 4),
        (five, 5),
        (six, 6),
        (seven, 7),
        (eight, 8),
        (nine, 9),
    ];

    // println!("{:?}", digits);
    let result = BTreeMap::from_iter(digits.into_iter());
    // dbg!(&result);
    assert_eq!(result.len(), 10);

    result
}

#[derive(Debug)]
struct Entry {
    signal_patterns: Vec<Vec<char>>,
    output_value: Vec<Vec<char>>,
}

fn entries(input: &str) -> IResult<&str, Vec<Entry>> {
    separated_list1(line_ending, entry)(input)
}

fn entry(input: &str) -> IResult<&str, Entry> {
    let (input, signal_patterns) = digit_patterns(input)?;
    let (input, _) = tuple((space1, tag("|"), space1))(input)?;
    let (input, output_value) = digit_patterns(input)?;

    let signal_patterns = signal_patterns
        .iter()
        .map(|p| {
            let mut v: Vec<char> = p.chars().collect();
            v.sort();
            v
        })
        .collect();

    let output_value = output_value
        .iter()
        .map(|p| {
            let mut v: Vec<char> = p.chars().collect();
            v.sort();
            v
        })
        .collect();

    Ok((
        input,
        Entry {
            signal_patterns,
            output_value,
        },
    ))
}

fn digit_patterns(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(space1, alpha1)(input)
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

    const INPUT: &str = include_str!("../test.txt");

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 26;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 61229;
        assert_eq!(result, expected);
    }
}
