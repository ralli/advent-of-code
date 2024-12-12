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

    if blocks.is_empty() {
        return Ok(0);
    }

    let mut l = 0;
    let size = blocks.len();
    let mut r = size - 1;

    while l < r {
        while blocks[l] >= 0 && l < size - 1 {
            l += 1;
        }

        while blocks[r] < 0 && r > 0 {
            r -= 1;
        }

        if l < r {
            blocks[l] = blocks[r];
            blocks[r] = -1;
        }
    }

    let check_sum: usize = blocks
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| if b >= 0 { Some(i * b as usize) } else { None })
        .sum();

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

    for file_info in file_infos.iter_mut().rev() {
        if let Some(free_info) = free_infos
            .iter_mut()
            .take_while(|free_info| free_info.start < file_info.start)
            .find(|free_info| free_info.len >= file_info.len)
        {
            let x = blocks[file_info.start];
            for _ in 0..file_info.len {
                blocks[free_info.start] = x;
                blocks[file_info.start] = -1;
                free_info.start += 1;
                free_info.len -= 1;
                file_info.start += 1;
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
        .filter(|c| c.is_ascii_digit())
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
