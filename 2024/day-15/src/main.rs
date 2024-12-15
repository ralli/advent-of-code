use anyhow::{anyhow, Context};
use nom::character::complete::{line_ending, multispace0, one_of};
use nom::multi::{many0, many1, separated_list0};
use nom::sequence::preceded;
use nom::IResult;
use std::collections::{BTreeSet, VecDeque};
use std::fmt::Formatter;
use std::{fmt, fs};

fn main() -> anyhow::Result<()> {
    let filename = "day-15/input.txt";
    let content = fs::read_to_string(filename).context(format!("cannot load {filename}"))?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(_input: &str) -> anyhow::Result<usize> {
    let (_, mut warehouse) = parse_warehouse(_input).map_err(|e| anyhow!("{e}"))?;

    for &command in warehouse.commands.iter() {
        move_robot(command, &mut warehouse.grid);
    }
    let mut result = 0;
    for (row_idx, row) in warehouse.grid.cells.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if *col == 'O' {
                result += 100 * row_idx + col_idx;
            }
        }
    }
    Ok(result)
}

fn part2(_input: &str) -> anyhow::Result<usize> {
    let (_, warehouse) = parse_warehouse(_input).map_err(|e| anyhow!("{e}"))?;
    let mut grid = warehouse.grid.convert();

    for &command in warehouse.commands.iter() {
        move_robot2(command, &mut grid);
    }
    let mut result = 0;
    for (row_idx, row) in grid.cells.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if *col == '[' {
                result += 100 * row_idx + col_idx;
            }
        }
    }
    Ok(result)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Warehouse {
    grid: Grid,
    commands: Vec<char>,
}

fn move_robot2(command: char, grid: &mut Grid) {
    if perform_move2(command, grid.robot_row, grid.robot_col, grid) {
        let (dr, dc) = command_direction(command);
        grid.robot_row += dr;
        grid.robot_col += dc;
    }
}

fn perform_move2(command: char, start_row: isize, start_col: isize, grid: &mut Grid) -> bool {
    let mut visited = BTreeSet::new();
    let mut q = VecDeque::from([(start_row, start_col)]);
    let mut moves = Vec::new();
    let (dr, dc) = command_direction(command);

    while let Some((row, col)) = q.pop_front() {
        if !visited.insert((row, col)) {
            continue;
        }
        let c = grid.cells[row as usize][col as usize];
        if c == '.' {
            continue;
        }
        if c == '#' {
            return false;
        }
        if c == '[' && dr != 0 {
            moves.push((row, col + 1, c, true));
            q.push_back((row, col + 1));
        } else if c == ']' && dr != 0 {
            moves.push((row, col - 1, c, true));
            q.push_back((row, col - 1));
        }
        moves.push((row + dr, col + dc, c, false));
        q.push_back((row + dr, col + dc));
    }
    for &(row, col, c, replace_with_blank) in moves.iter().rev() {
        grid.cells[row as usize][col as usize] = c;
        if replace_with_blank {
            grid.cells[row as usize][col as usize] = '.';
        }
    }
    grid.cells[start_row as usize][start_col as usize] = '.';
    true
}

fn move_robot(command: char, grid: &mut Grid) -> bool {
    if perform_move(command, grid.robot_row, grid.robot_col, grid) {
        grid.cells[grid.robot_row as usize][grid.robot_col as usize] = '.';
        let (dr, dc) = command_direction(command);
        grid.robot_row += dr;
        grid.robot_col += dc;
        return true;
    }
    false
}

fn perform_move(command: char, row: isize, col: isize, grid: &mut Grid) -> bool {
    let cell = grid.cells[row as usize][col as usize];
    if cell == '#' {
        return false;
    }
    if cell == '.' {
        return true;
    }
    let (dr, dc) = command_direction(command);
    let (next_row, next_col) = (row + dr, col + dc);
    let result = perform_move(command, next_row, next_col, grid);
    if result {
        grid.cells[next_row as usize][next_col as usize] = cell;
    }
    result
}

fn command_direction(command: char) -> (isize, isize) {
    match command {
        '<' => (0, -1),
        '>' => (0, 1),
        '^' => (-1, 0),
        'v' => (1, 0),
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Grid {
    width: isize,
    height: isize,
    robot_row: isize,
    robot_col: isize,
    cells: Vec<Vec<char>>,
}

impl Grid {
    fn convert(&self) -> Grid {
        let mut cells = Vec::new();
        for row in self.cells.iter() {
            let mut new_row = Vec::new();
            for col in row.iter() {
                match col {
                    'O' => {
                        new_row.push('[');
                        new_row.push(']');
                    }
                    '.' => {
                        new_row.push('.');
                        new_row.push('.');
                    }
                    '@' => {
                        new_row.push('@');
                        new_row.push('.');
                    }
                    '#' => {
                        new_row.push('#');
                        new_row.push('#');
                    }
                    _ => unreachable!(),
                }
            }
            cells.push(new_row);
        }
        let width = cells.first().map(|r| r.len()).unwrap_or_default() as isize;
        let height = cells.len() as isize;
        let (start_row, row) = cells
            .iter()
            .enumerate()
            .find(|(_, r)| r.contains(&'@'))
            .unwrap();
        let start_col = row.iter().position(|c| *c == '@').unwrap_or_default() as isize;
        Grid {
            width,
            height,
            robot_row: start_row as isize,
            robot_col: start_col,
            cells,
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in self.cells.iter() {
            for cell in row.iter() {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_warehouse(input: &str) -> IResult<&str, Warehouse> {
    let (rest, grid) = parse_grid(input)?;
    let (rest, _) = many1(line_ending)(rest)?;
    let (rest, commands) = parse_commands(rest)?;
    let (rest, _) = multispace0(rest)?;
    Ok((rest, Warehouse { grid, commands }))
}

fn parse_grid(input: &str) -> IResult<&str, Grid> {
    let (rest, cells) = separated_list0(line_ending, parse_grid_line)(input)?;
    let width = cells.first().map(|r| r.len()).unwrap_or_default() as isize;
    let height = cells.len() as isize;
    let (start_row, row) = cells
        .iter()
        .enumerate()
        .find(|(_, r)| r.contains(&'@'))
        .unwrap();
    let start_col = row.iter().position(|c| *c == '@').unwrap_or_default() as isize;
    Ok((
        rest,
        Grid {
            width,
            height,
            robot_row: start_row as isize,
            robot_col: start_col,
            cells,
        },
    ))
}

fn parse_grid_line(input: &str) -> IResult<&str, Vec<char>> {
    many1(one_of("#.O@"))(input)
}

fn parse_commands(input: &str) -> IResult<&str, Vec<char>> {
    many0(preceded(many0(line_ending), one_of("v<^>")))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        assert_eq!(result, 10092);
        Ok(())
    }

    #[test]
    fn part2_works() -> anyhow::Result<()> {
        let result = part2(INPUT)?;
        assert_eq!(result, 9021);
        Ok(())
    }
}
