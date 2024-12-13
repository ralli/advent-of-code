use anyhow::{anyhow, Context};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, multispace0, one_of, space1};
use nom::multi::{many1, separated_list0};
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;
use std::fs;

fn main() -> anyhow::Result<()> {
    let filename = "day-13/input.txt";
    let content = fs::read_to_string(filename).context(format!("cannot load {filename}"))?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let (_, machines) = parse_input(input).map_err(|e| anyhow!("{e}"))?;
    let mut result = 0;
    for machine in machines {
        let (n1, n2) = solve(&machine)?;
        if n1 > 0.0 && n2 > 0.0 && n1.fract() < 1e-10 && n2.fract() < 1e-10 {
            result += 3 * (n1 as usize) + (n2 as usize);
        }
    }
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let (_, mut machines) = parse_input(input).map_err(|e| anyhow!("{e}"))?;
    for m in machines.iter_mut() {
        m.price_x += 10000000000000.0;
        m.price_y += 10000000000000.0;
    }
    let mut result = 0;
    for machine in machines {
        let (n1, n2) = solve(&machine)?;
        if n1 > 0.0 && n2 > 0.0 && n1.fract() < 1e-10 && n2.fract() < 1e-10 {
            result += 3 * (n1 as usize) + (n2 as usize);
        }
    }
    Ok(result)
}

fn solve(claw_machine: &ClawMachine) -> anyhow::Result<(f64, f64)> {
    let determinant = claw_machine.ax * claw_machine.by - claw_machine.ay * claw_machine.bx;

    if determinant.abs() < 1e-8 {
        return Err(anyhow!("not solvable"));
    }

    let n1 = (claw_machine.price_x * claw_machine.by - claw_machine.price_y * claw_machine.bx)
        / determinant;
    let n2 = (claw_machine.price_y * claw_machine.ax - claw_machine.price_x * claw_machine.ay)
        / determinant;

    Ok((n1, n2))
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
struct ClawMachine {
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    price_x: f64,
    price_y: f64,
}

fn parse_input(input: &str) -> IResult<&str, Vec<ClawMachine>> {
    terminated(
        separated_list0(many1(line_ending), parse_claw_machine),
        multispace0,
    )(input)
}

fn parse_claw_machine(input: &str) -> IResult<&str, ClawMachine> {
    let (rest, (ax, ay)) = parse_button(input)?;
    let (rest, _) = line_ending(rest)?;
    let (rest, (bx, by)) = parse_button(rest)?;
    let (rest, _) = line_ending(rest)?;
    let (rest, (price_x, price_y)) = parse_price(rest)?;

    Ok((
        rest,
        ClawMachine {
            ax,
            ay,
            bx,
            by,
            price_x,
            price_y,
        },
    ))
}

fn parse_button(input: &str) -> IResult<&str, (f64, f64)> {
    let (rest, _) = tuple((tag("Button"), preceded(space1, one_of("AB")), tag(":")))(input)?;
    let (rest, _) = space1(rest)?;
    let (rest, x) = preceded(tag("X+"), complete::i64)(rest)?;
    let (rest, _) = tuple((tag(","), space1))(rest)?;
    let (rest, y) = preceded(tag("Y+"), complete::i64)(rest)?;
    Ok((rest, (x as f64, y as f64)))
}

fn parse_price(input: &str) -> IResult<&str, (f64, f64)> {
    let (rest, _) = tag("Prize:")(input)?;
    let (rest, _) = space1(rest)?;
    let (rest, x) = preceded(tag("X="), complete::i64)(rest)?;
    let (rest, _) = tuple((tag(","), space1))(rest)?;
    let (rest, y) = preceded(tag("Y="), complete::i64)(rest)?;
    Ok((rest, (x as f64, y as f64)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        assert_eq!(result, 480);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        assert_eq!(result, 875318608908);
        Ok(())
    }
}
