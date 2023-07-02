use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    let content = read_file("./day-8/input.txt")?;

    let result = part1(&content)?;
    println!("{}", result);

    let result = part2(&content)?;
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    if grid.is_empty() || grid[0].is_empty() {
        return Err(anyhow!("empty grid"));
    }

    let mut result = 0;
    let height = grid.len();
    let width = grid[0].len();

    for row in 0..height {
        for col in 0..width {
            if is_visible(&grid, row, col) {
                result += 1;
            }
        }
    }

    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    if grid.is_empty() || grid[0].is_empty() {
        return Err(anyhow!("empty grid"));
    }

    let mut result = 0;
    let height = grid.len();
    let width = grid[0].len();

    for row in 0..height {
        for col in 0..width {
            let score = scenic_score(&grid, row, col);
            result = result.max(score);
        }
    }

    Ok(result)
}

fn is_visible(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let height = grid.len();
    let width = grid[0].len();

    if row == 0 || col == 0 || row + 1 == height || col + 1 == width {
        return true;
    }

    let value = grid[row][col];

    let visible_from_bottom = (0..row).all(|r| grid[r][col] < value);
    let visible_from_top = (row + 1..height).all(|r| grid[r][col] < value);
    let visible_from_left = (0..col).all(|c| grid[row][c] < value);
    let visible_from_right = (col + 1..width).all(|c| grid[row][c] < value);

    visible_from_bottom || visible_from_top || visible_from_left || visible_from_right
}

fn scenic_score(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    if row == 0 || col == 0 || row + 1 == height || col + 1 == width {
        return 0;
    }

    let value = grid[row][col];

    let score_to_bottom = (1..row).rev().take_while(|&r| grid[r][col] < value).count() + 1;

    let score_to_top = (row + 1..(height - 1))
        .take_while(|&r| grid[r][col] < value)
        .count()
        + 1;

    let score_to_left = (1..col).rev().take_while(|&c| grid[row][c] < value).count() + 1;

    let score_to_right = (col + 1..(width - 1))
        .take_while(|&c| grid[row][c] < value)
        .count()
        + 1;

    score_to_top * score_to_bottom * score_to_left * score_to_right
}

fn read_file(filename: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = test_input();
        let result = part1(&input).unwrap();
        assert_eq!(result, 21);
    }

    #[test]
    fn part2_works_2() {
        let input = test_input();
        let result = part2(&input).unwrap();
        assert_eq!(result, 8);
    }

    fn test_input() -> String {
        r#"30373
25512
65332
33549
35390"#
            .to_owned()
    }
}
