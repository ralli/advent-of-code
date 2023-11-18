use std::fs;

use anyhow::{anyhow, Context};

fn main() -> anyhow::Result<()> {
    let filename = "day-9.txt";
    let input =
        fs::read_to_string(filename).with_context(|| format!("cannot load {}", filename))?;
    let result = part1(&input)?;
    println!("{result}");
    let result = part2(&input)?;
    println!("{result}");
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i64> {
    let numbers = parse_numbers(input)?;
    first_mismatch(&numbers, 25).ok_or_else(|| anyhow!("no solution found"))
}

fn part2(input: &str) -> anyhow::Result<i64> {
    let numbers = parse_numbers(input)?;
    let mismatch = first_mismatch(&numbers, 25).ok_or_else(|| anyhow!("no mismatch found"))?;
    let result = find_sum(&numbers, mismatch).ok_or_else(|| anyhow!("no solution found"))?;
    Ok(result)
}

fn parse_numbers(input: &str) -> anyhow::Result<Vec<i64>> {
    input
        .lines()
        .map(|line| line.parse::<i64>().map_err(Into::into))
        .collect()
}
fn first_mismatch(numbers: &[i64], window_size: usize) -> Option<i64> {
    for w in numbers.windows(window_size + 1) {
        let goal = w.last().unwrap();
        let bla = &w[0..window_size];
        let m = bla.iter().filter(|x| bla.contains(&(*goal - *x))).next();
        if m.is_none() {
            // dbg!(goal, bla);
            return Some(*goal);
        }
    }
    None
}

fn find_sum(numbers: &[i64], goal: i64) -> Option<i64> {
    let size = numbers
        .iter()
        .enumerate()
        .filter(|(_i, &v)| v == goal)
        .next()
        .map(|(i, _v)| i)?;

    for i in 0..size {
        for j in (i + 1)..size {
            let r = &numbers[i..j];
            let x: i64 = r.iter().sum();
            if x == goal {
                let a = r.iter().min().copied().unwrap();
                let b = r.iter().max().copied().unwrap();
                // dbg!(&numbers[i..j], x, goal, a, b);
                return Some(a + b);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#;

    #[test]
    fn first_mismatch_works() -> anyhow::Result<()> {
        let numbers = parse_numbers(INPUT)?;
        let mismatch = first_mismatch(&numbers, 5).unwrap();
        let expected = 127;
        assert_eq!(mismatch, expected);
        Ok(())
    }

    #[test]
    fn find_sum_works() -> anyhow::Result<()> {
        let numbers = parse_numbers(INPUT)?;
        let mismatch = 127;
        let result = find_sum(&numbers, mismatch).unwrap();
        let expected = 62;
        assert_eq!(result, expected);
        Ok(())
    }
}
