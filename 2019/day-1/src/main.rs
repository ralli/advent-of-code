use std::fs;
use std::num::ParseIntError;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let s = read_file("day-1/day-1.txt")?;
    let values: Vec<i64> = parse_values(&s)?;
    let result: i64 = values.iter().map(|x| x / 3 - 2).sum();
    println!("{result}");
    let result = part2(&values);
    println!("{result}");
    Ok(())
}

fn part2(values: &[i64]) -> i64 {
    values.iter().map(|&x| fuel_requirement(x)).sum()
}

fn fuel_requirement(value: i64) -> i64 {
    let mut x = value;
    let mut result = 0;
    while x > 0 {
        x = x / 3 - 2;
        if x > 0 {
            result += x;
        }
    }
    result
}

fn parse_values(input: &str) -> anyhow::Result<Vec<i64>> {
    let values: Result<Vec<i64>, ParseIntError> = input.lines().map(|l| l.parse::<i64>()).collect();
    let values = values?;
    Ok(values)
}

fn read_file(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let s = fs::read_to_string(&path)?;
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_requirement() {
        assert_eq!(fuel_requirement(1969), 966);
        assert_eq!(fuel_requirement(100756), 50346);
    }
}