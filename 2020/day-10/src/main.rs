use std::collections::BTreeMap;
use std::fs;

use anyhow::Context;
use nom::character::complete;
use nom::character::complete::multispace1;
use nom::combinator::{all_consuming, opt};
use nom::multi::separated_list0;
use nom::sequence::terminated;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let filename = "day-10.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;
    let result = part1(&input)?;
    println!("{result}");
    let result = part2(&input)?;
    println!("{result}");
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i32> {
    let (_, mut numbers) = parse_input(input).map_err(|e| anyhow::Error::msg(e.to_string()))?;
    numbers.sort();
    numbers.insert(0, 0);
    numbers.push(numbers.last().unwrap() + 3);

    let n1 = numbers
        .as_slice()
        .windows(2)
        .map(|w| w[1] - w[0])
        .filter(|&n| n == 1)
        .count();
    let n3 = numbers
        .as_slice()
        .windows(2)
        .map(|w| w[1] - w[0])
        .filter(|&n| n == 3)
        .count();
    Ok((n1 * n3) as i32)
}

fn part2(input: &str) -> anyhow::Result<i64> {
    let (_, mut numbers) = parse_input(input).map_err(|e| anyhow::Error::msg(e.to_string()))?;
    numbers.sort();
    numbers.insert(0, 0);
    numbers.push(numbers.last().unwrap() + 3);

    fn number_of_combinations(i: usize, numbers: &[i32], memo: &mut BTreeMap<usize, i64>) -> i64 {
        if let Some(result) = memo.get(&i) {
            return *result;
        }
        let size = numbers.len();
        if i + 1 == size {
            return 1;
        }
        let current = numbers[i];
        let n1 = if i + 1 < size && numbers[i + 1] - current <= 3 {
            number_of_combinations(i + 1, numbers, memo)
        } else {
            0
        };
        let n2 = if i + 2 < size && numbers[i + 2] - current <= 3 {
            number_of_combinations(i + 2, numbers, memo)
        } else {
            0
        };
        let n3 = if i + 3 < size && numbers[i + 3] - current <= 3 {
            number_of_combinations(i + 3, numbers, memo)
        } else {
            0
        };
        let result = n1 + n2 + n3;
        memo.insert(i, result);
        result
    }
    let mut memo = BTreeMap::new();
    let result = number_of_combinations(0, &numbers, &mut memo);
    Ok(result)
}

fn parse_input(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, numbers) = all_consuming(terminated(
        separated_list0(multispace1, complete::i32),
        opt(multispace1),
    ))(input)?;
    Ok((input, numbers))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"16
10
15
5
1
11
7
19
6
12
4"#;

    static INPUT2: &str = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 35;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 8;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_long_input() -> anyhow::Result<()> {
        let result = part2(INPUT2)?;
        let expected = 19208;
        assert_eq!(result, expected);
        Ok(())
    }
}
