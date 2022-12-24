use nom::character::complete;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let input = read_file("./day-20/input.txt")?;

    let result = part1(&input);
    println!("{}", result);

    let result = part2(&input);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> i64 {
    let (_, numbers) = numbers(input).unwrap();
    let mut pairs: Vec<_> = numbers.iter().copied().enumerate().collect();
    let size = numbers.len();

    mix(&numbers, &mut pairs);

    let zero_idx = pairs.iter().position(|(_, x)| *x == 0).unwrap();
    let (_, x1) = pairs[(zero_idx + 1_000) % size];
    let (_, x2) = pairs[(zero_idx + 2_000) % size];
    let (_, x3) = pairs[(zero_idx + 3_000) % size];

    x1 + x2 + x3
}

fn mix(numbers: &[i64], pairs: &mut Vec<(usize, i64)>) {
    for (idx, &value) in numbers.iter().enumerate() {
        if value == 0 {
            continue;
        }
        let current_idx = pairs.iter().position(|(i, _)| *i == idx).unwrap();
        let tmp = pairs.remove(current_idx);
        let new_idx = (current_idx as i64 + value).rem_euclid(pairs.len() as i64);
        pairs.insert(new_idx as usize, tmp);
    }
}

fn part2(input: &str) -> i64 {
    let num_rounds = 10;
    let key = 811_589_153;
    let (_, arr) = numbers(input).unwrap();
    let size = arr.len();
    let numbers: Vec<_> = arr.into_iter().map(|v| v * key).collect();
    let mut pairs: Vec<_> = numbers.iter().copied().enumerate().collect();

    for _ in 0..num_rounds {
        mix(&numbers, &mut pairs);
    }

    let zero_idx = pairs.iter().position(|(_, x)| *x == 0).unwrap();
    let (_, x1) = pairs[(zero_idx + 1_000) % size];
    let (_, x2) = pairs[(zero_idx + 2_000) % size];
    let (_, x3) = pairs[(zero_idx + 3_000) % size];
    x1 + x2 + x3
}

fn numbers(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(line_ending, complete::i64)(input)
}

fn read_file(filename: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 1623178306i64;
        assert_eq!(result, expected);
    }
}
