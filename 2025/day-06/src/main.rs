use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day-06.txt")?;
    let result = part1(&input)?;
    println!("{result}");
    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add,
    Multiply,
}

fn part1(input: &str) -> anyhow::Result<u64> {
    let mut lines = input.lines().collect::<Vec<_>>();
    let ops = lines.pop().ok_or_else(|| anyhow!("empty input"))?;
    let numbers: Vec<_> = lines
        .iter()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
        .collect();
    let ops = ops
        .split_ascii_whitespace()
        .map(|s| match s {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    let height = numbers.len();
    let width = numbers.first().map(|row| row.len()).unwrap_or(0);
    let mut result = 0;
    for col in 0..width {
        let op = ops[col];
        let mut res = match op {
            Operation::Add => 0,
            Operation::Multiply => 1,
        };
        for row in 0..height {
            let value = numbers[row][col].parse::<u64>()?;
            match op {
                Operation::Add => res += value,
                Operation::Multiply => res *= value,
            }
        }
        result += res;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn test_part1() {
        let result = part1(INPUT).unwrap();
        assert_eq!(result, 4277556);
    }
}
