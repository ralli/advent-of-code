use day_19::{blueprints, max_geodes};

fn main() -> anyhow::Result<()> {
    let input = read_file("./day-19/input.txt")?;

    let result = part1(&input);
    println!("{}", result);

    let result = part2(&input);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> i32 {
    let (_, blueprints) = blueprints(input).unwrap();
    let max_steps = 24;
    blueprints
        .iter()
        .map(|bp| bp.id * max_geodes(bp, max_steps))
        .sum()
}

fn part2(input: &str) -> i32 {
    let (_, blueprints) = blueprints(input).unwrap();
    let max_steps = 32;
    blueprints
        .iter()
        .take(3)
        .map(|bp| max_geodes(bp, max_steps))
        .product()
}

fn read_file(filename: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test.txt");

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        let expected = 33;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        let expected = 12160;
        assert_eq!(result, expected);
    }
}
