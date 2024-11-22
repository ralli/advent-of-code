use anyhow::{anyhow, Context};
use core::fmt;
use nom::character::complete::multispace0;
use nom::sequence::terminated;
use nom::{bytes::complete::tag, character::complete, multi::separated_list0, IResult};
use std::collections::BTreeMap;
use std::{fs, path::Path};

fn main() -> anyhow::Result<()> {
    let content = read_file("day-9/day-9.txt")?;

    let result = part1(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i64> {
    let (_, v) = parse_input(input).map_err(|e| anyhow!(e.to_string()))?;
    let mut program = Program::new(&v);
    run(&mut program, 1)
}

#[derive(Debug, Clone)]
struct Program {
    values: BTreeMap<usize, i64>,
}

impl Program {
    fn new(values: &[i64]) -> Self {
        Self {
            values: values.iter().copied().enumerate().collect(),
        }
    }

    fn get(&self, index: isize) -> i64 {
        assert!(index >= 0);
        self.values
            .get(&(index as usize))
            .copied()
            .unwrap_or_default()
    }

    fn mode_get(&self, index: isize, mode: i64, base: isize) -> i64 {
        match mode {
            0 => {
                let pos = self.get(index) as isize;
                self.get(pos)
            }
            1 => self.get(index),
            2 => self.get(base + index),
            _ => panic!("invalid mode {}", mode),
        }
    }

    fn set(&mut self, index: isize, value: i64) {
        assert!(index >= 0);
        self.values.insert(index as usize, value);
    }
}

fn run(program: &mut Program, input: i64) -> anyhow::Result<i64> {
    let mut ip: isize = 0;
    let mut output = 0;
    let mut base: isize = 0;

    loop {
        let mut x = program.get(ip);

        let opcode = x % 100;
        x /= 100;

        let a_mode = x % 10;
        assert!(a_mode == 0 || a_mode == 1 || a_mode == 2);
        x /= 10;

        let b_mode = x % 10;
        assert!(b_mode == 0 || b_mode == 1 || b_mode == 2);
        x /= 10;

        let c_mode = x % 10;
        assert!(c_mode == 0 || c_mode == 1 || c_mode == 2);

        match opcode % 100 {
            1 => {
                let a = program.mode_get(ip + 1, a_mode, base);
                let b = program.mode_get(ip + 2, b_mode, base);
                let c = program.get(ip + 3);
                println!("add {a_mode} {a} {b_mode} {b} {c_mode} {c} {}", a * b);
                program.set(c as isize, a + b);
                ip += 4;
            }
            2 => {
                let a = program.mode_get(ip + 1, a_mode, base);
                let b = program.mode_get(ip + 2, b_mode, base);
                let c = program.get(ip + 3);
                println!("mul {a_mode} {a} {b_mode} {b} {c_mode} {c} {}", a * b);
                program.set(c as isize, a * b);
                ip += 4;
            }
            3 => {
                let a = program.get(ip + 1);
                program.set(a as isize, input);
                ip += 2;
            }
            4 => {
                let a = program.get(ip + 1);
                output = program.mode_get(a as isize, a_mode, base);
                println!("out {a_mode} {a} {output}");
                ip += 2;
            }
            5 => {
                let a = program.mode_get(ip + 1, a_mode, base);
                let b = program.mode_get(ip + 2, b_mode, base);
                ip = if a != 0 { b as isize } else { ip + 3 };
            }
            6 => {
                let a = program.mode_get(ip + 1, a_mode, base);
                let b = program.mode_get(ip + 2, b_mode, base);
                ip = if a == 0 { b as isize } else { ip + 3 };
            }
            7 => {
                let a = program.mode_get(ip + 1, a_mode, base);
                let b = program.mode_get(ip + 2, b_mode, base);
                let c = program.get(ip + 3);
                program.set(c as isize, if a < b { 1 } else { 0 });
                ip += 4;
            }
            8 => {
                let a = program.mode_get(ip + 1, a_mode, base);
                let b = program.mode_get(ip + 2, b_mode, base);
                let c = program.get(ip + 3);
                program.set(c as isize, if a == b { 1 } else { 0 });
                ip += 4;
            }
            9 => {
                let a = program.mode_get(ip + 1, a_mode, base);
                base = a as isize;
                ip += 2;
            }
            99 => return Ok(output),
            _ => return Err(anyhow!("invalid opcode {opcode}")),
        };
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<i64>> {
    terminated(separated_list0(tag(","), complete::i64), multispace0)(input)
}

fn read_file(filename: impl AsRef<Path> + fmt::Display) -> anyhow::Result<String> {
    fs::read_to_string(filename.as_ref()).with_context(|| format!("cannot read file {filename}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let input = r#"1102,34915192,34915192,7,4,7,99,0"#;
        let result = part1(input)?;
        assert_eq!(result, 0);
        Ok(())
    }
}
