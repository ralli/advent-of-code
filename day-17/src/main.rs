use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;

fn main() -> anyhow::Result<()> {
    let input = read_file("./day-17/input.txt")?.trim().to_owned();

    let result = part1(&input);
    println!("{}", result);

    // let result = part2(&input);
    // println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> i32 {
    let rocks = create_rocks();
    let commands: Vec<char> = input.chars().filter(|&c| c == '<' || c == '>').collect();
    let mut rock_iter = rocks.iter().cycle();
    let mut command_iter = commands.iter().cycle();
    let mut grid = Grid::new();

    for _round in 0..2022 {
        let rock = rock_iter.next().unwrap();
        let mut row = grid.height() + 3;
        let mut col = 2;

        // println!("round {}", _round + 1);
        // println!("========================================\n");
        //
        // grid.draw(row, col, rock);
        // println!("{}\n", &grid);
        // grid.remove(row, col, rock);

        loop {
            let command = command_iter.next().copied().unwrap();
            let next_col = if command == '>' { col + 1 } else { col - 1 };
            if grid.can_draw(row, next_col, rock) {
                col = next_col;
            }

            if grid.can_draw(row - 1, col, rock) {
                row -= 1;
            } else {
                grid.draw(row, col, &rock);
                break;
            }
        }
    }

    grid.height()
}

fn part2(_input: &str) -> i32 {
    todo!()
}

#[derive(Debug)]
struct Grid {
    state: HashSet<(i32, i32)>,
}

impl Grid {
    fn new() -> Self {
        Self {
            state: HashSet::new(),
        }
    }

    fn get_char(&self, r: i32, c: i32) -> char {
        if r < 0 && !(0..7).contains(&c) {
            return '+';
        }

        if !(0..7).contains(&c) {
            return '|';
        }

        if r < 0 {
            return '-';
        }

        if self.state.contains(&(r, c)) {
            '#'
        } else {
            '.'
        }
    }

    fn can_draw(&mut self, row: i32, col: i32, rock: &Rock) -> bool {
        for (i, r) in rock.rows.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c != '.' && self.get_char(row + i as i32, col + j as i32) != '.' {
                    return false;
                }
            }
        }
        true
    }

    fn draw(&mut self, row: i32, col: i32, rock: &Rock) {
        for (i, r) in rock.rows.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c != '.' {
                    self.state.insert((row + i as i32, col + j as i32));
                }
            }
        }
    }

    fn remove(&mut self, row: i32, col: i32, rock: &Rock) {
        for (i, r) in rock.rows.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c != '.' {
                    self.state.remove(&(row + i as i32, col + j as i32));
                }
            }
        }
    }

    fn width(&self) -> i32 {
        7
    }

    fn height(&self) -> i32 {
        if self.state.is_empty() {
            return 0;
        }
        self.state
            .iter()
            .map(|(r, _)| r)
            .max()
            .copied()
            .unwrap_or(0)
            + 1
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let height = self.height();
        let width = self.width();

        for r in (-1..height).rev() {
            for c in -1..=width {
                let c = self.get_char(r, c);
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Rock {
    rows: Vec<Vec<char>>,
}

impl Rock {
    fn width(&self) -> i32 {
        self.rows[0].len() as i32
    }

    fn height(&self) -> i32 {
        self.rows.len() as i32
    }
}

fn create_rocks() -> Vec<Rock> {
    let defs = vec![
        vec!["####"],
        vec![".#.", "###", ".#."],
        vec!["..#", "..#", "###"],
        vec!["#", "#", "#", "#"],
        vec!["##", "##"],
    ];

    defs.into_iter()
        .map(|stone| {
            stone
                .into_iter()
                .rev()
                .map(|line| line.chars().collect())
                .collect()
        })
        .map(|v| Rock { rows: v })
        .collect()
}
fn read_file(filename: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 3068;
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 1707;
        assert_eq!(result, expected);
    }
}
