use std::fs;
use std::ops::Range;

use anyhow::{anyhow, Context};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, line_ending, space1};
use nom::multi::{many1, separated_list0};
use nom::sequence::{delimited, preceded};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let filename = "day-5.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i64> {
    let category_mappings = parse_input(input)?;
    let result = category_mappings
        .seeds
        .iter()
        .map(|value| category_mappings.location_value(*value))
        .min()
        .unwrap_or_default();
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<i64> {
    let category_mappings = parse_input(input)?;
    let seed_ranges: Vec<Range<i64>> = category_mappings
        .seeds
        .chunks(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect();
    let mut results: Vec<i64> = Vec::new();

    for seed_range in seed_ranges.into_iter() {
        let mut ranges = vec![seed_range];
        for mapping in category_mappings.category_mappings.iter() {
            ranges = mapping.calculate_ranges(&ranges);
        }
        results.push(
            ranges
                .into_iter()
                .map(|r| r.start)
                .min()
                .unwrap_or_default(),
        );
    }
    Ok(results.iter().min().copied().unwrap())
}

#[derive(Debug)]
struct Almanach {
    seeds: Vec<i64>,
    category_mappings: Vec<CategoryMapping>,
}

impl Almanach {
    fn location_value(&self, value: i64) -> i64 {
        let result = self.category_mappings.iter().fold(value, |v, m| {
            
            m.range_value(v)
        });
        result
    }
}

#[derive(Debug)]
struct CategoryMapping {
    source: String,
    destination: String,
    mappings: Vec<Mapping>,
}

impl CategoryMapping {
    fn calculate_ranges(&self, ranges: &[Range<i64>]) -> Vec<Range<i64>> {
        let mut result = Vec::new();
        let mut ranges = ranges.to_vec();
        let mut next_ranges = Vec::new();

        for mapping in self.mappings.iter() {
            let source = mapping.source_start..(mapping.source_start + mapping.range_length);
            while let Some(range) = ranges.pop() {
                let before = range.start..source.start.min(range.end);
                let between = range.start.max(source.start)..source.end.min(range.end);
                let after = range.start.max(source.end)..range.end;

                if before.start < before.end {
                    next_ranges.push(before);
                }

                if between.start < between.end {
                    let next_start = between.start + mapping.dest_start - source.start;
                    let next_end = between.end + mapping.dest_start - source.start;
                    result.push(next_start..next_end);
                }

                if after.start < after.end {
                    next_ranges.push(after);
                }
            }
            ranges.append(&mut next_ranges);
        }
        result.append(&mut ranges);
        result
    }

    fn range_value(&self, value: i64) -> i64 {
        self.mappings
            .iter()
            .find_map(|m| m.range_value(value))
            .unwrap_or(value)
    }
}

#[derive(Debug)]
struct Mapping {
    dest_start: i64,
    source_start: i64,
    range_length: i64,
}

impl Mapping {
    fn range_value(&self, value: i64) -> Option<i64> {
        if (self.source_start..self.source_start + self.range_length).contains(&value) {
            Some(self.dest_start + (value - self.source_start))
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> anyhow::Result<Almanach> {
    let (_, category_mappings) =
        seed_category_mappings(input).map_err(|e| anyhow!(e.to_string()))?;
    Ok(category_mappings)
}

fn seed_category_mappings(input: &str) -> IResult<&str, Almanach> {
    let (input, seeds) = delimited(
        preceded(tag("seeds:"), space1),
        separated_list0(space1, complete::i64),
        many1(line_ending),
    )(input)?;
    let (input, category_mappings) = separated_list0(many1(line_ending), category_mapping)(input)?;
    Ok((
        input,
        Almanach {
            seeds,
            category_mappings,
        },
    ))
}

fn category_mapping(input: &str) -> IResult<&str, CategoryMapping> {
    let (input, source) = alpha1(input)?;
    let (input, _) = tag("-to-")(input)?;
    let (input, destination) = alpha1(input)?;
    let (input, _) = preceded(space1, tag("map:"))(input)?;
    let (input, _) = line_ending(input)?;
    let (input, mappings) = separated_list0(line_ending, mapping)(input)?;

    Ok((
        input,
        CategoryMapping {
            source: source.to_string(),
            destination: destination.to_string(),
            mappings,
        },
    ))
}

fn mapping(input: &str) -> IResult<&str, Mapping> {
    let (input, destination_range_start) = complete::i64(input)?;
    let (input, _) = space1(input)?;
    let (input, source_range_start) = complete::i64(input)?;
    let (input, _) = space1(input)?;
    let (input, range_length) = complete::i64(input)?;

    Ok((
        input,
        Mapping {
            dest_start: destination_range_start,
            source_start: source_range_start,
            range_length,
        },
    ))
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
        let m = Mapping {
            dest_start: 50,
            source_start: 98,
            range_length: 2,
        };
        assert_eq!(m.range_value(98), Some(50));
        assert_eq!(m.range_value(99), Some(51));
        assert_eq!(m.range_value(100), None);
        assert_eq!(m.range_value(97), None);
    }
}
