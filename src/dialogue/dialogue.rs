use core::iter::Iterator;
use std::fs;
use serde::Deserialize;
use std::collections::HashMap;

use crate::{
    game::Player
};

#[derive(Debug, Deserialize)]
pub struct Dialogue {
    #[serde(flatten)]
    pub nodes: HashMap<String, Node>,
}

#[derive(Debug, Deserialize)]
pub struct Node {
    pub text: String,

    #[serde(default)]
    pub answers: Vec<Answer>,

    #[serde(default)]
    pub end: bool,
}

#[derive(Debug, Deserialize)]
pub struct Answer {
    pub text: String,

    pub next: Option<String>,

    #[serde(default)]
    pub end: bool,

    #[serde(default, rename = "condition")]
    condition_src: Option<String>,

    #[serde(skip)]
    pub condition: Option<Condition>,
}

pub fn load_dialogue(dialogue_file: &str) -> Result<Dialogue, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(dialogue_file)?;
    let mut dialogue: Dialogue = toml::from_str(&content)?;

    for (node_name, node) in dialogue.nodes.iter_mut() {
        for answer in node.answers.iter_mut() {
            if let Some(src) = &answer.condition_src {
                let cond = Condition::parse(src)
                    .map_err(|e| format!("Node '{node_name}': {e}"))?;
                answer.condition = Some(cond);
            }
        }
    }

    Ok(dialogue)
}

#[derive(Debug, Clone)]
enum Op { Gt, Ge, Lt, Le, Eq, Ne }

#[derive(Debug)]
struct Check {
    stat: String,
    op: Op,
    value: i16,
}

impl Check {
    pub fn check(&self, player: &Player) -> bool {
        let stat = player.stat_str(self.stat.as_str()).unwrap();
        match self.op {
            Op::Gt => stat > self.value,
            Op::Ge => stat >= self.value,
            Op::Lt => stat < self.value,
            Op::Le => stat <= self.value,
            Op::Eq => stat == self.value,
            Op::Ne => stat != self.value,
        }
    }
}

#[derive(Debug)]
pub struct Condition(Vec<Check>);

impl Condition {
    fn parse_condition(condition: &str) -> Result<Check, Box<dyn std::error::Error>> {
        const OPERATORS: [(&str, Op); 6] = [
            (">=", Op::Ge),
            ("<=", Op::Le),
            ("==", Op::Eq),
            ("!=", Op::Ne),
            (">", Op::Gt),
            ("<", Op::Lt),
        ];

        const VALID_STATS: [&str; 20] = [
            "strength",
            "perception",
            "endurance",
            "charisma",
            "intelligence",
            "agility",
            "luck",
            "barter",
	        "e_weapons",
	        "explosives",
	        "guns",
	        "lockpick",
	        "medicine",
	        "m_weapons",
	        "repair",
	        "science",
	        "sneak",
	        "speech",
	        "survival",
	        "unarmed",
        ];

        let (stat, op, value) = OPERATORS.iter()
            .find_map(|(operator, op)| {
                let (stat, value) = condition.split_once(operator)?;
                Some((stat, op, value))
            }).ok_or(std::io::Error::other("failed on parsing"))?;

            let stat = stat.trim().to_string().to_lowercase();

            if !VALID_STATS.contains(&stat.as_str()) {
                Err(std::io::Error::other(format!("invalid stat given: {}", stat)))?;
            }

            Ok(Check {
                stat,
                op: op.clone(),
                value: value.trim().parse()?
            })
    }
    pub fn parse(conditions_src: &str) -> Result<Condition, Box<dyn std::error::Error>> {
        let parsed: Vec<Check> = conditions_src.split("&&")
        .map(Self::parse_condition)
        .collect::<Result<_, _>>()?;
        Ok(Condition(parsed))
    }

    pub fn check(&self, player: &Player) -> bool {
        !self.0.iter()
        .map(|check| check.check(player))
        .collect::<Vec<bool>>()
        .contains(&false)
    }
}
