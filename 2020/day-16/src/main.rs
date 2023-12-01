use anyhow::{anyhow, Context};
use itertools::Itertools;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete;
use nom::character::complete::{line_ending, multispace0};
use nom::combinator::all_consuming;
use nom::multi::{many1, separated_list0, separated_list1};
use nom::sequence::terminated;
use nom::IResult;
use std::fs;

fn main() -> anyhow::Result<()> {
    let filename = "day-16.txt";
    let input = fs::read_to_string(filename).with_context(|| format!("cannot read {filename}"))?;
    let result = part1(&input)?;
    println!("{result}");
    Ok(())
}

#[derive(Debug)]
struct Problem {
    fields: Vec<Field>,
    your_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

impl Problem {
    fn is_valid_ticket(&self, ticket: &[u32]) -> bool {
        ticket.iter().all(|value| self.is_valid_value(*value))
    }

    fn is_valid_value(&self, value: u32) -> bool {
        self.fields.iter().any(|f| f.is_valid(value))
    }
}

#[derive(Debug)]
struct Field {
    name: String,
    constraints: Vec<Constraint>,
}

impl Field {
    fn is_valid(&self, value: u32) -> bool {
        self.constraints
            .iter()
            .all(|constraint| constraint.is_valid(value))
    }
}
#[derive(Debug)]
struct Constraint {
    from: u32,
    to: u32,
}

impl Constraint {
    fn is_valid(&self, value: u32) -> bool {
        self.from <= value && self.to >= value
    }
}

fn part1(input: &str) -> anyhow::Result<u32> {
    let problem = parse_input(input)?;
    let all_constraints: Vec<&Constraint> =
        problem.fields.iter().flat_map(|f| &f.constraints).collect();
    let scanning_error_rate: u32 = problem
        .nearby_tickets
        .iter()
        .flat_map(|ticket| {
            ticket
                .iter()
                .filter(|&value| !all_constraints.iter().any(|c| c.is_valid(*value)))
        })
        .copied()
        .sum();
    Ok(scanning_error_rate)
}

fn part2(input: &str) -> anyhow::Result<u64> {
    let problem = parse_input(input)?;
    let valid_tickets: Vec<Vec<u32>> = problem
        .nearby_tickets
        .into_iter()
        .filter(|ticket| problem.is_valid_ticket(ticket.as_slice()))
        .collect();
    dbg!(&valid_tickets);
    let departure_fields: Vec<Field> = problem
        .fields
        .into_iter()
        .filter(|field| field.name.starts_with("departure"))
        .collect();
    let num_fields = problem.your_ticket.len();
    let field_assignments: Vec<(&Field, usize)> = departure_fields
        .iter()
        .cartesian_product(0..num_fields)
        .filter(|(field, idx)| {
            valid_tickets
                .iter()
                .all(|ticket| field.is_valid(ticket[*idx]))
        })
        .collect();
    for (f, i) in field_assignments.iter() {
        println!("{}, {}", f.name, i);
    }
    // dbg!(&field_assignments);
    let ticket_values: u64 = field_assignments
        .iter()
        .map(|(_, idx)| problem.your_ticket[*idx] as u64)
        .product();
    Ok(ticket_values)
}

fn parse_input(input: &str) -> anyhow::Result<Problem> {
    let (_, problem) = all_consuming(terminated(problem, multispace0))(input)
        .map_err(|e| anyhow!(e.to_string()))?;
    Ok(problem)
}

fn problem(input: &str) -> IResult<&str, Problem> {
    let (input, fields) = terminated(fields, many1(line_ending))(input)?;
    let (input, your_ticket) = terminated(your_ticket, many1(line_ending))(input)?;
    let (input, nearby_tickets) = nearby_tickets(input)?;
    Ok((
        input,
        Problem {
            fields,
            your_ticket,
            nearby_tickets,
        },
    ))
}

fn nearby_tickets(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, _) = terminated(tag("nearby tickets:"), line_ending)(input)?;
    separated_list1(line_ending, number_list)(input)
}

fn your_ticket(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = terminated(tag("your ticket:"), line_ending)(input)?;
    terminated(number_list, line_ending)(input)
}

fn number_list(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), complete::u32)(input)
}

fn fields(input: &str) -> IResult<&str, Vec<Field>> {
    separated_list0(line_ending, field)(input)
}

fn field(input: &str) -> IResult<&str, Field> {
    let (input, name) = take_until(":")(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, constraints) = constraints(input)?;
    Ok((
        input,
        Field {
            name: name.to_string(),
            constraints,
        },
    ))
}

fn constraints(input: &str) -> IResult<&str, Vec<Constraint>> {
    separated_list0(tag(" or "), contstraint)(input)
}

fn contstraint(input: &str) -> IResult<&str, Constraint> {
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, to) = complete::u32(input)?;
    Ok((input, Constraint { from, to }))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        let expected = 71;
        assert_eq!(result, expected);
        Ok(())
    }

    static INPUT2: &str = r#"departure class: 0-1 or 4-19
departure row: 0-5 or 8-19
departure seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"#;

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let _ = part2(INPUT2)?;
        Ok(())
    }
}
