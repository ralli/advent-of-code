use day_19::{blueprints, BluePrint, State};
use std::collections::VecDeque;

fn main() -> anyhow::Result<()> {
    let input = read_file("./day-19/test.txt")?;

    let result = part1(&input);
    println!("{}", result);

    // let result = part2(&input);
    // println!("{}", result);

    Ok(())
}

fn part1(input: &str) -> i32 {
    let (_, blueprints) = blueprints(input).unwrap();
    let geodes: Vec<i32> = blueprints.iter().map(|bp| bp.id * max_geodes(bp)).collect();
    geodes.iter().sum()
}

fn max_geodes(&blueprint: &BluePrint) -> i32 {
    let initial_state = State {
        num_ore_bots: 1,
        ..Default::default()
    };
    let mut q = VecDeque::from([initial_state]);
    let max_steps = 24;
    let mut max_geode = 0;

    let robot_definitions = [
        &blueprint.ore,
        &blueprint.clay,
        &blueprint.obsidian,
        &blueprint.geode,
    ];

    let max_ore_bots = robot_definitions.iter().map(|d| d.costs_ore).max().unwrap();
    let max_clay_bots = robot_definitions
        .iter()
        .map(|d| d.costs_clay)
        .max()
        .unwrap();
    let max_obsidian_bots = robot_definitions
        .iter()
        .map(|d| d.costs_obsidian)
        .max()
        .unwrap();

    while let Some(current) = q.pop_front() {
        assert!(q.len() < 100_000_000);
        println!("{:?}", current);
        if current.step == max_steps && current.num_geode > max_geode {
            max_geode = current.num_geode;
        }

        if current.step > max_steps {
            break;
        }

        if current.can_produce(&blueprint.geode) {
            q.push_back(current.produce(&blueprint.geode))
        }

        if current.num_ore_bots < max_ore_bots && current.can_produce(&blueprint.ore) {
            q.push_back(current.produce(&blueprint.ore))
        }

        if current.num_clay_bots < max_clay_bots && current.can_produce(&blueprint.clay) {
            q.push_back(current.produce(&blueprint.clay))
        }

        if current.num_obsidian_bots < max_obsidian_bots && current.can_produce(&blueprint.obsidian)
        {
            q.push_back(current.produce(&blueprint.obsidian))
        }

        let mut next = current;
        next.num_ore += next.num_ore_bots;
        next.num_clay += next.num_clay_bots;
        next.num_obsidian += next.num_obsidian_bots;
        next.num_geode += next.num_geode_bots;
        next.step += 1;

        q.push_back(next);
    }

    max_geode
}

fn part2(input: &str) -> i32 {
    todo!()
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
}
