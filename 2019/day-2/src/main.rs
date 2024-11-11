use anyhow::{anyhow, Context};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list0;
use nom::IResult;
use std::path::Path;
use std::fs;

fn main() -> anyhow::Result<()> {
    let filename = "day-2/day-2.txt";
    let input = read_file(filename).with_context(|| format!("cannot load {filename}"))?;
    let result = part1(&input)?;
    println!("{result}");
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i32> {
    let (_, mut values) = parse_input(input).map_err(|e| anyhow!(e.to_string()))?;
    let size = values.len();
    for i in (0..size).step_by(4) {
        let arr = &values[i..i + 4];
        println!("{arr:?}");
        if i + 3 >= values.len() {
            break;
        }
        assert_eq!(arr.len(), 4);
        let opcode = arr[0];
        let a = arr[1];
        let b = arr[2];
        let c = arr[3] as usize;
        match opcode {
            1 => {
                values[c] = a + b;
            }
            2 => {
                values[c] = a * b;
            }
            99 => {
                break;
            }
            _ => unreachable!()
        }
        println!("{opcode} {a} {b} {c} {values:?}");
    }
    Ok(values[0])
}

fn parse_input(input: &str) -> IResult<&str, Vec<i32>> {
    let (rest, values): (&str, Vec<i32>) = separated_list0(tag(","), complete::i32)(input)?;
    Ok((rest, values))
}

fn read_file(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let s = fs::read_to_string(&path)?;
    Ok(s)
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";
        let result = part1(input)?;
        assert_eq!(result, 3500);
        Ok(())
    }
}