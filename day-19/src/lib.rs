use std::collections::{HashMap, HashSet, VecDeque};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, space1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Material {
    Ore,
    Clay,
    Obsidian,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Copy, Clone)]
pub struct RobotDefinition {
    pub robot_type: RobotType,
    pub costs_ore: i32,
    pub costs_clay: i32,
    pub costs_obsidian: i32,
}

impl RobotDefinition {
    pub fn is_fulfilled(&self, num_ore: i32, num_clay: i32, num_obsidian: i32) -> bool {
        num_ore >= self.costs_ore
            && num_clay >= self.costs_clay
            && num_obsidian >= self.costs_obsidian
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BluePrint {
    pub id: i32,
    pub ore: RobotDefinition,
    pub clay: RobotDefinition,
    pub obsidian: RobotDefinition,
    pub geode: RobotDefinition,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct State {
    pub step: i32,

    pub num_ore: i32,
    pub num_clay: i32,
    pub num_obsidian: i32,
    pub num_geode: i32,

    pub num_ore_bots: i32,
    pub num_clay_bots: i32,
    pub num_obsidian_bots: i32,
    pub num_geode_bots: i32,
}

impl State {
    pub fn next_state(&self, definition: &RobotDefinition, max_steps: i32) -> Option<State> {
        assert!(self.num_geode >= 0);
        assert!(self.num_ore >= 0);
        assert!(self.num_clay >= 0);
        assert!(self.num_obsidian >= 0);
        if self.step == max_steps {
            return None;
        }
        if self.can_produce(definition) {
            return Some(self.produce(definition));
        }

        if definition.costs_ore > 0 && self.num_ore_bots == 0 {
            return None;
        }

        let ore_steps = get_num_steps(self.num_ore_bots, self.num_ore, definition.costs_ore);

        if definition.costs_clay > 0 && self.num_clay_bots == 0 {
            return None;
        }

        let clay_steps = get_num_steps(self.num_clay_bots, self.num_clay, definition.costs_clay);

        if definition.costs_obsidian > 0 && self.num_obsidian_bots == 0 {
            return None;
        }

        let obsidian_steps = get_num_steps(
            self.num_obsidian_bots,
            self.num_obsidian,
            definition.costs_obsidian,
        );

        let num_steps = ore_steps.max(clay_steps).max(obsidian_steps) + 1;

        if self.step + num_steps > max_steps {
            //return None;
            let mut state = *self;
            state.advance_num_steps(max_steps - self.step);
            return Some(state);
        }

        let mut state = *self;
        state.advance_num_steps(num_steps);
        state.increase_bots(definition);

        Some(state)
    }

    fn advance_num_steps(&mut self, num_steps: i32) {
        self.step += num_steps;
        self.num_ore += self.num_ore_bots * num_steps;
        self.num_clay += self.num_clay_bots * num_steps;
        self.num_obsidian += self.num_obsidian_bots * num_steps;
        self.num_geode += self.num_geode_bots * num_steps;
    }

    fn increase_bots(&mut self, definition: &RobotDefinition) {
        match definition.robot_type {
            RobotType::Ore => {
                self.num_ore_bots += 1;
            }
            RobotType::Clay => {
                self.num_clay_bots += 1;
            }
            RobotType::Obsidian => {
                self.num_obsidian_bots += 1;
            }
            RobotType::Geode => {
                self.num_geode_bots += 1;
            }
        };
        self.num_ore -= definition.costs_ore;
        self.num_obsidian -= definition.costs_obsidian;
        self.num_clay -= definition.costs_clay;
    }

    pub fn produce(&self, definition: &RobotDefinition) -> State {
        let mut state = *self;

        state.advance_num_steps(1);
        state.increase_bots(definition);

        state
    }

    pub fn can_produce(&self, definition: &RobotDefinition) -> bool {
        definition.is_fulfilled(self.num_ore, self.num_clay, self.num_obsidian)
    }
}

pub fn max_geodes(&blueprint: &BluePrint, max_steps: i32) -> i32 {
    let initial_state = State {
        num_ore_bots: 1,
        ..Default::default()
    };
    let mut q = VecDeque::from([initial_state]);
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

    let mut visited = HashSet::from([initial_state]);

    while let Some(current) = q.pop_front() {
        // assert!(q.len() < 100_000_000);
        // println!("{:?}", current);

        if current.num_geode > max_geode {
            max_geode = current.num_geode;
        }

        if current.num_ore_bots < max_ore_bots {
            if let Some(ore_state) = current.next_state(&blueprint.ore, max_steps) {
                if visited.insert(ore_state) {
                    q.push_back(ore_state);
                }
            }
        }

        if current.num_clay_bots < max_clay_bots {
            if let Some(clay_state) = current.next_state(&blueprint.clay, max_steps) {
                if visited.insert(clay_state) {
                    q.push_back(clay_state);
                }
            }
        }

        if current.num_obsidian_bots < max_obsidian_bots {
            if let Some(obsidian_state) = current.next_state(&blueprint.obsidian, max_steps) {
                if visited.insert(obsidian_state) {
                    q.push_back(obsidian_state);
                }
            }
        }

        if let Some(geode_state) = current.next_state(&blueprint.geode, max_steps) {
            if visited.insert(geode_state) {
                q.push_back(geode_state);
            }
        }
    }

    max_geode
}

fn get_num_steps(num_bots: i32, current: i32, goal: i32) -> i32 {
    if num_bots == 0 {
        return 0;
    }
    let mut i = 0;
    while current + i * num_bots < goal {
        i += 1;
    }
    i
}

pub fn blueprints(input: &str) -> IResult<&str, Vec<BluePrint>> {
    separated_list1(line_ending, blueprint)(input)
}

fn blueprint(input: &str) -> IResult<&str, BluePrint> {
    let (input, _) = tag("Blueprint ")(input)?;
    let (input, id) = complete::i32(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, definitions) = robot_definitions(input)?;
    let bp = BluePrint {
        id,
        ore: definitions[&RobotType::Ore],
        clay: definitions[&RobotType::Clay],
        obsidian: definitions[&RobotType::Obsidian],
        geode: definitions[&RobotType::Geode],
    };

    Ok((input, bp))
}

fn robot_definitions(input: &str) -> IResult<&str, HashMap<RobotType, RobotDefinition>> {
    let (input, definitions) = separated_list1(space1, robot_definition)(input)?;
    let definition_map: HashMap<RobotType, RobotDefinition> = definitions
        .into_iter()
        .map(|definition| (definition.robot_type, definition))
        .collect();

    Ok((input, definition_map))
}

fn robot_definition(input: &str) -> IResult<&str, RobotDefinition> {
    let (input, _) = tag("Each ")(input)?;
    let (input, robot_type) = alt((
        map(tag("ore"), |_| RobotType::Ore),
        map(tag("clay"), |_| RobotType::Clay),
        map(tag("obsidian"), |_| RobotType::Obsidian),
        map(tag("geode"), |_| RobotType::Geode),
    ))(input)?;
    let (input, _) = tag(" robot costs ")(input)?;
    let (input, costs) = costs(input)?;
    let (input, _) = tag(".")(input)?;

    let definition = RobotDefinition {
        robot_type,
        costs_ore: costs.get(&Material::Ore).copied().unwrap_or(0),
        costs_clay: costs.get(&Material::Clay).copied().unwrap_or(0),
        costs_obsidian: costs.get(&Material::Obsidian).copied().unwrap_or(0),
    };
    Ok((input, definition))
}

fn costs(input: &str) -> IResult<&str, HashMap<Material, i32>> {
    let (input, cost_list) = separated_list1(tag(" and "), cost)(input)?;
    let cost_map: HashMap<Material, i32> = cost_list.into_iter().collect();

    Ok((input, cost_map))
}

fn cost(input: &str) -> IResult<&str, (Material, i32)> {
    let (input, cost) = complete::i32(input)?;
    let (input, _) = space1(input)?;
    let (input, name) = alt((
        map(tag("ore"), |_| Material::Ore),
        map(tag("clay"), |_| Material::Clay),
        map(tag("obsidian"), |_| Material::Obsidian),
    ))(input)?;

    Ok((input, (name, cost)))
}
