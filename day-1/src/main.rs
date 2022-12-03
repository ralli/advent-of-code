use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let lines = read_lines("./input.txt")?;
    let mut current_sum = 0;
    let mut sums = Vec::new();
    for line in lines {
        let line = line?;
        if line.trim().len() > 0 {
            let value = line.parse::<i32>()?;
            current_sum += value;
        } else {
            sums.push(current_sum);
            current_sum = 0;
        }
    }

    sums.sort_by(|a, b| b.cmp(a));
    let total: i32 = sums.iter().take(3).sum();
    println!("{}", total);
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
