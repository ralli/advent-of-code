use core::fmt;
use std::{fs, path::Path};

use anyhow::{anyhow, Context};
use nom::{bytes::complete::tag, character::complete, multi::separated_list0, IResult};

fn main() -> anyhow::Result<()> {
    let content = read_file("day-5/day-5.txt")?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i32> {
    let (_, mut v) = parse_input(input).map_err(|e| anyhow!(e.to_string()))?;
    run1(&mut v, 1)
}

fn part2(input: &str) -> anyhow::Result<i32> {
    let (_, mut v) = parse_input(input).map_err(|e| anyhow!(e.to_string()))?;
    run2(&mut v, 5)
}

fn run1(program: &mut [i32], input: i32) -> anyhow::Result<i32> {
    let mut ip = 0;
    let mut output = 0;

    loop {
        let mut x = program[ip];

        let opcode = x % 100;
        x /= 100;

        let a_mode = x % 10;
        assert!(a_mode == 0 || a_mode == 1);
        x /= 10;

        let b_mode = x % 10;
        assert!(b_mode == 0 || b_mode == 1);
        x /= 10;

        let c_mode = x % 10;
        assert!(c_mode == 0 || c_mode == 1);

        match opcode % 100 {
            1 => {
                let a = if a_mode == 0 {
                    let a_pos = program[ip + 1];
                    program[a_pos as usize]
                } else {
                    program[ip + 1]
                };

                let b = if b_mode == 0 {
                    let b_pos = program[ip + 2];
                    program[b_pos as usize]
                } else {
                    program[ip + 2]
                };

                let c = program[ip + 3];

                program[c as usize] = a + b;

                ip += 4;
            }
            2 => {
                let a = if a_mode == 0 {
                    let a_pos = program[ip + 1];
                    program[a_pos as usize]
                } else {
                    program[ip + 1]
                };

                let b = if b_mode == 0 {
                    let b_pos = program[ip + 2];
                    program[b_pos as usize]
                } else {
                    program[ip + 2]
                };

                let c = program[ip + 3];

                program[c as usize] = a * b;

                ip += 4;
            }
            3 => {
                let a = program[ip + 1];
                program[a as usize] = input;
                ip += 2;
            }
            4 => {
                let a = program[ip + 1];
                output = if c_mode == 0 { program[a as usize] } else { a };
                ip += 2;
            }
            99 => return Ok(output),
            _ => return Err(anyhow!("invalid opcode {opcode}")),
        };
    }
}

fn run2(program: &mut [i32], input: i32) -> anyhow::Result<i32> {
    let mut ip = 0;
    let mut output = 0;

    loop {
        let mut x = program[ip];

        let opcode = x % 100;
        x /= 100;

        let a_mode = x % 10;
        assert!(a_mode == 0 || a_mode == 1);
        x /= 10;

        let b_mode = x % 10;
        assert!(b_mode == 0 || b_mode == 1);
        x /= 10;

        let c_mode = x % 10;
        assert!(c_mode == 0 || c_mode == 1);

        match opcode % 100 {
            1 => {
                let a = if a_mode == 0 {
                    let a_pos = program[ip + 1];
                    program[a_pos as usize]
                } else {
                    program[ip + 1]
                };

                let b = if b_mode == 0 {
                    let b_pos = program[ip + 2];
                    program[b_pos as usize]
                } else {
                    program[ip + 2]
                };

                let c = program[ip + 3];

                program[c as usize] = a + b;

                ip += 4;
            }
            2 => {
                let a = if a_mode == 0 {
                    let a_pos = program[ip + 1];
                    program[a_pos as usize]
                } else {
                    program[ip + 1]
                };

                let b = if b_mode == 0 {
                    let b_pos = program[ip + 2];
                    program[b_pos as usize]
                } else {
                    program[ip + 2]
                };

                let c = program[ip + 3];

                program[c as usize] = a * b;

                ip += 4;
            }
            3 => {
                let a = program[ip + 1];
                program[a as usize] = input;
                ip += 2;
            }
            4 => {
                let a = program[ip + 1];
                output = if c_mode == 0 { program[a as usize] } else { a };
                ip += 2;
            }
            5 => {
                let a = if a_mode == 0 {
                    let a_pos = program[ip + 1];
                    program[a_pos as usize]
                } else {
                    program[ip + 1]
                };

                let b = if b_mode == 0 {
                    let b_pos = program[ip + 2];
                    program[b_pos as usize]
                } else {
                    program[ip + 2]
                };

                ip = if a != 0 { b as usize } else { ip + 3 };
            }
            6 => {
                let a = if a_mode == 0 {
                    let a_pos = program[ip + 1];
                    program[a_pos as usize]
                } else {
                    program[ip + 1]
                };

                let b = if b_mode == 0 {
                    let b_pos = program[ip + 2];
                    program[b_pos as usize]
                } else {
                    program[ip + 2]
                };

                ip = if a == 0 { b as usize } else { ip + 3 };
            }
            7 => {
                let a = if a_mode == 0 {
                    let a_pos = program[ip + 1];
                    program[a_pos as usize]
                } else {
                    program[ip + 1]
                };

                let b = if b_mode == 0 {
                    let b_pos = program[ip + 2];
                    program[b_pos as usize]
                } else {
                    program[ip + 2]
                };

                let c = program[ip + 3];

                program[c as usize] = if a < b { 1 } else { 0 };

                ip += 4;
            }
            8 => {
                let a = if a_mode == 0 {
                    let a_pos = program[ip + 1];
                    program[a_pos as usize]
                } else {
                    program[ip + 1]
                };

                let b = if b_mode == 0 {
                    let b_pos = program[ip + 2];
                    program[b_pos as usize]
                } else {
                    program[ip + 2]
                };

                let c = program[ip + 3];

                program[c as usize] = if a == b { 1 } else { 0 };

                ip += 4;
            }
            99 => return Ok(output),
            _ => return Err(anyhow!("invalid opcode {opcode}")),
        };
    }
}
fn parse_input(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list0(tag(","), complete::i32)(input)
}

fn read_file(filename: impl AsRef<Path> + fmt::Display) -> anyhow::Result<String> {
    fs::read_to_string(filename.as_ref()).with_context(|| format!("cannot read file {filename}"))
}
