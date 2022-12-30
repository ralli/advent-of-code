use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{line_ending, space1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::HashMap;

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
    pub fn is_fullfilled(&self, num_ore: i32, num_clay: i32, num_obsidian: i32) -> bool {
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

#[derive(Debug, Copy, Clone, Default)]
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
    pub fn produce(&self, definition: &RobotDefinition) -> State {
        let mut state = *self;

        state.step += 1;
        state.num_ore += state.num_ore_bots;
        state.num_clay += state.num_clay_bots;
        state.num_obsidian += state.num_obsidian_bots;
        state.num_geode += state.num_geode_bots;

        match definition.robot_type {
            RobotType::Ore => {
                state.num_ore_bots += 1;
            }
            RobotType::Clay => {
                state.num_clay_bots += 1;
            }
            RobotType::Obsidian => {
                state.num_obsidian_bots += 1;
            }
            RobotType::Geode => {
                state.num_geode_bots += 1;
            }
        };

        state.num_ore -= definition.costs_ore;
        state.num_obsidian -= definition.costs_obsidian;
        state.num_clay -= definition.costs_clay;

        state
    }

    pub fn can_produce(&self, definition: &RobotDefinition) -> bool {
        definition.is_fullfilled(self.num_ore, self.num_clay, self.num_obsidian)
    }
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
