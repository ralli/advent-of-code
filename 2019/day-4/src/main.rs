fn main() {
    let result = part1();
    println!("{result}");

    let result = part2();
    println!("{result}");
}

fn part1() -> usize {
    let min = 172851;
    let max = 671087;

    (min..=max)
        .map(|i| format!("{}", i))
        .filter(|x| is_valid(&x))
        .count()
}

fn part2() -> usize {
    let min = 172851;
    let max = 671087;

    (min..=max)
        .map(|i| format!("{}", i))
        .filter(|x| is_valid2(&x))
        // .inspect(|x| println!("{x}"))
        .count()
}

fn is_valid(s: &str) -> bool {
    let mut doubles = false;
    for w in s.as_bytes().windows(2) {
        if w[1] < w[0] {
            return false;
        }
        if w[0] == w[1] {
            doubles = true;
        }
    }
    doubles
}

fn is_valid2(s: &str) -> bool {
    let b = s.as_bytes();

    if b.windows(2).any(|w| w[1] < w[0]) {
        return false;
    }

    let size = b.len();
    let mut i = 0;
    while i < size {
        let current = b[i];
        let mut count = 0;
        while i < size && b[i] == current {
            count += 1;
            i += 1;
        }
        if count == 2 {
            return true;
        }
    }

    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        // assert_eq!(is_valid("578999"), false);
        assert!(!is_valid2("578999"));
        assert!(is_valid2("578899"));
    }
}
