fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day-07.txt")?;

    let result = part1(&input)?;
    println!("{result}");

    let result = part2(&input)?;
    println!("{result}");

    Ok(())
}

type Position = (usize, usize);

#[derive(Debug, Clone)]
struct Grid {
    values: Vec<Vec<char>>,
    width: usize,
    height: usize,
    start: Position,
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut result = 0;
    let grid = parse_grid(input)?;
    let mut current_beams = vec![false; grid.width];
    current_beams[grid.start.1] = true;
    for row in 1..grid.height {
        for col in 0..grid.width {
            if grid.values[row][col] == '^' {
                if current_beams[col] {
                    result += 1;
                    current_beams[col] = false;
                    current_beams[col - 1] = true;
                    current_beams[col + 1] = true;
                }
            }
        }
    }
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let grid = parse_grid(input)?;
    let mut current_beams: Vec<usize> = vec![0; grid.width];
    current_beams[grid.start.1] = 1;

    for row in 1..grid.height {
        for col in 0..grid.width {
            if grid.values[row][col] == '^' {
                if current_beams[col] != 0 {
                    let current = current_beams[col];
                    current_beams[col] = 0;
                    current_beams[col - 1] += current;
                    current_beams[col + 1] += current;
                }
            }
        }
    }

    Ok(current_beams.into_iter().sum())
}

fn parse_grid(input: &str) -> anyhow::Result<Grid> {
    let values: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();
    let width = values.first().map(|row| row.len()).unwrap_or(0);
    let height = values.len();
    let start_col = values
        .first()
        .map(|row| row.iter().position(|c| *c == 'S').unwrap_or_default())
        .unwrap_or(0);
    Ok(Grid {
        values,
        width,
        height,
        start: (0, start_col),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), 40);
    }
}
