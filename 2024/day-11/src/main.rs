use anyhow::Context;
use std::collections::BTreeMap;
use std::fs;

fn main() -> anyhow::Result<()> {
    let filename = "day-11/input.txt";
    let content = fs::read_to_string(filename).context(format!("cannot open file {filename}"))?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut numbers = parse_numbers(input)?;

    for _ in 0..25 {
        let mut arr = Vec::new();
        for &n in numbers.iter() {
            if n == 0 {
                arr.push(1);
            } else if num_digits(n) % 2 == 0 {
                let (a, b) = split(n);
                arr.push(a);
                arr.push(b);
            } else {
                arr.push(n * 2024);
            }
        }
        numbers = arr;
    }
    Ok(numbers.len())
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let numbers = parse_numbers(input)?;
    Ok(get_stone_count(&numbers, 75))
}

fn get_stone_count(numbers: &[i64], n: i64) -> usize {
    fn count(x: i64, n: i64, cache: &mut BTreeMap<(i64, i64), usize>) -> usize {
        if n == 0 {
            return 1;
        }

        if let Some(y) = cache.get(&((x, n))) {
            return *y;
        }

        let result = if x == 0 {
            count(1, n - 1, cache)
        } else if num_digits(x) % 2 == 0 {
            let (a, b) = split(x);
            count(a, n - 1, cache) + count(b, n - 1, cache)
        } else {
            count(x * 2024, n - 1, cache)
        };

        cache.insert((x, n), result);

        result
    }

    let mut cache = BTreeMap::new();
    let mut result = 0;
    for x in numbers.iter() {
        result += count(*x, n, &mut cache)
    }

    result
}

fn split(n: i64) -> (i64, i64) {
    let digits = num_digits(n);
    let mut m = 1;
    for _ in 0..(digits / 2) {
        m *= 10;
    }
    (n / m, n % m)
}

fn num_digits(n: i64) -> i64 {
    n.ilog10() as i64 + 1
    // let mut n = n;
    // let mut digits = 0;
    // while n > 0 {
    //     digits += 1;
    //     n /= 10;
    // }
    // digits
}

fn parse_numbers(input: &str) -> anyhow::Result<Vec<i64>> {
    let numbers: Result<Vec<i64>, _> = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>())
        .collect();
    let numbers = numbers?;
    Ok(numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let input = r#"125 17"#;
        let result = part1(input)?;
        assert_eq!(result, 55312);
        Ok(())
    }
    #[test]
    fn test_get_stone_count() {
        let numbers = [125, 17];
        assert_eq!(get_stone_count(&numbers, 75), 65601038650482);
    }

    #[test]
    fn test_num_digits() {
        assert_eq!(num_digits(1234), 4);
    }

    #[test]
    fn test_split() {
        assert_eq!(split(12), (1, 2));
        assert_eq!(split(123), (12, 3));
        assert_eq!(split(1234), (12, 34));
    }
}
