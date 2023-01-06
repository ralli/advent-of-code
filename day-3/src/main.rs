use std::fs::File;
use std::io::Read;

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let filename = "./day-3/input.txt";
    let content = read_file(filename).context(filename)?;

    let result = part1(&content);
    println!("{}", result);

    let result = part2(&content);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> u32 {
    let numbers = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut gamma = Vec::new();
    let len = numbers.iter().next().unwrap().len();
    for i in 0..len {
        gamma.push(most_common_bit(&numbers, i));
    }

    let g = to_binary(&gamma);
    let e = gamma
        .iter()
        .map(|&c| if c == '1' { 0 } else { 1 })
        .fold(0, |n, d| 2 * n + d);

    e * g
}

fn most_common_bit(numbers: &Vec<Vec<char>>, idx: usize) -> char {
    let one_count = numbers.iter().filter(|n| n[idx] == '1').count();
    let zero_count = numbers.len() - one_count;
    if one_count >= zero_count {
        '1'
    } else {
        '0'
    }
}

fn part2(input: &str) -> u32 {
    let numbers = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let len = numbers.iter().next().unwrap().len();
    let mut oxigen_rating = numbers.iter().map(|v| v.clone()).collect::<Vec<_>>();
    let mut co2_rating = numbers.iter().map(|v| v.clone()).collect::<Vec<_>>();

    for i in 0..len {
        if oxigen_rating.len() > 1 {
            let mbit = most_common_bit(&oxigen_rating, i);
            oxigen_rating = oxigen_rating
                .into_iter()
                .filter(|v| v[i] == mbit)
                .collect::<Vec<_>>();
        }
        if co2_rating.len() > 1 {
            let mbit = most_common_bit(&co2_rating, i);
            co2_rating = co2_rating
                .into_iter()
                .filter(|v| v[i] != mbit)
                .collect::<Vec<_>>();
        }
    }

    let o = to_binary(oxigen_rating.iter().next().unwrap());
    let c = to_binary(co2_rating.iter().next().unwrap());

    o * c
}

fn to_binary(v: &[char]) -> u32 {
    v.iter()
        .map(|&c| if c == '1' { 1 } else { 0 })
        .fold(0, |n, d| 2 * n + d)
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

    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 198;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 230;
        assert_eq!(result, expected);
    }
}
