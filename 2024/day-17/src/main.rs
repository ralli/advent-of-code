use anyhow::{anyhow, Context};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, multispace0, one_of, space1};
use nom::combinator::eof;
use nom::multi::{many1, separated_list0};
use nom::IResult;
use std::collections::VecDeque;
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

fn run(computer: &Computer, a: i64) -> Vec<i64> {
    let mut c = computer.clone();
    c.a = a;
    c.run().unwrap()
}

/// A bit ugly, but it works
///
/// The output of the program depends on the lowest 3 bits of the register "A".
///
/// The idea:
///
/// * Find all values of 'A' for the last output value of the program (0 <= A <= 7).
/// * Multiply the result by 8 (a' = a <<= 3) and repeat find a new value for a' + a that outputs the last two digits.
/// * ...
///
/// Using a BFS because there may be multiple values of A that generate the intended output.
///
fn part2(_input: &str) -> anyhow::Result<usize> {
    let (_, computer) = parse_computer(_input).map_err(|e| anyhow!("{e}"))?;
    let program: Vec<i64> = computer.program.iter().map(|i| *i as i64).collect();
    let mut q = VecDeque::from([(0i64, program)]);
    while let Some((a, program)) = q.pop_front() {
        if program.is_empty() {
            return Ok(a as usize);
        }
        let goal = program.last().copied().unwrap();
        for i in 0..8 {
            let output = run(&computer, 8 * a + i as i64);
            if output.first() == Some(&goal) {
                // println!("{} {:?}", a + i as i64, output);
                let mut next_program = program.clone();
                next_program.pop();
                q.push_back(((8 * a + i as i64), next_program));
            }
        }
    }
    Err(anyhow!("no solution found"))
}

#[derive(Debug, Clone)]
struct Computer {
    a: i64,
    b: i64,
    c: i64,
    program: Vec<u8>,
    ip: usize,
}


/// looking at the instruction dump of part1 you see some reasons, why part2 works:
///
/// 1. the a register value gets 3 bits shorter each round
/// 2. the output of the program for a given state (register values a, b, c) is only dependent on
///    the 2 lower bits of the a register.
///
/// a=   1100000101110111010111111 b=                           0 c=                           0 ip= 0 inst=2
/// a=   1100000101110111010111111 b=                         111 c=                           0 ip= 2 inst=1
/// a=   1100000101110111010111111 b=                         110 c=                           0 ip= 4 inst=7
/// a=   1100000101110111010111111 b=                         110 c=         1100000101110111010 ip= 6 inst=0
/// a=      1100000101110111010111 b=                         110 c=         1100000101110111010 ip= 8 inst=4
/// a=      1100000101110111010111 b=         1100000101110111100 c=         1100000101110111010 ip=10 inst=1
/// a=      1100000101110111010111 b=         1100000101110111010 c=         1100000101110111010 ip=12 inst=5
/// a=      1100000101110111010111 b=         1100000101110111010 c=         1100000101110111010 ip=14 inst=3
///
/// a=      1100000101110111010111 b=         1100000101110111010 c=         1100000101110111010 ip= 0 inst=2
/// a=      1100000101110111010111 b=                         111 c=         1100000101110111010 ip= 2 inst=1
/// a=      1100000101110111010111 b=                         110 c=         1100000101110111010 ip= 4 inst=7
/// a=      1100000101110111010111 b=                         110 c=            1100000101110111 ip= 6 inst=0
/// a=         1100000101110111010 b=                         110 c=            1100000101110111 ip= 8 inst=4
/// a=         1100000101110111010 b=            1100000101110001 c=            1100000101110111 ip=10 inst=1
/// a=         1100000101110111010 b=            1100000101110111 c=            1100000101110111 ip=12 inst=5
/// a=         1100000101110111010 b=            1100000101110111 c=            1100000101110111 ip=14 inst=3
///
/// ...many more rounds...
///
/// a=                        1100 b=                      110111 c=                      110000 ip= 0 inst=2
/// a=                        1100 b=                         100 c=                      110000 ip= 2 inst=1
/// a=                        1100 b=                         101 c=                      110000 ip= 4 inst=7
/// a=                        1100 b=                         101 c=                           0 ip= 6 inst=0
/// a=                           1 b=                         101 c=                           0 ip= 8 inst=4
/// a=                           1 b=                         101 c=                           0 ip=10 inst=1
/// a=                           1 b=                          11 c=                           0 ip=12 inst=5
/// a=                           1 b=                          11 c=                           0 ip=14 inst=3
/// a=                           1 b=                          11 c=                           0 ip= 0 inst=2
/// a=                           1 b=                           1 c=                           0 ip= 2 inst=1
/// a=                           1 b=                           0 c=                           0 ip= 4 inst=7
/// a=                           1 b=                           0 c=                           1 ip= 6 inst=0
/// a=                           0 b=                           0 c=                           1 ip= 8 inst=4
/// a=                           0 b=                           1 c=                           1 ip=10 inst=1
/// a=                           0 b=                         111 c=                           1 ip=12 inst=5
/// a=                           0 b=                         111 c=                           1 ip=14 inst=3

#[allow(dead_code)]
fn dump_state(computer: &Computer) {
    println!(
        "a={:28b} b={:28b} c={:28b} ip={:2} inst={}",
        computer.a, computer.b, computer.c, computer.ip, computer.program[computer.ip]
    );
}

impl Computer {
    fn run(&mut self) -> anyhow::Result<Vec<i64>> {
        let mut output: Vec<i64> = Vec::new();
        let size = self.program.len();
        while self.ip < size {
            // dump_state(self);
            let instruction = self.program[self.ip];
            match instruction {
                0 => {
                    let combo_value = self.combo_op_value(self.program[self.ip + 1])?;
                    self.a >>= combo_value;
                    self.ip += 2;
                }
                1 => {
                    let operand = self.program[self.ip + 1] as i64;
                    self.b ^= operand;
                    self.ip += 2;
                }
                2 => {
                    let combo = self.combo_op_value(self.program[self.ip + 1])?;
                    self.b = combo % 8;
                    self.ip += 2;
                }
                3 => {
                    let operand = self.program[self.ip + 1] as i64;
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
                    let combo_value = self.combo_op_value(self.program[self.ip + 1])?;
                    self.b = self.a >> combo_value;
                    self.ip += 2;
                }
                7 => {
                    let combo_value = self.combo_op_value(self.program[self.ip + 1])?;
                    self.c = self.a >> combo_value;
                    self.ip += 2;
                }
                _ => return Err(anyhow!("invalid instruction {}", instruction)),
            }
        }

        Ok(output)
    }

    fn combo_op_value(&self, operand: u8) -> anyhow::Result<i64> {
        match operand {
            4 => Ok(self.a),
            5 => Ok(self.b),
            6 => Ok(self.c),
            o @ 0..=3 => Ok(o as i64),
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
fn parse_register(input: &str) -> IResult<&str, i64> {
    let (input, _) = tag("Register")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = one_of("ABC")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space1(input)?;
    let (input, value) = complete::i64(input)?;

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
