use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;
use std::str::FromStr;
use std::{fmt, fs};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Choice {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissors),
            _ => Err(anyhow::Error::msg(format!("cannot parse {}", s))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Intention {
    Loose,
    Draw,
    Win,
}

impl FromStr for Intention {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Intention::Loose),
            "Y" => Ok(Intention::Draw),
            "Z" => Ok(Intention::Win),
            _ => Err(anyhow::Error::msg(format!("cannot parse {}", s))),
        }
    }
}

impl Choice {
    fn choice_with_intended_result(&self, intention: &Intention) -> Self {
        match intention {
            Intention::Loose => match self {
                Choice::Rock => Choice::Scissors,
                Choice::Paper => Choice::Rock,
                Choice::Scissors => Choice::Paper,
            },
            Intention::Draw => *self,
            Intention::Win => match self {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissors,
                Choice::Scissors => Choice::Rock,
            },
        }
    }

    fn value(&self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn wins_against(&self, opponent: &Choice) -> bool {
        match self {
            Choice::Rock => *opponent == Choice::Scissors,
            Choice::Paper => *opponent == Choice::Rock,
            Choice::Scissors => *opponent == Choice::Paper,
        }
    }

    fn game_score(&self, opponent: &Choice) -> i32 {
        if *self == *opponent {
            3
        } else if self.wins_against(opponent) {
            6
        } else {
            0
        }
    }

    fn total_score(&self, opponent: &Choice) -> i32 {
        self.value() + self.game_score(opponent)
    }
}

fn part1(input: &str) -> anyhow::Result<i32> {
    let lines = input.lines();
    let mut result = 0;
    for line in lines {
        let parts: Vec<_> = line.split(' ').collect();
        let opponent = Choice::from_str(parts[0])?;
        let me = Choice::from_str(parts[1])?;
        result += me.total_score(&opponent);
    }
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<i32> {
    let lines = input.lines();
    let mut result = 0;
    for line in lines {
        let parts: Vec<_> = line.split(' ').collect();
        let opponent = parts[0].parse::<Choice>()?;
        let intention = parts[1].parse::<Intention>()?;
        let me = opponent.choice_with_intended_result(&intention);
        result += me.total_score(&opponent);
    }
    Ok(result)
}

fn main() -> anyhow::Result<()> {
    let filename = "./day-2/input.txt";
    let content = read_file(filename)?;
    let sum = part1(&content)?;
    println!("{}", sum);

    let sum = part2(&content)?;
    println!("{}", sum);

    Ok(())
}

fn read_file(filename: &str) -> anyhow::Result<String> {
    fs::read_to_string(filename)
        .map_err(|err| anyhow::Error::new(err).context(format!("cannot read {}", filename)))
}
