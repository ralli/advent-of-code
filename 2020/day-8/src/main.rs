use std::collections::BTreeSet;
use std::fmt::Formatter;
use std::{fmt, fs};

use anyhow::{anyhow, Context};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, multispace0, space1};
use nom::combinator::{eof, map};
use nom::multi::separated_list0;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let filename = "day-8.txt";
    let input =
        fs::read_to_string(filename).with_context(|| format!("cannot load file {filename}"))?;
    let result = part1(&input)?;
    println!("{result}");
    let result = part2(&input)?;
    println!("{result}");
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i32> {
    let instructions = parse_input(input)?;
    let mut visited: BTreeSet<i32> = BTreeSet::new();
    let mut acc = 0;
    let mut pc = 0;
    loop {
        if pc < 0 || (pc as usize) > instructions.len() {
            return Err(anyhow!("instrcution out of range {pc}"));
        }
        if !visited.insert(pc) {
            return Ok(acc);
        }

        let instr = instructions[pc as usize];
        match instr {
            Instruction::Nop(_) => pc += 1,
            Instruction::Acc(n) => {
                acc += n;
                pc += 1;
            }
            Instruction::Jmp(n) => {
                pc += n;
            }
        }
    }
}

fn part2(input: &str) -> anyhow::Result<i32> {
    fn try_alternative(
        visited: &BTreeSet<i32>,
        instructions: &[Instruction],
        acc: i32,
        pc: i32,
        n: i32,
    ) -> Option<i32> {
        if n > 1 {
            return None;
        }
        let mut visited = visited.clone();
        let mut acc = acc;
        let mut pc = pc;
        loop {
            if pc as usize == instructions.len() {
                return Some(acc);
            }
            if pc < 0 || (pc as usize) > instructions.len() {
                return None;
            }
            if !visited.insert(pc) {
                return None;
            }
            let instruction = instructions[pc as usize];
            match instruction {
                Instruction::Nop(delta) => {
                    // try JMP
                    if let Some(result) =
                        try_alternative(&visited, instructions, acc, pc + delta, n + 1)
                    {
                        return Some(result);
                    }
                    pc += 1;
                }
                Instruction::Acc(delta) => {
                    pc += 1;
                    acc += delta;
                }
                Instruction::Jmp(delta) => {
                    // try NOP
                    if let Some(result) =
                        try_alternative(&visited, instructions, acc, pc + 1, n + 1)
                    {
                        return Some(result);
                    }
                    pc += delta;
                }
            }
        }
    }
    let instructions = parse_input(input)?;
    let visited: BTreeSet<i32> = BTreeSet::new();
    let acc = 0;
    let pc = 0;
    let result = try_alternative(&visited, &instructions, acc, pc, 0);
    result.ok_or_else(|| anyhow!("No solution found"))
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Nop(n) => write!(f, "nop {n}"),
            Instruction::Acc(n) => write!(f, "acc {n}"),
            Instruction::Jmp(n) => write!(f, "jmp {n}"),
        }
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Instruction>> {
    let (_, instructions) = terminated(instructions, tuple((multispace0, eof)))(input)
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
    Ok(instructions)
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list0(line_ending, instruction)(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let nop = map(preceded(tuple((tag("nop"), space1)), complete::i32), |n| {
        Instruction::Nop(n)
    });
    let acc = map(preceded(tuple((tag("acc"), space1)), complete::i32), |n| {
        Instruction::Acc(n)
    });
    let jmp = map(preceded(tuple((tag("jmp"), space1)), complete::i32), |n| {
        Instruction::Jmp(n)
    });
    alt((nop, acc, jmp))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 5;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        let expected = 8;
        assert_eq!(result, expected);
        Ok(())
    }
}
