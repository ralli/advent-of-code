use std::collections::hash_set::HashSet;
use std::hash::Hash;

fn main() -> anyhow::Result<()> {
    let filename = "./day-3/input.txt";
    let content = read_file(filename)?;
    part1(&content);
    part2(&content);
    Ok(())
}

fn part1(content: &str) {
    let lines = content.lines();
    let mut sum = 0;

    for line in lines {
        let (s1, s2) = compartments(line);
        let m1: HashSet<_> = s1.chars().collect();
        let m2: HashSet<_> = s2.chars().collect();
        let dups: String = m1.intersection(&m2).copied().collect();
        let priority = dups.chars().next().map(|c| priority(c)).unwrap_or(0);
        sum += priority;
    }

    println!("{}", sum);
}

fn part2(content: &str) {
    let lines: Vec<_> = content.lines().collect();
    let chunks = lines.chunks_exact(3);
    let mut sum = 0;

    for chunk in chunks {
        let m1: HashSet<char> = chunk[0].chars().collect();
        let m2: HashSet<char> = chunk[1].chars().collect();
        let m3: HashSet<char> = chunk[2].chars().collect();
        let dups: String = m1
            .intersection(&m2)
            .copied()
            .collect::<HashSet<_>>()
            .intersection(&m3)
            .collect();
        let priority = dups.chars().next().map(|c| priority(c)).unwrap_or(0);
        sum += priority;
    }

    println!("{}", sum);
}

fn priority(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 'a' as usize + 1,
        'A'..='Z' => c as usize - 'A' as usize + 27,
        _ => 0,
    }
}

fn compartments(s: &str) -> (&str, &str) {
    let l = s.len();
    (&s[0..(l / 2)], &s[(l / 2)..])
}

fn read_file(filename: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}
