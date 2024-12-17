use anyhow::{anyhow, Context};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, multispace0, one_of, space1};
use nom::combinator::eof;
use nom::multi::{many1, separated_list0};
use nom::IResult;
use rayon::prelude::*;
use std::fs;

fn main() -> anyhow::Result<()> {
    let filename = "day-17/input.txt";
    let content = fs::read_to_string(filename).context(format!("cannot load {filename}"))?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<String> {
    let (_, mut computer) = parse_computer(input).map_err(|e| anyhow!("{e}"))?;
    let output = computer.run()?;
    let result = output.iter().join(",");
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let (_, mut computer) = parse_computer(input).map_err(|e| anyhow!("{e}"))?;
    let result_attempt = (0..i32::MAX).find(|a| {
        let mut c = computer.clone();
        c.a = *a;
        if c.run2().unwrap() {
            return true;
        }
        if a % 100_000_000 == 0 {
            println!("a: {a}");
        }
        false
    });
    result_attempt
        .map(|a| a as usize)
        .ok_or_else(|| anyhow!("no result found"))
}

#[derive(Debug, Clone)]
struct Computer {
    a: i32,
    b: i32,
    c: i32,
    program: Vec<u8>,
    ip: usize,
}

impl Computer {
    fn run(&mut self) -> anyhow::Result<Vec<i32>> {
        let mut output: Vec<i32> = Vec::new();
        let size = self.program.len();
        while self.ip < size {
            let instruction = self.program[self.ip];
            match instruction {
                0 => {
                    let numerator = self.a;
                    let combo_value = self.combo_op_value(self.program[self.ip + 1])?;
                    let denominator: i32 = 2i32.pow(combo_value as u32);
                    self.a = numerator / denominator;
                    self.ip += 2;
                }
                1 => {
                    let operand = self.program[self.ip + 1] as i32;
                    self.b ^= operand;
                    self.ip += 2;
                }
                2 => {
                    self.b = self.combo_op_value(self.program[self.ip + 1])? % 8;
                    self.ip += 2;
                }
                3 => {
                    let operand = self.program[self.ip + 1] as i32;
                    if self.a == 0 {
                        self.ip += 2;
                    } else {
                        self.ip = operand as usize;
                    }
                }
                4 => {
                    self.b = self.b ^ self.c;
                    self.ip += 2;
                }
                5 => {
                    let value = self.combo_op_value(self.program[self.ip + 1])? % 8;
                    output.push(value);
                    self.ip += 2;
                }
                6 => {
                    let numerator = self.a;
                    let combo_value = self.combo_op_value(self.program[self.ip + 1])?;
                    let denominator: i32 = 2i32.pow(combo_value as u32);
                    self.b = numerator / denominator;
                    self.ip += 2;
                }
                7 => {
                    let numerator = self.a;
                    let combo_value = self.combo_op_value(self.program[self.ip + 1])?;
                    let denominator: i32 = 2i32.pow(combo_value as u32);
                    self.c = numerator / denominator;
                    self.ip += 2;
                }
                _ => return Err(anyhow!("invalid instruction {}", instruction)),
            }
        }

        Ok(output)
    }

    fn run2(&mut self) -> anyhow::Result<bool> {
        let size = self.program.len();
        let mut out_idx = 0;
        while self.ip < size {
            let instruction = self.program[self.ip];
            match instruction {
                0 => {
                    let numerator = self.a;
                    let combo_value = self.combo_op_value(self.program[self.ip + 1])?;
                    let denominator: i32 = 2i32.pow(combo_value as u32);
                    self.a = numerator / denominator;
                    self.ip += 2;
                }
                1 => {
                    let operand = self.program[self.ip + 1] as i32;
                    self.b ^= operand;
                    self.ip += 2;
                }
                2 => {
                    self.b = self.combo_op_value(self.program[self.ip + 1])? % 8;
                    self.ip += 2;
                }
                3 => {
                    let operand = self.program[self.ip + 1] as i32;
                    if self.a == 0 {
                        self.ip += 2;
                    } else {
                        self.ip = operand as usize;
                    }
                }
                4 => {
                    self.b = self.b ^ self.c;
                    self.ip += 2;
                }
                5 => {
                    let value = self.combo_op_value(self.program[self.ip + 1])? % 8;
                    if self.program[out_idx] != value as u8 {
                        return Ok(false);
                    }
                    out_idx += 1;
                    if out_idx == size {
                        return Ok(true);
                    }
                    self.ip += 2;
                }
                6 => {
                    let numerator = self.a;
                    let combo_value = self.combo_op_value(self.program[self.ip + 1])?;
                    let denominator: i32 = 2i32.pow(combo_value as u32);
                    self.b = numerator / denominator;
                    self.ip += 2;
                }
                7 => {
                    let numerator = self.a;
                    let combo_value = self.combo_op_value(self.program[self.ip + 1])?;
                    let denominator: i32 = 2i32.pow(combo_value as u32);
                    self.c = numerator / denominator;
                    self.ip += 2;
                }
                _ => return Err(anyhow!("invalid instruction {}", instruction)),
            }
        }

        Ok(false)
    }

    fn combo_op_value(&self, operand: u8) -> anyhow::Result<i32> {
        match operand {
            4 => Ok(self.a),
            5 => Ok(self.b),
            6 => Ok(self.c),
            o @ 0..=3 => Ok(o as i32),
            _ => Err(anyhow!("invalid operand {}", operand)),
        }
    }
}

fn parse_computer(input: &str) -> IResult<&str, Computer> {
    let (input, a) = parse_register(input)?;
    let (input, _) = line_ending(input)?;
    let (input, b) = parse_register(input)?;
    let (input, _) = line_ending(input)?;
    let (input, c) = parse_register(input)?;
    let (input, _) = many1(line_ending)(input)?;
    let (input, program) = parse_program(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = eof(input)?;

    Ok((
        input,
        Computer {
            a,
            b,
            c,
            program,
            ip: 0,
        },
    ))
}
fn parse_register(input: &str) -> IResult<&str, i32> {
    let (input, _) = tag("Register")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = one_of("ABC")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space1(input)?;
    let (input, value) = complete::i32(input)?;

    Ok((input, value))
}

fn parse_program(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, _) = tag("Program: ")(input)?;
    separated_list0(tag(","), complete::u8)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    #[test]
    fn test1() -> anyhow::Result<()> {
        let mut computer = Computer {
            a: 0,
            b: 0,
            c: 9,
            program: vec![2, 6],
            ip: 0,
        };
        computer.run()?;
        assert_eq!(computer.b, 1);
        Ok(())
    }

    #[test]
    fn test2() -> anyhow::Result<()> {
        let mut computer = Computer {
            a: 10,
            b: 0,
            c: 0,
            program: vec![5, 0, 5, 1, 5, 4],
            ip: 0,
        };
        let output = computer.run()?;
        assert_eq!(output, vec![0, 1, 2]);
        Ok(())
    }

    #[test]
    fn test3() -> anyhow::Result<()> {
        let mut computer = Computer {
            a: 2024,
            b: 0,
            c: 0,
            program: vec![0, 1, 5, 4, 3, 0],
            ip: 0,
        };
        let output = computer.run()?;
        assert_eq!(output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(computer.a, 0);
        Ok(())
    }

    #[test]
    fn test4() -> anyhow::Result<()> {
        let mut computer = Computer {
            a: 0,
            b: 29,
            c: 0,
            program: vec![1, 7],
            ip: 0,
        };
        computer.run()?;
        assert_eq!(computer.b, 26);
        Ok(())
    }

    #[test]
    fn test6() -> anyhow::Result<()> {
        let mut computer = Computer {
            a: 117440,
            b: 0,
            c: 0,
            program: vec![0, 3, 5, 4, 3, 0],
            ip: 0,
        };
        let output = computer.run2()?;
        assert!(output);
        // assert_eq!(output, vec![0, 3, 5, 4, 3, 0]);
        Ok(())
    }

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let input = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;
        let result = part2(input)?;

        assert_eq!(result, 117440);
        Ok(())
    }
}
