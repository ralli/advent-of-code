use day_24::{board, Board, Position};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

fn main() -> anyhow::Result<()> {
    let input = read_file("./day-24/input.txt")?;

    let result = part1(&input);
    println!("{}", result);

    let result = part2(&input);
    println!("{}", result);

    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Entry {
    minute: i32,
    position: Position,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.minute.cmp(&self.minute) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => other.position.cmp(&self.position),
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const DIRECTIONS: [(i32, i32); 5] = [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)];

fn part1(input: &str) -> i32 {
    let (_, board) = board(input).unwrap();
    let start_pos = Position::new(0, 1);
    let end_pos = Position::new(board.height - 1, board.width - 2);

    find_path(&board, start_pos, end_pos).unwrap()
}

fn part2(input: &str) -> i32 {
    let (_, board) = board(input).unwrap();
    let start_pos = Position::new(0, 1);
    let end_pos = Position::new(board.height - 1, board.width - 2);

    let first = find_path(&board, start_pos, end_pos).unwrap();
    let second_board = board.at_minute(first);
    let second = find_path(&second_board, end_pos, start_pos).unwrap();
    let third_board = board.at_minute(first + second);
    let third = find_path(&third_board, start_pos, end_pos).unwrap();

    first + second + third
}
fn find_path(board: &Board, start_pos: Position, end_pos: Position) -> Option<i32> {
    let initial = Entry {
        minute: 0,
        position: start_pos,
    };
    let mut q: BinaryHeap<Entry> = BinaryHeap::from([initial]);
    let mut visited = HashSet::from([initial]);
    let board_cache: HashMap<i32, Board> = (0..1000)
        .map(|minute| (minute, board.at_minute(minute)))
        .collect();

    while let Some(current) = q.pop() {
        if current.position == end_pos {
            return Some(current.minute);
        }
        let next_minute = current.minute + 1;
        let next_board = board_cache.get(&next_minute).unwrap();
        for &d in DIRECTIONS.iter() {
            let next_position = current.position.plus(d);
            let next_entry = Entry {
                minute: next_minute,
                position: next_position,
            };
            if visited.insert(next_entry)
                && (0..next_board.width).contains(&next_position.col)
                && (0..next_board.height).contains(&next_position.row)
                && next_board
                    .get(next_position.row, next_position.col)
                    .is_none()
            {
                q.push(next_entry);
            }
        }
    }
    None
}

fn read_file(filename: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
    //     const INPUT: &str = "#.#####
    // #.....#
    // #>....#
    // #.....#
    // #...v.#
    // #.....#
    // #####.#";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 18;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 54;
        assert_eq!(result, expected);
    }
}
