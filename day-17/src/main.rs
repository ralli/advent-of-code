use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Formatter;

fn main() -> anyhow::Result<()> {
    let input = read_file("./day-17/input.txt")?.trim().to_owned();

    let result = part1(&input);
    println!("{}", result);

    let result = part2(&input);
    println!("{}", result);

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
                grid.draw(row, col, rock);
                break;
            }
        }
    }

    grid.height()
}

fn part2(input: &str) -> i64 {
    let rocks = create_rocks();
    let commands: Vec<char> = input.chars().filter(|&c| c == '<' || c == '>').collect();
    let mut rock_iter = rocks.iter().enumerate().cycle();
    let mut command_iter = commands.iter().cycle();
    let mut grid = Grid::new();
    let mut cache: HashMap<(usize, String), i64> = HashMap::new();
    let mut heights = vec![0i64];
    let mut round_and_heights = Vec::new();
    let mut check_cached_round = -1;
    let max_rounds = 1_000_000_000_000i64;

    for round in 0..max_rounds {
        let (rock_idx, rock) = rock_iter.next().unwrap();
        let mut row = grid.height() + 3;
        let mut col = 2;

        loop {
            let command = command_iter.next().copied().unwrap();
            let next_col = if command == '>' { col + 1 } else { col - 1 };
            if grid.can_draw(row, next_col, rock) {
                col = next_col;
            }

            if grid.can_draw(row - 1, col, rock) {
                row -= 1;
            } else {
                grid.draw(row, col, rock);
                break;
            }
        }

        let height = grid.height();
        heights.push(height as i64);

        let key = (rock_idx, grid.get_hash_string());
        if let Some(cached_round) = cache.get(&key) {
            if check_cached_round == -1 {
                check_cached_round = *cached_round;
            }
            if check_cached_round == *cached_round {
                // println!(
                //     "found cycle in round={}, height={}, cached_round={}, cached_height={} rock_idx={}",
                //     round, height, cached_round, cached_height, rock_idx
                // );
                round_and_heights.push((round, height));
                if round_and_heights.len() == 2 {
                    break;
                }
            }
        } else {
            cache.insert(key, round);
        }
    }

    let start_idx: i64 = round_and_heights[0].0;
    let start_height: i64 = round_and_heights[0].1 as i64;
    let idx_diff: i64 = round_and_heights[1].0 - round_and_heights[0].0;
    let height_diff: i64 = (round_and_heights[1].1 - round_and_heights[0].1) as i64;

    let rest_idx = (max_rounds - start_idx) % idx_diff;
    let rest_height_diff = heights[(start_idx + rest_idx) as usize] - start_height;

    // dbg!(
    //     start_idx,
    //     start_height,
    //     idx_diff,
    //     height_diff,
    //     max_rounds,
    //     rest_idx,
    //     rest_height_diff
    // );

    start_height + (max_rounds - start_idx) / idx_diff * height_diff + rest_height_diff
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

    /// the top 20 lines of the grid (including air and border etc.)
    /// used for cycle detection
    fn get_hash_string(&self) -> String {
        let h = self.height();
        let w = self.width();
        let mut result = String::new();

        for r in (h - 20)..h {
            for c in 0..w {
                result.push(self.get_char(r, c));
            }
        }

        result
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
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 1514285714288i64;
        assert_eq!(result, expected);
    }
}
