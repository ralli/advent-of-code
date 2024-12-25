use anyhow::anyhow;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alphanumeric1, line_ending, multispace0, space1};
use nom::combinator::{eof, value};
use nom::multi::{many1, separated_list0};
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::{BTreeMap, HashSet};

fn main() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("day-24/input.txt")?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let (_, (inputs, instructions)) = parse_input(input).map_err(|e| anyhow!("{e}"))?;
    // println!("{inputs:?}");
    // println!("{instructions:#?}");
    let mut wires = Wires::new(inputs, instructions);
    let mut outputs: Vec<_> = wires
        .outputs
        .iter()
        .cloned()
        .filter(|o| o.starts_with('z'))
        .collect();
    outputs.sort_by(|a, b| b.cmp(a));
    // println!("{:?}", outputs);
    let mut result = 0;
    for o in outputs.iter() {
        result <<= 1;
        let b = wires.get(o).unwrap() as usize;
        // println!("{o}: {b}");
        result |= b;
    }
    // println!("{result:012b}");
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<String> {
    let (_, (inputs, instructions)) = parse_input(input).map_err(|e| anyhow!("{e}"))?;
    let mut wires = Wires::new(inputs, instructions);
    let z_count = wires
        .outputs
        .iter()
        .filter(|out| out.starts_with('z'))
        .count();
    let z_max = z_count - 1;
    println!("z_max: {z_max}");
    let mut bads: Vec<_> = (2..z_max)
        .filter_map(|z_num| wires.check(z_num as i32))
        .collect();
    bads.sort();
    let result = bads.join(",");
    Ok(result)
}

#[derive(Debug, Clone)]
struct Wires<'a> {
    inputs: Inputs<'a>,
    instructions: BTreeMap<&'a str, Instruction<'a>>,
    outputs: Vec<&'a str>,
    cache: BTreeMap<&'a str, u8>,
}

impl<'a> Wires<'a> {
    fn new(inputs: Inputs<'a>, instructions: Instructions<'a>) -> Self {
        Self {
            inputs,
            outputs: instructions.iter().map(|i| i.result).collect(),
            instructions: instructions
                .into_iter()
                .fold(BTreeMap::new(), |mut acc, i| {
                    acc.insert(i.result, i);
                    acc
                }),
            cache: BTreeMap::new(),
        }
    }

    fn get(&mut self, key: &'a str) -> Option<u8> {
        if let Some(v) = self.cache.get(key) {
            return Some(*v);
        }
        if let Some(v) = self.inputs.get(key) {
            return Some(*v);
        }
        if let Some(instruction) = self.instructions.get(key).copied() {
            let (left_value, right_value) = self
                .get(instruction.left)
                .and_then(|a| self.get(instruction.right).map(|b| (a, b)))
                .unwrap();
            let v = match instruction.op {
                Op::AND => left_value & right_value,
                Op::OR => left_value | right_value,
                Op::XOR => left_value ^ right_value,
            };
            self.cache.insert(key, v);
            return Some(v);
        }
        None
    }

    // by far more checks needed, but works on my input
    // returns the name of a wire to swap or None...
    fn check(&self, num: i32) -> Option<String> {
        let z_key = key_for('z', num);
        let inst = self.instructions.get(z_key.as_str()).unwrap();

        if inst.op != Op::XOR {
            return Some(z_key);
        }

        let mut not_seen = HashSet::from([Op::XOR, Op::OR]);
        let left_inst = self.instructions.get(inst.left).unwrap();
        if !not_seen.contains(&left_inst.op) {
            return Some(inst.left.to_string());
        }
        not_seen.remove(&left_inst.op);

        if left_inst.op == Op::XOR && !is_x_and_y(left_inst.left, left_inst.right, num) {
            return Some(inst.left.to_string());
        }

        if left_inst.op == Op::OR {
            let id = left_inst.left.to_string();
            let ll_inst = self.instructions.get(id.as_str()).unwrap();
            if ll_inst.op != Op::AND {
                return Some(id);
            }
        }

        let right_inst = self.instructions.get(inst.right).unwrap();
        if !not_seen.contains(&right_inst.op) {
            return Some(inst.right.to_string());
        }

        if right_inst.op == Op::XOR && !is_x_and_y(right_inst.left, right_inst.right, num) {
            return Some(inst.right.to_string());
        }

        if right_inst.op == Op::OR {
            let id = right_inst.left.to_string();
            let rl_inst = self.instructions.get(id.as_str()).unwrap();
            if rl_inst.op != Op::AND {
                return Some(id);
            }
        }

        None
    }
}

fn is_x_and_y(lhs: &str, rhs: &str, num: i32) -> bool {
    let k1 = key_for('x', num);
    let k2 = key_for('y', num);
    (lhs == k1 && rhs == k2) || (lhs == k2 && rhs == k1)
}

fn key_for(prefix: char, num: i32) -> String {
    format!("{}{:02}", prefix, num)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Op {
    AND,
    XOR,
    OR,
}

#[derive(Debug, Clone, Copy)]
struct Instruction<'a> {
    op: Op,
    left: &'a str,
    right: &'a str,
    result: &'a str,
}

type Inputs<'a> = BTreeMap<&'a str, u8>;
type Instructions<'a> = Vec<Instruction<'a>>;

fn parse_input(input: &str) -> IResult<&str, (Inputs, Instructions)> {
    let (input, inputs) = parse_inputs(input)?;
    let (input, _) = many1(line_ending)(input)?;
    let (input, instructions) = parse_instructions(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = eof(input)?;

    Ok((input, (inputs, instructions)))
}

fn parse_instructions(input: &str) -> IResult<&str, Instructions> {
    let (input, instructions) = separated_list0(line_ending, parse_instruction)(input)?;
    Ok((input, instructions))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, left) = alphanumeric1(input)?;
    let (input, _) = space1(input)?;
    let (input, op) = parse_op(input)?;
    let (input, _) = space1(input)?;
    let (input, right) = alphanumeric1(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("->")(input)?;
    let (input, _) = space1(input)?;
    let (input, result) = alphanumeric1(input)?;
    Ok((
        input,
        Instruction {
            op,
            left,
            right,
            result,
        },
    ))
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    alt((
        value(Op::AND, tag("AND")),
        value(Op::XOR, tag("XOR")),
        value(Op::OR, tag("OR")),
    ))(input)
}

fn parse_inputs(input: &str) -> IResult<&str, BTreeMap<&str, u8>> {
    let (input, inputs) = separated_list0(line_ending, parse_input_value)(input)?;
    let inputs = BTreeMap::from_iter(inputs);
    Ok((input, inputs))
}

fn parse_input_value(input: &str) -> IResult<&str, (&str, u8)> {
    separated_pair(alphanumeric1, tag(": "), complete::u8)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        println!("{result}");
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let _result = part2(INPUT)?;
        // println!("{result}");
        Ok(())
    }
}
