use anyhow::Context;
use std::fs;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let input = "input.txt";
    let lines = fs::read_to_string(input).with_context(|| format!("cannot load {input}"))?;
    let result = part1(&lines)?;
    println!("{result}");
    let result = part2(&lines)?;
    println!("{result}");
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct Seat {
    row: u32,
    column: u32,
}

impl FromStr for Seat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row = s
            .chars()
            .filter(|&c| c == 'F' || c == 'B')
            .fold(0, |r, c| (r << 1) + if c == 'B' { 1 } else { 0 });
        let column = s
            .chars()
            .filter(|&c| c == 'L' || c == 'R')
            .fold(0, |r, c| (r << 1) + if c == 'R' { 1 } else { 0 });
        Ok(Seat { row, column })
    }
}

impl Seat {
    fn id(&self) -> u32 {
        self.row * 8 + self.column
    }
}

fn part1(input: &str) -> anyhow::Result<u32> {
    let ids: anyhow::Result<Vec<u32>> = input
        .lines()
        .map(|line| line.parse::<Seat>())
        .map(|seat| seat.map(|r| r.id()))
        .collect();
    let ids = ids?;
    Ok(ids.into_iter().max().unwrap_or(0))
}

fn part2(input: &str) -> anyhow::Result<u32> {
    let ids: anyhow::Result<Vec<u32>> = input
        .lines()
        .map(|line| line.parse::<Seat>())
        .map(|seat| seat.map(|r| r.id()))
        .collect();
    let mut ids = ids?;
    ids.sort();
    Ok(ids
        .iter()
        .as_slice()
        .windows(2)
        // w[0] = the first seat with missing successor
        .filter(|w| w[1] != w[0] + 1)
        .map(|w| w[0] + 1)
        .next()
        .unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL"#;

    #[test]
    fn seat_from_str_works() -> anyhow::Result<()> {
        let input = "BFFFBBFRRR";
        let seat = input.parse::<Seat>()?;
        let expected = Seat { row: 70, column: 7 };
        assert_eq!(seat, expected);
        Ok(())
    }

    #[test]
    fn seat_id_works() -> anyhow::Result<()> {
        let input = Seat { row: 70, column: 7 };
        let expected = 567;
        let result = input.id();
        assert_eq!(result, expected);
        Ok(())
    }
    fn part1_works() -> anyhow::Result<()> {
        Ok(())
    }
}
