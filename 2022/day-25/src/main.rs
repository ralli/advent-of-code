fn main() -> anyhow::Result<()> {
    let input = read_file("./day-25/input.txt")?;

    let result = part1(&input);
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> String {
    let number: i64 = input.lines().map(from_snafu).sum();
    to_snafu(number)
}

fn from_snafu(input: &str) -> i64 {
    let mut result = 0;
    for c in input.chars() {
        let d = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            _ => unreachable!("{}", c),
        };
        result = result * 5 + d;
    }
    result
}

fn to_snafu(x: i64) -> String {
    if x == 0 {
        return String::new();
    }

    // 0 = 0
    // 1 = 1
    // 2 = 2
    // 3 = 1=
    // 4 = 1-

    let (digit, carry) = match x % 5 {
        0 => ('0', 0),
        1 => ('1', 0),
        2 => ('2', 0),
        3 => ('=', 1),
        4 => ('-', 1),
        _ => unreachable!("{}", x % 5),
    };

    let mut result = to_snafu(x / 5 + carry);
    result.push(digit);

    result
}

fn read_file(filename: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn from_snafu_works() {
        let numbers: Vec<_> = INPUT.lines().map(from_snafu).collect();
        let result: i64 = numbers.iter().sum();
        let expected = 4890;
        assert_eq!(result, expected);
    }
}
