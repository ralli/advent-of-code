use anyhow::anyhow;
use nom::character::complete;
use nom::character::complete::{line_ending, multispace0};
use nom::combinator::eof;
use nom::multi::separated_list0;
use nom::IResult;
use std::collections::{HashMap, HashSet};
use std::iter;

fn main() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("day-22/input.txt")?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let (_, numbers) = parse_input(input).map_err(|e| anyhow!("{e}"))?;
    let result: i64 = numbers.iter().map(|n| number_after_steps(*n, 2000)).sum();
    Ok(result as usize)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let (_, numbers) = parse_input(input).map_err(|e| anyhow!("{e}"))?;
    let mut totals: HashMap<(i8, i8, i8, i8), i64> = HashMap::new();
    for num in numbers.iter() {
        let prices = prices_for_number(*num).take(2000).collect::<Vec<_>>();
        let mut visited = HashSet::new();
        for w in prices.windows(5) {
            let differences = sequence_key(w);
            if visited.insert(differences) {
                let total = totals.entry(differences).or_default();
                *total += w[4] as i64;
            }
        }
    }
    let result = totals.values().max().copied().unwrap_or_default() as usize;
    Ok(result)
}

fn sequence_key(a: &[i8]) -> (i8, i8, i8, i8) {
    (a[1] - a[0], a[2] - a[1], a[3] - a[2], a[4] - a[3])
}

fn prices_for_number(num: i64) -> impl Iterator<Item = i8> {
    iter::once((num % 10) as i8).chain(random_sequence(num).map(|n| (n % 10) as i8))
}

fn number_after_steps(num: i64, steps: usize) -> i64 {
    random_sequence(num).nth(steps).unwrap()
}

fn random_sequence(num: i64) -> impl Iterator<Item = i64> {
    iter::successors(Some(num), |n| Some(next_number(*n)))
}

fn next_number(num: i64) -> i64 {
    let num = (num ^ (num << 6)) & 16777215;
    let num = (num ^ (num >> 5)) & 16777215;
    (num ^ (num << 11)) & 16777215
}

fn parse_input(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, numbers) = separated_list0(line_ending, complete::i64)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = eof(input)?;

    Ok((input, numbers))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"1
10
100
2024"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        assert_eq!(result, 37327623);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let input = r#"1
2
3
2024"#;
        let result = part2(input)?;
        assert_eq!(result, 23);
        Ok(())
    }

    #[test]
    fn test_123() -> anyhow::Result<()> {
        let result = number_after_steps(123, 10);
        assert_eq!(result, 5908254);
        Ok(())
    }
}
