use std::ops::RangeInclusive;

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let content = read_file("./day-4/input.txt")?;

    let part1_result = part1(&content).context("part1")?;
    println!("{}", part1_result);

    let part2_result = part2(&content).context("part2")?;
    println!("{}", part2_result);

    Ok(())
}

fn part1(content: &str) -> anyhow::Result<i32> {
    let range_lists = range_lists(content)?;
    let mut count = 0;
    for range_list in range_lists.iter() {
        if has_full_overlap(&range_list[0], &range_list[1]) {
            count += 1;
        }
    }
    Ok(count)
}

fn part2(content: &str) -> anyhow::Result<i32> {
    let range_lists = range_lists(content)?;
    let mut count = 0;
    for range_list in range_lists.iter() {
        if has_partial_overlap(&range_list[0], &range_list[1]) {
            count += 1;
        }
    }
    Ok(count)
}

fn has_partial_overlap(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    a.start() <= b.end() && a.end() >= b.start()
}

fn has_full_overlap(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    is_contained_in(a, b) || is_contained_in(b, a)
}

fn is_contained_in(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    a.start() <= b.start() && a.end() >= b.end()
}

fn range_lists(content: &str) -> anyhow::Result<Vec<Vec<RangeInclusive<i32>>>> {
    content
        .lines()
        .enumerate()
        .map(|(line_no, line)| ranges_from_line(line).context(line_no))
        .collect()
}

fn ranges_from_line(line: &str) -> Result<Vec<RangeInclusive<i32>>, anyhow::Error> {
    let range_parts = line.split(',');
    let ranges = range_parts.map(|range_part| {
        let mut num_parts = range_part.split('-');
        let start = num_parts
            .next()
            .unwrap_or_default()
            .trim()
            .parse::<i32>()
            .context("start")?;
        let end = num_parts
            .next()
            .unwrap_or_default()
            .trim()
            .parse::<i32>()
            .context("end")?;
        Ok(start..=end)
    });
    ranges.collect()
}

fn read_file(filename: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ranges_from_line() {
        let input = "2-4,6-8\n";
        let result = ranges_from_line(input);
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn test_part1() {
        let input = r#"2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8"#;

        let result = part1(input);
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_part2() {
        let input = r#"2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8"#;

        let result = part2(input);
        assert_eq!(result.unwrap(), 4);
    }
}
