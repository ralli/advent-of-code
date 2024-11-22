extern crate core;

use anyhow::Context;
use core::fmt;
use std::fs;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let content = read_file("day-8/day-8.txt")?;
    let result = part1(&content)?;

    println!("{result}");

    part2(&content, 25, 6)?;

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let pixels = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();

    let (layer, _) = pixels
        .chunks(25 * 6)
        .map(|layer| {
            let zero_count = layer.iter().filter(|p| **p == 0).count();
            (layer, zero_count)
        })
        .min_by(|(l1, count1), (l2, count2)| count1.cmp(count2))
        .unwrap();

    let num_ones = layer.iter().filter(|p| **p == 1).count();
    let num_twos = layer.iter().filter(|p| **p == 2).count();

    Ok(num_ones * num_twos)
}

fn part2(input: &str, width: usize, height: usize) -> anyhow::Result<()> {
    let pixels = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    let size = width * height;
    let mut img = vec![2u32; size];

    for layer in pixels.chunks(size).rev() {
        for i in 0..size {
            let p1 = img[i];
            let p2 = layer[i];
            img[i] = if p2 == 2 { p1 } else { p2 }
        }
    }

    for i in 0..height {
        for j in 0..width {
            print!("{}", if img[i * width + j] == 0 { ' ' } else { '#' });
        }
        println!();
    }
    Ok(())
}

fn read_file(filename: impl AsRef<Path> + fmt::Display) -> anyhow::Result<String> {
    fs::read_to_string(filename.as_ref()).with_context(|| format!("cannot read file {filename}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = r#"0222112222120000"#;
        part2(&input.to_string(), 2, 2).unwrap();
    }
}
