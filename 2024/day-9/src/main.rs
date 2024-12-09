use anyhow::Context;
use std::fs;

fn main() -> anyhow::Result<()> {
    let filename = "day-9/input.txt";
    let content = fs::read_to_string(filename).context(format!("cannot read {filename}"))?;

    let result = part1(&content)?;
    println!("{result}");

    let result = part2(&content)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let input = parse_input(input);
    let mut blocks: Vec<i32> = Vec::new();
    let mut file_id = 0;

    for (idx, &fs_size) in input.iter().enumerate() {
        if idx % 2 == 0 {
            for _ in 0..fs_size {
                blocks.push(file_id);
            }
            file_id += 1;
        } else {
            for _ in 0..fs_size {
                blocks.push(-1);
            }
        }
    }

    let mut check_sum = 0;
    let mut l = 0;
    let mut r = blocks.len() - 1;

    while l < r {
        while blocks[l] >= 0 {
            check_sum += (blocks[l] as usize) * l;
            l += 1;
        }
        while blocks[r] < 0 {
            r -= 1;
        }
        blocks[l] = blocks[r];
        blocks[r] = -1;
    }

    Ok(check_sum)
}

struct BlockInfo {
    start: usize,
    len: usize,
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let input = parse_input(input);
    let mut blocks: Vec<i32> = Vec::new();
    let mut file_id = 0;

    let mut file_infos: Vec<BlockInfo> = Vec::new();
    let mut free_infos: Vec<BlockInfo> = Vec::new();

    for (idx, &fs_size) in input.iter().enumerate() {
        if idx % 2 == 0 {
            file_infos.push(BlockInfo {
                start: blocks.len(),
                len: fs_size as usize,
            });
            for _ in 0..fs_size {
                blocks.push(file_id);
            }
            file_id += 1;
        } else {
            if fs_size > 0 {
                free_infos.push(BlockInfo {
                    start: blocks.len(),
                    len: fs_size as usize,
                });
            }
            for _ in 0..fs_size {
                blocks.push(-1);
            }
        }
    }

    for f in file_infos.iter_mut().rev() {
        let x = blocks[f.start];
        for s in free_infos.iter_mut() {
            if s.len >= f.len && s.start < f.start {
                for _ in 0..f.len {
                    blocks[s.start] = x;
                    blocks[f.start] = -1;
                    s.start += 1;
                    s.len -= 1;
                    f.start += 1;
                }
                break;
            }
        }
    }

    let check_sum = blocks
        .iter()
        .enumerate()
        .filter_map(|(i, &e)| if e > 0 { Some(i * e as usize) } else { None })
        .sum();

    Ok(check_sum)
}

fn parse_input(input: &str) -> Vec<i8> {
    input
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as i8)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"2333133121414131402"#;
        let result = part1(&input).unwrap();
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part2() {
        let input = r#"2333133121414131402"#;
        let result = part2(&input).unwrap();
        assert_eq!(result, 2858);
    }
}
