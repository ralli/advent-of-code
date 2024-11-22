use anyhow::{anyhow, Context};
use core::fmt;
use itertools::Itertools;
use nom::character::complete::multispace0;
use nom::sequence::terminated;
use nom::{bytes::complete::tag, character::complete, multi::separated_list0, IResult};
use std::{fs, path::Path};

fn main() -> anyhow::Result<()> {
    let content = read_file("day-7/day-7.txt")?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<i64> {
    let (_, v) = parse_input(input).map_err(|e| anyhow!(e.to_string()))?;
    let mut max_result = 0;

    for p in (0..=4).permutations(5) {
        let mut program = v.clone();
        let result = run(program.as_mut(), p[0], 0)?;
        let mut program = v.clone();
        let result = run(program.as_mut(), p[1], result)?;
        let mut program = v.clone();
        let result = run(program.as_mut(), p[2], result)?;
        let mut program = v.clone();
        let result = run(program.as_mut(), p[3], result)?;
        let mut program = v.clone();
        let result = run(program.as_mut(), p[4], result)?;
        if result > max_result {
            max_result = result;
        }
    }
    Ok(max_result)
}

fn part2(input: &str) -> anyhow::Result<i64> {
    let (_, v) = parse_input(input).map_err(|e| anyhow!(e.to_string()))?;
    let mut max_result = 0;

    for p in (5..=9).permutations(5) {
        let mut amps = p.iter().map(|&phase| Amp::new(&v, phase)).collect_vec();
        let mut idx = 0;
        let mut output = 0;
        while !amps.last().unwrap().halted {
            amps[idx].run(output)?;
            output = amps[idx].output;
            idx += 1;
            idx %= 5;
        }
        max_result = max_result.max(amps.last().unwrap().output);
    }
    Ok(max_result)
}

fn run(program: &mut [i64], input1: i64, input2: i64) -> anyhow::Result<i64> {
    let mut ip = 0;
    let mut output = 0;
    let mut input = input1;
    assert!(program.len() != 0);
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
                input = input2;
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

struct Amp {
    program: Vec<i64>,
    phase: i64,
    phase_first: bool,
    ip: usize,
    output: i64,
    halted: bool,
}

impl Amp {
    fn new(program: &[i64], phase: i64) -> Self {
        Self {
            program: program.to_vec(),
            phase,
            phase_first: true,
            ip: 0,
            output: 0,
            halted: false,
        }
    }

    fn run(&mut self, input: i64) -> anyhow::Result<()> {
        loop {
            let mut x = self.program[self.ip];

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
                        let a_pos = self.program[self.ip + 1];
                        self.program[a_pos as usize]
                    } else {
                        self.program[self.ip + 1]
                    };

                    let b = if b_mode == 0 {
                        let b_pos = self.program[self.ip + 2];
                        self.program[b_pos as usize]
                    } else {
                        self.program[self.ip + 2]
                    };

                    let c = self.program[self.ip + 3];

                    self.program[c as usize] = a + b;

                    self.ip += 4;
                }
                2 => {
                    let a = if a_mode == 0 {
                        let a_pos = self.program[self.ip + 1];
                        self.program[a_pos as usize]
                    } else {
                        self.program[self.ip + 1]
                    };

                    let b = if b_mode == 0 {
                        let b_pos = self.program[self.ip + 2];
                        self.program[b_pos as usize]
                    } else {
                        self.program[self.ip + 2]
                    };

                    let c = self.program[self.ip + 3];

                    self.program[c as usize] = a * b;

                    self.ip += 4;
                }
                3 => {
                    let a = self.program[self.ip + 1];
                    let input_value = if self.phase_first {
                        self.phase_first = false;
                        self.phase
                    } else {
                        input
                    };
                    self.program[a as usize] = input_value;
                    self.ip += 2;
                }
                4 => {
                    let a = self.program[self.ip + 1];
                    self.output = if c_mode == 0 {
                        self.program[a as usize]
                    } else {
                        a
                    };
                    self.ip += 2;
                    return Ok(());
                }
                5 => {
                    let a = if a_mode == 0 {
                        let a_pos = self.program[self.ip + 1];
                        self.program[a_pos as usize]
                    } else {
                        self.program[self.ip + 1]
                    };

                    let b = if b_mode == 0 {
                        let b_pos = self.program[self.ip + 2];
                        self.program[b_pos as usize]
                    } else {
                        self.program[self.ip + 2]
                    };

                    self.ip = if a != 0 { b as usize } else { self.ip + 3 };
                }
                6 => {
                    let a = if a_mode == 0 {
                        let a_pos = self.program[self.ip + 1];
                        self.program[a_pos as usize]
                    } else {
                        self.program[self.ip + 1]
                    };

                    let b = if b_mode == 0 {
                        let b_pos = self.program[self.ip + 2];
                        self.program[b_pos as usize]
                    } else {
                        self.program[self.ip + 2]
                    };

                    self.ip = if a == 0 { b as usize } else { self.ip + 3 };
                }
                7 => {
                    let a = if a_mode == 0 {
                        let a_pos = self.program[self.ip + 1];
                        self.program[a_pos as usize]
                    } else {
                        self.program[self.ip + 1]
                    };

                    let b = if b_mode == 0 {
                        let b_pos = self.program[self.ip + 2];
                        self.program[b_pos as usize]
                    } else {
                        self.program[self.ip + 2]
                    };

                    let c = self.program[self.ip + 3];

                    self.program[c as usize] = if a < b { 1 } else { 0 };

                    self.ip += 4;
                }
                8 => {
                    let a = if a_mode == 0 {
                        let a_pos = self.program[self.ip + 1];
                        self.program[a_pos as usize]
                    } else {
                        self.program[self.ip + 1]
                    };

                    let b = if b_mode == 0 {
                        let b_pos = self.program[self.ip + 2];
                        self.program[b_pos as usize]
                    } else {
                        self.program[self.ip + 2]
                    };

                    let c = self.program[self.ip + 3];

                    self.program[c as usize] = if a == b { 1 } else { 0 };

                    self.ip += 4;
                }
                99 => {
                    self.halted = true;
                    return Ok(());
                }
                _ => return Err(anyhow!("invalid opcode {opcode}")),
            };
        }
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
    fn part1_works() -> anyhow::Result<()> {
        let input = r#"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"#;
        let result = part1(input)?;
        assert_eq!(result, 43210);
        Ok(())
    }

    #[test]
    fn part1_works2() -> anyhow::Result<()> {
        let input = r#"3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"#;
        let result = part1(input)?;
        assert_eq!(result, 54321);
        Ok(())
    }

    #[test]
    fn part1_works3() -> anyhow::Result<()> {
        let input = r#"3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"#;
        let result = part1(input)?;
        assert_eq!(result, 65210);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let input = r#"3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"#;
        let result = part2(input)?;
        assert_eq!(result, 139629729);
        Ok(())
    }
}
