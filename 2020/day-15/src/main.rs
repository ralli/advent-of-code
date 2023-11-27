use anyhow::anyhow;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list0;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let input = r#"0,12,6,13,20,1,17"#;
    let result = part1(input)?;
    println!("{result}");
    let result = part2(input)?;
    println!("{result}");
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i32> {
    let (_, numbers) = parse_input(input).map_err(|e| anyhow!(e.to_string()))?;
    let last_spoken = game(&numbers, 2020);
    Ok(last_spoken)
}

fn part2(input: &str) -> anyhow::Result<i32> {
    let (_, numbers) = parse_input(input).map_err(|e| anyhow!(e.to_string()))?;
    let last_spoken = game(&numbers, 30000000);
    Ok(last_spoken)
}

fn game(numbers: &[i32], num_rounds: usize) -> i32 {
    let mut last_visit: Vec<usize> = vec![0; num_rounds];
    let mut visited: Vec<bool> = vec![false; num_rounds];
    let mut last_number = 0;
    let mut next_number = 0;

    for round in 0..num_rounds {
        next_number = if round < numbers.len() {
            numbers[round]
        } else if !visited[last_number] {
            0
        } else {
            (round - last_visit[last_number]) as i32
        };
        if round != 0 {
            last_visit[last_number] = round;
            visited[last_number] = true;
        }
        last_number = next_number as usize;
    }
    next_number
}

fn parse_input(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list0(tag(","), complete::i32)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let input = "0,3,6";
        let result = part1(input)?;
        let expected = 436;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let input = "0,3,6";
        let result = part2(input)?;
        let expected = 175594;
        assert_eq!(result, expected);
        Ok(())
    }
}
