use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

use anyhow::Context;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let filename = "./day-6/input.txt";
    let content = read_file(filename).context(filename)?;

    let result = part1(&content);
    println!("{}", result);

    let result = part2(&content);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> usize {
    let (_, mut fishes) = fishes(input).unwrap();
    let num_days = 80;

    for _ in 0..num_days {
        let count = fishes.iter().filter(|&&v| v == 0).count();
        for v in fishes.iter_mut() {
            if *v == 0 {
                *v = 7;
            }
        }
        for v in fishes.iter_mut() {
            *v -= 1;
        }
        for _ in 0..count {
            fishes.push(8);
        }
    }
    fishes.len()
}

fn part2(input: &str) -> u64 {
    let (_, mut fishes) = fishes(input).unwrap();
    let mut fish_map: BTreeMap<i32, u64> = BTreeMap::new();

    for &fish in fishes.iter() {
        *fish_map.entry(fish).or_insert(0) += 1;
    }
    let num_days = 256;

    for _ in 0..num_days {
        let count = *fish_map.entry(0).or_insert(0);
        *fish_map.entry(7).or_insert(0) += count;

        for i in 1..=8 {
            let next_entry = fish_map.get(&i).copied().unwrap_or_default();
            let entry = fish_map.entry(i - 1).or_insert(0);
            *entry = next_entry;
        }

        *fish_map.entry(8).or_insert(0) = count;
    }

    fish_map.values().copied().sum()
}

fn fishes(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(tag(","), nom::character::complete::i32)(input)
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

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 5934;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 26984457539u64;
        assert_eq!(result, expected);
    }
}
