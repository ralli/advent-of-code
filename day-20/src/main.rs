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
    let (_, arr) = numbers(input).unwrap();
    let size = arr.len();
    let mut numbers: Vec<_> = arr.iter().copied().enumerate().collect();
    for (original_index, value) in arr.into_iter().enumerate() {
        let index = numbers
            .iter()
            .position(|(idx, _)| *idx == original_index)
            .unwrap();
        let new_index = index as i64 + value as i64;
        let new_index = new_index.rem_euclid(size as i64 - 1);
        let tmp = numbers.remove(index);
        numbers.insert(new_index as usize, tmp);
    }
    let zero_idx = numbers.iter().position(|(_, x)| *x == 0).unwrap();
    let (_, x1) = numbers[(zero_idx + 1_000) % size];
    let (_, x2) = numbers[(zero_idx + 2_000) % size];
    let (_, x3) = numbers[(zero_idx + 3_000) % size];
    x1 + x2 + x3
}

fn part2(input: &str) -> i64 {
    let num_rounds = 10;
    let key = 811_589_153;
    let (_, arr) = numbers(input).unwrap();
    let size = arr.len();
    let mut numbers: Vec<_> = arr.iter().copied().map(|x| x * key).enumerate().collect();
    for _ in 0..num_rounds {
        for (original_index, &value) in arr.iter().enumerate() {
            let index = numbers
                .iter()
                .position(|(idx, _)| *idx == original_index)
                .unwrap();
            let new_index = index as i64 + value;
            let new_index = new_index.rem_euclid(size as i64 - 1);
            let tmp = numbers.remove(index);
            numbers.insert(new_index as usize, tmp);
        }
        let bla: Vec<_> = numbers.iter().map(|(_, x)| x).collect();
        println!("{:?}", bla);
    }
    let zero_idx = numbers.iter().position(|(_, x)| *x == 0).unwrap();
    let (_, x1) = numbers[(zero_idx + 1_000) % size];
    let (_, x2) = numbers[(zero_idx + 2_000) % size];
    let (_, x3) = numbers[(zero_idx + 3_000) % size];
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
        let expected = 811589153i64;
        assert_eq!(result, expected);
    }
}
