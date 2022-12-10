use std::fs::File;
use std::io::Read;

use anyhow::*;
use nom::character::complete::line_ending;
use nom::IResult;
use nom::multi::separated_list1;
use std::slice::*;

fn main() -> anyhow::Result<()> {
    let filename = "./day-1/input.txt";
    let content = read_file(filename).context(filename)?;

    let result = part1(&content);
    println!("{}", result);

    let result = part2(&content);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> usize {
    let (_, values) = values(input).unwrap();
    values.windows(2).filter(|w| w[0] < w[1]).count()
}

fn part2(input: &str) -> usize {
    let (_, values) = values(input).unwrap();
    let bla = values.windows(3).map(|w| w.iter().sum()).collect::<Vec<u32>>();
    bla.windows(2).filter(|w| w[0] < w[1]).count()
}


fn values(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(line_ending, nom::character::complete::u32)(input)
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

    const INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 7;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 5;
        assert_eq!(result, expected);
    }
}
