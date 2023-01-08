extern crate core;

use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use anyhow::Context;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, multispace0};
use nom::combinator::{eof, map};
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair, terminated, tuple};
use nom::IResult;

fn main() -> anyhow::Result<()> {
    let filename = "./day-18/input.txt";
    let content = read_file(filename).context(filename)?;

    let result = part1(&content);
    println!("{}", result);

    let result = part2(&content);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> i32 {
    let (_, numbers) = input_data(input).unwrap();
    let first_number = numbers[0].clone();

    numbers
        .into_iter()
        .skip(1)
        .fold(first_number, add_and_reduce)
        .magnitude()
}

fn part2(input: &str) -> i32 {
    let (_, numbers) = input_data(input).unwrap();
    let size = numbers.len();
    let mut result = 0;

    for i in 0..size {
        for j in 0..size {
            if i != j {
                result =
                    result.max(add_and_reduce(numbers[i].clone(), numbers[j].clone()).magnitude());
            }
        }
    }

    result
}

#[derive(Debug, Clone)]
enum Number {
    Constant(i32),
    Pair(Rc<Number>, Rc<Number>),
}

impl Number {
    fn magnitude(&self) -> i32 {
        match self {
            Number::Constant(n) => *n,
            Number::Pair(lhs, rhs) => 3 * lhs.magnitude() + 2 * rhs.magnitude(),
        }
    }

    fn split(&self) -> Option<Rc<Number>> {
        match self {
            Number::Constant(n) => {
                if *n >= 10 {
                    Some(split(*n))
                } else {
                    None
                }
            }
            Number::Pair(lhs, rhs) => {
                if let Some(left) = lhs.split() {
                    Some(Rc::new(Number::Pair(left, rhs.clone())))
                } else {
                    rhs.split()
                        .map(|right| Rc::new(Number::Pair(lhs.clone(), right)))
                }
            }
        }
    }

    fn constant_value(&self) -> Option<i32> {
        match self {
            Number::Constant(n) => Some(*n),
            _ => None,
        }
    }
}

fn add_and_reduce(a: Rc<Number>, b: Rc<Number>) -> Rc<Number> {
    let added = Number::Pair(a, b);
    reduce(&added)
}

fn reduce(number: &Number) -> Rc<Number> {
    let (exploded, has_exploded) = explode(number);
    if has_exploded {
        reduce(&exploded)
    } else if let Some(splitted) = number.split() {
        reduce(&splitted)
    } else {
        Rc::new(number.clone())
    }
}

///
/// did not manage to implement the explode operation
/// adapted the scala solution of Florian Cassayre, which you can find here:
/// https://github.com/FlorianCassayre/AdventOfCode-2021/blob/master/src/main/scala/adventofcode/solutions/Day18.scala
///
fn explode(number: &Number) -> (Rc<Number>, bool) {
    // try to mimic the scala implementation with nested functions :-)
    fn accumulate(number: &Number, left_value: i32, right_value: i32) -> Rc<Number> {
        match number {
            Number::Constant(n) => Rc::new(Number::Constant(n + left_value + right_value)),
            Number::Pair(lhs, rhs) => Rc::new(Number::Pair(
                // if you come from the left, left_value = 0, right_value = 0 => just copy
                // if you come from the right, left_value > 0, right_value = 0
                //      => add left_value to the leftmost leaf, copy right subtree
                accumulate(lhs, left_value, 0),
                accumulate(rhs, 0, right_value),
            )),
        }
    }

    fn explode_n(level: i32, number: &Number, exploded: bool) -> (Rc<Number>, i32, i32, bool) {
        match number {
            Number::Constant(n) =>
            // constant reached but no explosion => the values to be added to the left and
            // right side are zero so the leftmost and rightmost leaves in the tree 
            // remain unchanged 
                (Rc::new(Number::Constant(*n)), 0, 0, exploded),
            Number::Pair(lhs, rhs) if level >= 4 && !exploded => {
                // the !exploded ensures, that at most only one node per traversal get exploded
                //
                // explode current node:
                //   - take the values if the children (which must be Constants)
                //   - return a Constant(0) Node
                //
                let left_value = lhs.constant_value().unwrap();
                let right_value = rhs.constant_value().unwrap();
                (Rc::new(Number::Constant(0)), left_value, right_value, true)
            }
            Number::Pair(lhs, rhs) => {
                //
                // exploded_l prevents, that the rhs gets exploded if the lhs already got exploded
                //
                let (left, ll, lr, exploded_l) = explode_n(level + 1, lhs, exploded);
                let (right, rl, rr, exploded_r) = explode_n(level + 1, rhs, exploded_l);
                (
                    Rc::new(Number::Pair(
                        // left: create a copy of the left subtree add value to the rightmost leave 
                        // of the subtree
                        accumulate(&left, 0, rl),
                        // right: create a copy of the right subtree add value to the leftmost leave of 
                        // the subtree
                        accumulate(&right, lr, 0),
                    )),
                    ll,
                    rr,
                    exploded_r,
                )
            }
        }
    }

    let (result, _, _, exploded) = explode_n(0, number, false);
    (result, exploded)
}

fn split(n: i32) -> Rc<Number> {
    let lhs = Rc::new(Number::Constant(n / 2));
    let rhs = Rc::new(Number::Constant(div_by_2_rounded_up(n)));
    Rc::new(Number::Pair(lhs, rhs))
}

fn div_by_2_rounded_up(n: i32) -> i32 {
    if n % 2 == 0 {
        n / 2
    } else {
        n / 2 + 1
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Number::Constant(n) => write!(f, "{}", n),
            Number::Pair(lhs, rhs) => write!(f, "[{},{}]", lhs, rhs),
        }
    }
}

fn input_data(input: &str) -> IResult<&str, Vec<Rc<Number>>> {
    terminated(
        separated_list1(line_ending, number),
        tuple((multispace0, eof)),
    )(input)
}

fn number(input: &str) -> IResult<&str, Rc<Number>> {
    alt((pair, constant))(input)
}

fn constant(input: &str) -> IResult<&str, Rc<Number>> {
    map(complete::i32, |n| Rc::new(Number::Constant(n)))(input)
}

fn pair(input: &str) -> IResult<&str, Rc<Number>> {
    map(
        delimited(tag("["), separated_pair(number, tag(","), number), tag("]")),
        |(lhs, rhs)| Rc::new(Number::Pair(lhs, rhs)),
    )(input)
}

fn read_file(name: &str) -> anyhow::Result<String> {
    let mut f = File::open(name)?;
    let mut result = String::new();
    f.read_to_string(&mut result)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_magnitude() {
        let examples = [
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ];
        for (input, expected) in examples {
            let (_, number) = number(input).unwrap();
            let result = number.magnitude();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_number_split() {
        let examples = [
            (
                "[[[[0,7],4],[15,[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            ),
            (
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
            ),
        ];
        for (input, expected) in examples {
            let (_, number) = number(input).unwrap();
            let result = format!("{}", number.split().unwrap());
            assert_eq!(result, expected);
        }
    }
    #[test]
    fn test_explode() {
        let examples = [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ];

        for (input, expected) in examples {
            let (_, number) = number(input).unwrap();
            let (exploded_number, exploded) = explode(&number);
            assert!(exploded);
            let result = format!("{}", exploded_number);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_split() {
        let examples = [(12, "[6,6]"), (11, "[5,6]")];
        for (n, expected) in examples {
            let result = format!("{}", split(n));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 4140;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 3993;
        assert_eq!(result, expected);
    }
}
