use std::fs;

use anyhow::{anyhow, Context};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, line_ending, space1};
use nom::IResult;
use nom::multi::{many1, separated_list0};
use nom::sequence::{delimited, preceded};
use rayon::prelude::*;

fn main() -> anyhow::Result<()> {
    let filename = "day-5.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<u64> {
    let category_mappings = parse_input(input)?;
    let result = category_mappings.seeds.iter().map(|value| category_mappings.location_value(*value)).min().unwrap_or_default();
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let category_mappings = parse_input(input)?;
    let mut result = u64::MAX;
    // brute force + multi-threading with rayon
    for v in category_mappings.seeds.chunks(2) {
        let from = v[0];
        let step = v[1];
        let to = from + step;
        result = (from..to).into_par_iter().map(|v| category_mappings.location_value(v)).min().unwrap().min(result);
    }
    Ok(result)
}

#[derive(Debug)]
struct SeedCategoryMappings {
    seeds: Vec<u64>,
    category_mappings: Vec<CategoryMapping>,
}

impl SeedCategoryMappings {
    fn range_location_value(&self, from: u64, to: u64) -> u64 {
        let value = self.category_mappings.first().unwrap().ranges_value(from, to);
        self.category_mappings.iter().skip(1).fold(value, |v, m| m.range_value(v))
    }

    fn location_value(&self, value: u64) -> u64 {
        self.category_mappings.iter().fold(value, |v, m| m.range_value(v))
    }
}

#[derive(Debug)]
struct CategoryMapping {
    source: String,
    destination: String,
    mappings: Vec<Mapping>,
}

impl CategoryMapping {
    fn ranges_value(&self, from: u64, to: u64) -> u64 {
        self.mappings.iter().find_map(|m| m.ranges_value(from, to)).unwrap()
    }

    fn range_value(&self, value: u64) -> u64 {
        self.mappings.iter().find_map(|m| m.range_value(value)).unwrap_or(value)
    }
}

#[derive(Debug)]
struct Mapping {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl Mapping {
    fn ranges_value(&self, from: u64, to: u64) -> Option<u64> {
        let bla = self.source_range_start..self.source_range_start + self.range_length;
        let value = if to >= bla.start && from < bla.end {
            if from < bla.start {
                Some(bla.start)
            } else {
                Some(from)
            }
        } else {
            None
        };
        value.and_then(|v| self.range_value(v))
    }

    fn range_value(&self, value: u64) -> Option<u64> {
        if (self.source_range_start..self.source_range_start + self.range_length).contains(&value) {
            return Some(self.destination_range_start + (value - self.source_range_start));
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> anyhow::Result<SeedCategoryMappings> {
    let (input, category_mappings) = seed_category_mappings(input).map_err(|e| anyhow!(e.to_string()))?;
    Ok(category_mappings)
}


fn seed_category_mappings(input: &str) -> IResult<&str, SeedCategoryMappings> {
    let (input, seeds) = delimited(preceded(tag("seeds:"), space1), separated_list0(space1, complete::u64), many1(line_ending))(input)?;
    let (input, category_mappings) = separated_list0(many1(line_ending), category_mapping)(input)?;
    Ok((input, SeedCategoryMappings { seeds, category_mappings }))
}

fn category_mapping(input: &str) -> IResult<&str, CategoryMapping> {
    let (input, source) = alpha1(input)?;
    let (input, _) = tag("-to-")(input)?;
    let (input, destination) = alpha1(input)?;
    let (input, _) = preceded(space1, tag("map:"))(input)?;
    let (input, _) = line_ending(input)?;
    let (input, mappings) = separated_list0(line_ending, mapping)(input)?;

    Ok((input, CategoryMapping {
        source: source.to_string(),
        destination: destination.to_string(),
        mappings,
    }))
}

fn mapping(input: &str) -> IResult<&str, Mapping> {
    let (input, destination_range_start) = complete::u64(input)?;
    let (input, _) = space1(input)?;
    let (input, source_range_start) = complete::u64(input)?;
    let (input, _) = space1(input)?;
    let (input, range_length) = complete::u64(input)?;

    Ok((input, Mapping {
        destination_range_start,
        source_range_start,
        range_length,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 35;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 46;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn range_value_works() {
        let m = Mapping { destination_range_start: 50, source_range_start: 98, range_length: 2 };
        assert_eq!(m.range_value(98), Some(50));
        assert_eq!(m.range_value(99), Some(51));
        assert_eq!(m.range_value(100), None);
        assert_eq!(m.range_value(97), None);
    }

    #[test]
    fn ranges_value_works() {
        let m = Mapping { destination_range_start: 52, source_range_start: 50, range_length: 48 };
        let from = 79;
        let to = 79 + 14 - 1;
        assert_eq!(m.ranges_value(from, to), Some(84));
    }
}