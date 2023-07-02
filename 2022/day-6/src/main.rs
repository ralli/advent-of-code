use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let content = read_file("./day-6/input.txt")?;

    let result = part1(&content).unwrap_or(0);
    println!("{}", result);

    let result = part2(&content).unwrap_or(0);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> Option<usize> {
    let a: Vec<char> = input.chars().collect();
    let result = a.windows(4).enumerate().find(|(_, w)| {
        w[0] != w[1] && w[0] != w[2] && w[0] != w[3] && w[1] != w[2] && w[1] != w[3] && w[2] != w[3]
    });

    result.map(|(idx, _)| idx + 4)
}

fn part2(input: &str) -> Option<usize> {
    let a: Vec<char> = input.chars().collect();
    let result = a.windows(14).enumerate().find(|(_, w)| is_all_distinct(w));

    result.map(|(idx, _)| idx + 14)
}

fn is_all_distinct(input: &[char]) -> bool {
    let mut m = HashSet::with_capacity(1024);

    for c in input.iter() {
        if m.contains(c) {
            return false;
        }
        m.insert(*c);
    }

    true
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
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = part1(&input).unwrap();
        let expected = 7;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_works_2() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = part2(&input).unwrap();
        let expected = 19;
        assert_eq!(result, expected);
    }
}
