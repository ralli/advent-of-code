use std::collections::BTreeMap;
use std::fs;

use anyhow::{anyhow, Context};
use nom::branch::alt;
use nom::character::complete;
use nom::character::complete::{char, line_ending, space1};
use nom::IResult;
use nom::multi::{many1, separated_list0, separated_list1};
use nom::Parser;
use rayon::prelude::*;

fn main() -> anyhow::Result<()> {
    let filename = "day-12.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot load {filename}"))?;

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i64> {
    let records = parse_input(input)?;
    let result = records.into_par_iter().map(|r| count_arrangements(&r.spring_states, &r.group_counts)).sum();
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<i64> {
    let records = parse_input(input)?;
    let result = records.into_par_iter().map(|r| count_arrangements2(&r.spring_states, &r.group_counts)).sum();
    Ok(result)
}

fn count_arrangements2(spring_states: &[SpringState], group_counts: &[i64]) -> i64 {
    let mut s = spring_states.to_vec();
    for _i in 0..4 {
        s.push(SpringState::Unknown);
        s.extend(spring_states.iter().copied());
    }
    assert_eq!(s.len(), 5 * spring_states.len() + 4);
    let mut g = group_counts.to_vec();
    for _i in 0..4 {
        g.extend(group_counts.iter().copied());
    }

    let result = count_arrangements(&s, &g);
    // println!("...{result}");
    result
}

fn count_arrangements(spring_states: &[SpringState], group_counts: &[i64]) -> i64 {
    fn count_arrangements_memo<'a>(spring_states: &'a [SpringState], current_count: i64, group_counts: &'a [i64], memo: &mut BTreeMap<(&'a [SpringState], i64, &'a [i64]), i64>) -> i64 {
        if let Some(result) = memo.get(&(spring_states, current_count, group_counts)) {
            return *result;
        }
        // println!("{spring_states:?} {current_count} {group_counts:?}");
        if spring_states.is_empty() && group_counts.is_empty() {
            // println!("found!");
            memo.insert((spring_states, current_count, group_counts), 1);
            return 1;
        }
        if group_counts.is_empty() {
            return if current_count == 0 && spring_states.iter().all(|&s| s == SpringState::Operational || s == SpringState::Unknown) {
                // println!("found!");
                memo.insert((spring_states, current_count, group_counts), 1);
                1
            } else {
                // println!("FALSE!");
                memo.insert((spring_states, current_count, group_counts), 0);
                0
            };
        }

        let first_count = group_counts[0];
        if first_count < current_count {
            // println!("FALSE!");
            memo.insert((spring_states, current_count, group_counts), 0);
            return 0;
        }
        if spring_states.is_empty() {
            return if current_count == first_count {
                let result = count_arrangements_memo(spring_states, 0, &group_counts[1..], memo);
                memo.insert((spring_states, current_count, group_counts), result);
                result
            } else {
                // println!("FALSE!");
                memo.insert((spring_states, current_count, group_counts), 0);
                0
            };
        }

        let first_state = spring_states[0];

        let result = match first_state {
            SpringState::Operational => {
                if current_count == first_count {
                    count_arrangements_memo(&spring_states[1..], 0, &group_counts[1..], memo)
                } else if current_count > 0 {
                    // println!("FALSE!");
                    0
                } else {
                    count_arrangements_memo(&spring_states[1..], 0, group_counts, memo)
                }
            }
            SpringState::Damaged => {
                let result = count_arrangements_memo(&spring_states[1..], current_count + 1, group_counts, memo);
                result
            }
            SpringState::Unknown => {
                let damaged = count_arrangements_memo(&spring_states[1..], current_count + 1, group_counts, memo);
                let operational = if current_count == first_count {
                    count_arrangements_memo(&spring_states[1..], 0, &group_counts[1..], memo)
                } else if current_count > 0 {
                    // println!("FALSE!");damaged
                    0
                } else {
                    count_arrangements_memo(&spring_states[1..], 0, group_counts, memo)
                };
                damaged + operational
            }
        };
        memo.insert((spring_states, current_count, group_counts), result);
        result
    }
    let mut memo: BTreeMap<(&[SpringState], i64, &[i64]), i64> = BTreeMap::new();
    count_arrangements_memo(spring_states, 0, group_counts, &mut memo)
}

#[derive(Debug)]
struct ConditionRecord {
    spring_states: Vec<SpringState>,
    group_counts: Vec<i64>,
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

fn parse_input(input: &str) -> anyhow::Result<Vec<ConditionRecord>> {
    let (_, condition_records) = separated_list0(line_ending, parse_condition_record)(input).map_err(|e| anyhow!(e.to_owned()))?;
    Ok(condition_records)
}

fn parse_condition_record(input: &str) -> IResult<&str, ConditionRecord> {
    let (input, spring_states) = many1(parse_spring_state)(input)?;
    let (input, _) = space1(input)?;
    let (input, group_counts) = separated_list1(char(','), complete::i64)(input)?;
    Ok((input, ConditionRecord {
        spring_states,
        group_counts,
    }))
}

fn parse_spring_state(input: &str) -> IResult<&str, SpringState> {
    let operational = char('.').map(|_| SpringState::Operational);
    let damaged = char('#').map(|_| SpringState::Damaged);
    let unknown = char('?').map(|_| SpringState::Unknown);
    alt((operational, damaged, unknown))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn test1() -> anyhow::Result<()> {
        let input = "???.### 1,1,3";
        let (_, record) = parse_condition_record(input).map_err(|e| anyhow!(e.to_string()))?;
        let result = count_arrangements(&record.spring_states, &record.group_counts);
        let expected = 1;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test2() -> anyhow::Result<()> {
        let input = ".??..??...?##. 1,1,3";
        let (_, record) = parse_condition_record(input).map_err(|e| anyhow!(e.to_string()))?;
        let result = count_arrangements(&record.spring_states, &record.group_counts);
        let expected = 4;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 21;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 525152;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test4() -> anyhow::Result<()> {
        let input = "???.### 1,1,3";
        let (_, record) = parse_condition_record(input).map_err(|e| anyhow!(e.to_string()))?;
        let result = count_arrangements2(&record.spring_states, &record.group_counts);
        let expected = 1;
        assert_eq!(result, expected);
        Ok(())
    }
}
