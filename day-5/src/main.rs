use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, satisfy, space0, space1},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

fn main() -> anyhow::Result<()> {
    let content = read_file("./day-5/input.txt")?;

    let result = part1(&content)?;
    println!("{}", result);

    let result = part2(&content)?;
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<String> {
    let (_input, mut document) = document(input).unwrap();
    let moves = document.moves.clone();
    for m in moves.iter() {
        document.perform_move(m);
    }

    let result = document
        .stacks
        .iter()
        .map(|s| s.last().unwrap())
        .collect::<String>();
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<String> {
    let (_input, mut document) = document(input).unwrap();
    let moves = document.moves.clone();
    for m in moves.iter() {
        document.perform_move_9001(m);
    }

    let result = document
        .stacks
        .iter()
        .map(|s| s.last().unwrap())
        .collect::<String>();
    Ok(result)
}

fn read_file(filename: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}

fn crate_item(input: &str) -> IResult<&str, char> {
    use nom::character::complete::char as char_parser;
    let first = delimited(tag("["), satisfy(|c| c.is_alphabetic()), tag("]"));
    let second = delimited(char_parser(' '), char_parser(' '), char_parser(' '));
    let (input, c) = alt((first, second))(input)?;
    Ok((input, c))
}

fn crate_item_line(input: &str) -> IResult<&str, Vec<char>> {
    use nom::character::complete::char as char_parser;
    let (input, result) = separated_list1(char_parser(' '), crate_item)(input)?;
    Ok((input, result))
}

fn crate_item_lines(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (input, result) = separated_list1(line_ending, crate_item_line)(input)?;
    Ok((input, result))
}

#[derive(Debug, Clone, Copy)]
struct Move {
    quantity: u32,
    from: u32,
    to: u32,
}

fn move_command(input: &str) -> IResult<&str, Move> {
    use nom::character::complete::char as char_parser;
    use nom::character::complete::u32 as u32_parser;
    let (input, (_, _, quantity, _, _, _, from, _, _, _, to)) = tuple((
        tag("move"),
        char_parser(' '),
        u32_parser,
        char_parser(' '),
        tag("from"),
        char_parser(' '),
        u32_parser,
        char_parser(' '),
        tag("to"),
        char_parser(' '),
        u32_parser,
    ))(input)?;
    let result = Move { quantity, from, to };
    Ok((input, result))
}

#[derive(Debug, Clone)]
struct Document {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

fn document(input: &str) -> IResult<&str, Document> {
    let (input, (lines, _, _, _, _, moves)) = tuple((
        crate_item_lines,
        line_ending,
        tuple((space0, separated_list1(space1, digit1), space0)),
        line_ending,
        line_ending,
        separated_list1(line_ending, move_command),
    ))(input)?;
    Ok((
        input,
        Document {
            stacks: build_stacks(&lines),
            moves,
        },
    ))
}

fn build_stacks(lines: &[Vec<char>]) -> Vec<Vec<char>> {
    let lines: Vec<Vec<char>> = lines.iter().cloned().rev().collect();

    if lines.is_empty() {
        return Vec::new();
    }

    let mut result: Vec<Vec<char>> = vec![Vec::new(); lines[0].len()];

    for line in lines {
        for (i, &c) in line.iter().enumerate() {
            if !c.is_whitespace() {
                result[i].push(c);
            }
        }
    }

    result
}

impl Document {
    fn perform_move(&mut self, move_command: &Move) {
        let from = (move_command.from - 1) as usize;
        let to = (move_command.to - 1) as usize;
        let mut items = Vec::new();

        for _ in 0..move_command.quantity {
            items.push(self.stacks[from].pop().unwrap());
        }

        for c in items.into_iter() {
            self.stacks[to].push(c);
        }
    }

    fn perform_move_9001(&mut self, move_command: &Move) {
        let from = (move_command.from - 1) as usize;
        let to = (move_command.to - 1) as usize;
        let mut items = Vec::new();

        for _ in 0..move_command.quantity {
            items.push(self.stacks[from].pop().unwrap());
        }

        for c in items.into_iter().rev() {
            self.stacks[to].push(c);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_item_line() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let result = part1(input).unwrap();
        let expected = "CMZ".to_owned();
        assert_eq!(result, expected);
    }
}
