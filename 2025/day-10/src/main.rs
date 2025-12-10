use anyhow::anyhow;
use good_lp::*;
use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};
use winnow::ascii::{digit1, line_ending, multispace0, space1};
use winnow::combinator::{alt, delimited, eof, repeat, separated, terminated};
use winnow::{ModalResult, Parser};

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day-10.txt")?;

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut inp = input;
    let machines = terminated(parse_machines, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|e| anyhow!("{e}"))?;
    let result = machines.iter().map(find_min_steps).sum();
    Ok(result)
}

struct State<'a> {
    indicators: Vec<bool>,
    schematic: &'a [usize],
    steps: usize,
}

fn find_min_steps(machine: &Machine) -> usize {
    let initial_state = vec![false; machine.indicators.len()];
    let mut q = VecDeque::from_iter(machine.schematics.iter().map(|s| State {
        indicators: initial_state.clone(),
        schematic: s,
        steps: 0,
    }));
    let mut visited = HashSet::from([initial_state.clone()]);
    while let Some(current) = q.pop_front() {
        if current.indicators == machine.indicators {
            return current.steps;
        }
        let next_state = apply_schematic(current.indicators.as_slice(), current.schematic);
        if !visited.insert(next_state.clone()) {
            continue;
        }
        for schematic in &machine.schematics {
            q.push_back(State {
                indicators: next_state.clone(),
                schematic,
                steps: current.steps + 1,
            });
        }
    }
    0
}

fn apply_schematic(state: &[bool], schematic: &[usize]) -> Vec<bool> {
    let mut result = state.to_vec();
    for &i in schematic {
        result[i] = !result[i];
    }
    result
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut inp = input;
    let machines = terminated(parse_machines, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|e| anyhow!("{e}"))?;
    let result = machines
        .par_iter()
        .map(|m| find_joltage_steps(m).unwrap())
        .sum();
    Ok(result)
}

// a bfs as in part1 was slow. This approach uses linear programming to solve the problem.
//
// (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
//
// x0, ..., x4 are integers and >= 0
// x4 + x5 = 3
// x1 + x5 = 5
// x2 + x3 + x4 = 4
// x0 + x1 + x3 = 7
//

fn find_joltage_steps(machine: &Machine) -> anyhow::Result<usize> {
    // define the variables x0, ..., xn
    let mut vars = variables!();
    let press_vars = machine
        .schematics
        .iter()
        .map(|_| vars.add(variable().min(0).integer()))
        .collect::<Vec<_>>();

    let mut expressions: Vec<Expression> = machine
        .joltages
        .iter()
        .map(|_| 0.into_expression())
        .collect();

    for (i, schematic) in machine.schematics.iter().enumerate() {
        for &j in schematic.iter() {
            // add "+ xi" to the expression for the joltage
            expressions[j] += press_vars[i];
        }
    }

    let mut problem = vars
        .minimise(press_vars.iter().sum::<Expression>())
        .using(default_solver);

    for (expression, &joltage) in expressions.into_iter().zip(machine.joltages.iter()) {
        problem.add_constraint(expression.eq(joltage as f64));
    }

    let solution = problem.solve()?;
    let result = press_vars.iter().map(|&v| solution.value(v)).sum::<f64>();

    Ok(result as usize)
}

#[derive(Debug, Clone)]
struct Machine {
    indicators: Vec<bool>,
    schematics: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

fn parse_machines(input: &mut &str) -> ModalResult<Vec<Machine>> {
    separated(1.., parse_machine, line_ending).parse_next(input)
}

fn parse_machine(input: &mut &str) -> ModalResult<Machine> {
    (
        parse_indicators,
        space1,
        parse_schematics,
        space1,
        parse_requirements,
    )
        .map(|(indicators, _, schematics, _, requirements)| Machine {
            indicators,
            schematics,
            joltages: requirements,
        })
        .parse_next(input)
}

fn parse_requirements(input: &mut &str) -> ModalResult<Vec<usize>> {
    delimited('{', parse_int_list, '}').parse_next(input)
}

fn parse_schematics(input: &mut &str) -> ModalResult<Vec<Vec<usize>>> {
    separated(1.., parse_schematic, space1).parse_next(input)
}

fn parse_schematic(input: &mut &str) -> ModalResult<Vec<usize>> {
    delimited('(', parse_int_list, ')').parse_next(input)
}

fn parse_indicators(input: &mut &str) -> ModalResult<Vec<bool>> {
    delimited('[', repeat(1.., parse_indicator), ']').parse_next(input)
}

fn parse_indicator(input: &mut &str) -> ModalResult<bool> {
    alt(('.'.value(false), '#'.value(true))).parse_next(input)
}

fn parse_int_list(input: &mut &str) -> ModalResult<Vec<usize>> {
    separated(1.., parse_int, ',').parse_next(input)
}

fn parse_int(input: &mut &str) -> ModalResult<usize> {
    digit1.parse_to::<usize>().parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 33);
    }
}
