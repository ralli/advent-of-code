use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space1};
use nom::multi::separated_list1;
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let input = read_file("./day-10/input.txt")?;
    let result = part1(&input);

    println!("{}", result);

    let result = part2(&input);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> i32 {
    let (_, instructions) = instructions(input).unwrap();
    let hist = execution_history(&instructions);
    let cycles: [i32; 6] = [20, 60, 100, 140, 180, 220];

    cycles.into_iter().map(|c| c * hist[(c - 1) as usize]).sum()
}

fn part2(input: &str) -> String {
    let (_, instructions) = instructions(input).unwrap();
    let hist = execution_history(&instructions);
    let mut display = Vec::new();

    let mut cycle = 0;
    for _ in 0..6 {
        let mut row = Vec::new();
        for c in 0..40 {
            let x = hist[cycle];
            cycle += 1;
            row.push(if (x - 1) <= c && (x + 1) >= c {
                '#'
            } else {
                '.'
            })
        }
        display.push(row);
    }

    let lines: Vec<String> = display
        .into_iter()
        .map(|v| v.into_iter().collect::<String>())
        .collect();
    lines.join("\n")
}

fn execution_history(instructions: &[Instruction]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut x = 1;

    for instruction in instructions.iter() {
        match instruction {
            Instruction::AddX(n) => {
                result.push(x);
                result.push(x);
                x += n;
            }
            Instruction::NoOp => {
                result.push(x);
            }
        }
    }
    result
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    AddX(i32),
    NoOp,
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, instruction)(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((noop, addx))(input)
}

fn noop(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Instruction::NoOp))
}

fn addx(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("addx")(input)?;
    let (input, _) = space1(input)?;
    let (input, n) = nom::character::complete::i32(input)?;

    Ok((input, Instruction::AddX(n)))
}

fn read_file(filename: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 13140;
        assert_eq!(result, expected);
    }
}
