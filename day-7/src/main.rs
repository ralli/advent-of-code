use anyhow::anyhow;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space1,
    character::complete::{line_ending, not_line_ending},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};
use std::iter::{Iterator, Peekable};
use std::rc::Rc;

fn main() -> anyhow::Result<()> {
    let content = read_file("./day-7/input.txt")?;

    let result = part1(&content)?;
    println!("{}", result);

    let result = part2(&content)?;
    println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let (_, lines) = parse_file(input).unwrap();
    let mut processor = Processor::new(&lines);
    let tree = processor.build_cd()?;
    let size = tree.size_of_dirs_less_than(100_000);
    Ok(size)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let (_, lines) = parse_file(input).unwrap();
    let mut processor = Processor::new(&lines);
    let tree = processor.build_cd()?;
    let file_system_size = 70_000_000;
    let min_free_size = 30_000_000;
    let space_used = tree.size();
    let free_size = file_system_size - space_used;
    let size_to_free = if free_size < min_free_size {
        min_free_size - free_size
    } else {
        0
    };
    let smallest_size = tree.smallest_dir_size_larger_than(size_to_free);
    match smallest_size {
        Some(size) => Ok(size),
        None => Err(anyhow!("no matching directory found")),
    }
}

#[derive(Debug, Clone)]
enum FileEntry {
    Dir {
        name: String,
        size: usize,
        entries: Vec<Rc<FileEntry>>,
    },
    File {
        name: String,
        size: usize,
    },
}

impl FileEntry {
    fn size(&self) -> usize {
        match self {
            FileEntry::Dir {
                name: _,
                size,
                entries: _,
            } => *size,
            FileEntry::File { name: _, size } => *size,
        }
    }

    fn size_of_dirs_less_than(&self, max_size: usize) -> usize {
        match self {
            FileEntry::Dir {
                name: _,
                size,
                entries,
            } => {
                let child_sum = entries
                    .iter()
                    .map(|e| e.size_of_dirs_less_than(max_size))
                    .sum();
                if *size < max_size {
                    size + child_sum
                } else {
                    child_sum
                }
            }
            FileEntry::File { name: _, size: _ } => 0,
        }
    }

    fn smallest_dir_size_larger_than(&self, min_size: usize) -> Option<usize> {
        match self {
            FileEntry::Dir {
                name: _,
                size,
                entries,
            } => {
                let min_child_size = entries
                    .iter()
                    .filter_map(|e| e.smallest_dir_size_larger_than(min_size))
                    .min();
                if *size >= min_size {
                    match min_child_size {
                        Some(child_size) => Some(child_size.min(*size)),
                        _ => Some(*size),
                    }
                } else {
                    min_child_size
                }
            }
            FileEntry::File { name: _, size: _ } => None,
        }
    }
}

struct Processor<'a> {
    it: Peekable<std::slice::Iter<'a, Line>>,
}

impl<'a> Processor<'a> {
    fn new(input: &'a [Line]) -> Self {
        Processor {
            it: input.iter().peekable(),
        }
    }

    fn build_cd(&mut self) -> anyhow::Result<Rc<FileEntry>> {
        if let Some(Line::Cd(dir)) = self.it.next() {
            self.process_dir(dir.as_str())
        } else {
            Err(anyhow!("no root provided"))
        }
    }

    fn process_dir(&mut self, dir_name: &str) -> anyhow::Result<Rc<FileEntry>> {
        let mut entries: Vec<Rc<FileEntry>> = Vec::new();
        while let Some(item) = self.it.next() {
            match item {
                Line::Cd(dir) => {
                    if dir == ".." {
                        return Ok(Rc::new(FileEntry::Dir {
                            name: dir_name.to_owned(),
                            size: entries.iter().map(|e| e.size()).sum(),
                            entries,
                        }));
                    } else {
                        entries.push(self.process_dir(dir)?)
                    }
                }
                Line::Ls => {}
                Line::Directory(_) => {}
                Line::File(size, name) => entries.push(Rc::new(FileEntry::File {
                    name: name.to_owned(),
                    size: *size,
                })),
            }
        }

        Ok(Rc::new(FileEntry::Dir {
            name: dir_name.to_owned(),
            size: entries.iter().map(|e| e.size()).sum(),
            entries,
        }))
    }
}

#[derive(Debug, Clone)]
enum Line {
    Cd(String),
    Ls,
    Directory(String),
    File(usize, String),
}

fn parse_file(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, lines) = separated_list1(line_ending, parse_line)(input)?;
    Ok((input, lines))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    alt((parse_command, parse_directory, parse_file_line))(input)
}

fn parse_command(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("$ ")(input)?;
    let (input, command) = alt((parse_cd, parse_ls))(input)?;

    Ok((input, command))
}

fn parse_cd(input: &str) -> IResult<&str, Line> {
    let (input, dir) = preceded(tag("cd "), not_line_ending)(input)?;
    Ok((input, Line::Cd(dir.to_owned())))
}

fn parse_ls(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("ls")(input)?;

    Ok((input, Line::Ls))
}

fn parse_directory(input: &str) -> IResult<&str, Line> {
    let (input, name) = preceded(tag("dir "), not_line_ending)(input)?;

    Ok((input, Line::Directory(name.to_owned())))
}

fn parse_file_line(input: &str) -> IResult<&str, Line> {
    use nom::character::complete::u64 as u64_parser;

    let (input, size) = u64_parser(input)?;
    let (input, _) = space1(input)?;
    let (input, name) = not_line_ending(input)?;

    Ok((input, Line::File(size as usize, name.to_owned())))
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
        let (_, lines) = parse_file(&input).unwrap();
        let mut processor = Processor::new(&lines);
        let tree = processor.build_cd().unwrap();
        let size = tree.size_of_dirs_less_than(100_000);
        assert_eq!(size, 95437);
    }

    #[test]
    fn part2_works_2() {
        let input = test_input();
        let result = part2(&input).unwrap();
        let expected = 24933642;
        assert_eq!(result, expected);
    }

    fn test_input() -> String {
        r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#
            .to_owned()
    }
}
