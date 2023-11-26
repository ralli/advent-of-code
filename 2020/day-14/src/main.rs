use std::collections::BTreeMap;
use std::fmt::Formatter;
use std::{fmt, fs};

use anyhow::{anyhow, Context};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, multispace0, one_of, space0};
use nom::combinator::{all_consuming, map};
use nom::multi::{many1, separated_list0};
use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let filename = "day-14.txt";
    let input =
        fs::read_to_string(filename).with_context(|| format!("cannot read file {filename}"))?;
    let result = part1(&input)?;
    println!("{result}");
    let result = part2(&input)?;
    println!("{result}");
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<u64> {
    let instructions = parse_input(input)?;
    let mut state = State::default();

    for instruction in instructions.into_iter() {
        state.apply(instruction);
    }

    Ok(state.mem.values().sum())
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let instructions = parse_input(input)?;
    let mut state = State::default();

    for instruction in instructions.into_iter() {
        state.apply2(instruction);
    }

    Ok(state.mem.values().sum())
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Instruction>> {
    let (_, result) = all_consuming(terminated(
        separated_list0(line_ending, instruction),
        multispace0,
    ))(input)
    .map_err(|e| anyhow!(e.to_string()))?;
    Ok(result)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MaskValue {
    Ignore,
    Bit(bool),
}

impl fmt::Display for MaskValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            MaskValue::Ignore => 'X',
            MaskValue::Bit(false) => '0',
            MaskValue::Bit(true) => '1',
        };
        write!(f, "{c}")
    }
}

struct State {
    mask: Vec<MaskValue>,
    and_mask: u64,
    or_mask: u64,
    mem: BTreeMap<u64, u64>,
}

impl State {
    fn apply(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Mask { mask } => {
                self.mask = mask;
                let mut and_mask: u64 = !0;
                let mut or_mask = 0;

                for (i, v) in self.mask.iter().rev().enumerate() {
                    match v {
                        MaskValue::Bit(false) => and_mask &= !(1 << i),
                        MaskValue::Bit(true) => or_mask |= 1 << i,
                        _ => {}
                    }
                }

                self.and_mask = and_mask;
                self.or_mask = or_mask;
            }
            Instruction::Mem { addr, value } => {
                self.mem.insert(addr, value & self.and_mask | self.or_mask);
            }
        }
    }

    fn apply2(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Mask { mask } => {
                self.mask = mask;
            }
            Instruction::Mem { addr, value } => {
                let num_x_bits = self
                    .mask
                    .iter()
                    .filter(|&m| *m == MaskValue::Ignore)
                    .count();
                let count = 1 << num_x_bits;
                for m in 0..count {
                    let mut or_mask: u64 = 0;
                    let mut and_mask: u64 = !0;
                    let mut j = 0;
                    for (i, v) in self.mask.iter().rev().enumerate() {
                        match v {
                            MaskValue::Bit(false) => {}
                            MaskValue::Bit(true) => {
                                or_mask |= 1 << i;
                            }
                            MaskValue::Ignore => {
                                let bit = (m >> j) & 1;
                                if bit == 0 {
                                    and_mask &= !(1 << i);
                                } else {
                                    or_mask |= 1 << i;
                                }
                                j += 1;
                            }
                        }
                    }
                    let mem_addr = addr & and_mask | or_mask;
                    self.mem.insert(mem_addr, value);
                }
            }
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            mask: [MaskValue::Ignore; 64].to_vec(),
            and_mask: !0,
            or_mask: 0,
            mem: BTreeMap::new(),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Mask { mask: Vec<MaskValue> },
    Mem { addr: u64, value: u64 },
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((write_mem, write_mask))(input)
}

fn write_mem(input: &str) -> IResult<&str, Instruction> {
    let (input, addr) = delimited(tag("mem["), complete::u64, tag("]"))(input)?;
    let (input, _) = tuple((space0, tag("="), space0))(input)?;
    let (input, value) = complete::u64(input)?;
    Ok((input, Instruction::Mem { addr, value }))
}

fn write_mask(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tuple((tag("mask"), space0, tag("="), space0))(input)?;
    let (input, values) = many1(mask_value)(input)?;
    Ok((input, Instruction::Mask { mask: values }))
}

fn mask_value(input: &str) -> IResult<&str, MaskValue> {
    map(one_of("X01"), |c| match c {
        'X' => MaskValue::Ignore,
        '0' => MaskValue::Bit(false),
        '1' => MaskValue::Bit(true),
        _ => unreachable!(),
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 165;
        assert_eq!(result, expected);
        Ok(())
    }

    static INPUT2: &str = r#"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"#;

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT2)?;
        let expected = 208;
        assert_eq!(result, expected);
        Ok(())
    }
}
