use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

use anyhow::Context;
use nom::{AsChar, IResult, Slice};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, satisfy};
use nom::multi::{many1, separated_list1};

fn main() -> anyhow::Result<()> {
    let filename = "./day-14/input.txt";
    let content = read_file(filename).context(filename)?;

    let result = part1(&content);
    println!("{}", result);

    let result = part2(&content);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> i32 {
    let (_, input_data) = input_data(input).unwrap();

    let polymer = (1..=10).fold(input_data.template.to_string(), |template, _| {
        step(&template, &input_data.rules)
    });

    let hist = create_histogram(&polymer);

    let mut counts: Vec<_> = hist.values().copied().collect();
    counts.sort_by(|a, b| b.cmp(a));

    counts.first().unwrap() - counts.last().unwrap()
}

fn step(template: &str, rules: &BTreeMap<&str, char>) -> String {
    let len = template.len();
    let mut result = String::with_capacity(len);

    for i in 0..len - 1 {
        let pair = template.slice(i..i + 2);
        result.push(pair.chars().next().unwrap());
        if let Some(rule) = rules.get(pair) {
            result.push(*rule);
        }
        // dbg!(pair, &result);
    }
    if len > 0 {
        result.push(template.chars().last().unwrap());
    }
    result
}

fn create_histogram(input: &str) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();
    input.chars().for_each(|c| {
        let count = result.entry(c).or_insert(0);
        *count += 1;
    }
    );
    result
}

fn part2(input: &str) -> u64 {
    let (_, input_data) = input_data(input).unwrap();
    let mut hist = create_initial_histogram(input_data.template);

    for _round in 1..=40 {
        step2(&mut hist, &input_data.rules);
    }

    let char_freq = create_final_histogram(input_data.template, &hist);

    let mut counts: Vec<_> = char_freq.values().copied().collect();

    counts.sort_by(|a, b| b.cmp(a));

    counts.first().unwrap() - counts.last().unwrap()
}

/// if using a string as in part1, we will quickly run out of RAM since the string size grows exponentially.
/// the idea:
///   We keep track of all pairs of characters.
///   Each rule (if applied) increases the count of the pairs (first, inserted_char), (inserted_char, last).
///   The pair itself will vanish.
///
fn step2(hist: &mut BTreeMap<String, u64>, rules: &BTreeMap<&str, char>) {
    let pairs: Vec<(String, u64)> = hist.iter().map(|(k, v)| (k.to_string(), *v)).collect();

    for (pair, count) in pairs.into_iter() {
        if let Some(&insert) = rules.get(pair.as_str()) {
            let first_pair: String = [pair.chars().next().unwrap(), insert].iter().collect();
            let entry = hist.entry(first_pair).or_insert(0);
            *entry += count;

            let second_pair: String = [insert, pair.chars().last().unwrap()].iter().collect();
            let entry = hist.entry(second_pair).or_insert(0);
            *entry += count;

            let entry = hist.entry(pair).or_insert(0);
            *entry -= count;
        }
    }
}

///
/// the character frequencies are calculated as follows:
///   aggregate the counts of the first character of each pair. If we would count the second
///   character, we will count most characters twice, since the second character is the start
///   of the next pair.
///   The last character of the template has to be added as well as it is never the first
///   character of a pair.
///
fn create_final_histogram(template: &str, hist: &BTreeMap<String, u64>) -> BTreeMap<char, u64> {
    let mut result = BTreeMap::new();

    for (pair, &count) in hist.iter() {
        let first_char = pair.chars().next().unwrap();
        let entry = result.entry(first_char).or_insert(0);
        *entry += count;
    }

    let last_char = template.chars().last().unwrap();
    let entry = result.entry(last_char).or_insert(0);
    *entry += 1;

    result
}

fn create_initial_histogram(template: &str) -> BTreeMap<String, u64> {
    let len = template.len();
    let mut result = BTreeMap::new();
    for i in 0..len - 1 {
        let pair = template.slice(i..i + 2).to_string();
        let elem = result.entry(pair).or_insert(0);
        *elem += 1;
    }
    result
}

fn input_data(input: &str) -> IResult<&str, InputData> {
    let (input, template) = alpha1(input)?;
    let (input, _) = many1(line_ending)(input)?;
    let (input, rules) = separated_list1(line_ending, rule)(input)?;

    Ok((input, InputData { template, rules: rules.into_iter().collect() }))
}

fn rule(input: &str) -> IResult<&str, (&str, char)> {
    let (input, pair) = alpha1(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, insert) = satisfy(|c| c.is_alpha())(input)?;
    Ok((input, (pair, insert)))
}

#[derive(Debug)]
struct InputData<'a> {
    template: &'a str,
    rules: BTreeMap<&'a str, char>,
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
        let expected = 1588;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 2188189693529;
        assert_eq!(result, expected);
    }
}
