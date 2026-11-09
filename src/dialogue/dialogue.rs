use core::iter::Iterator;
use std::fs;
use serde::Deserialize;
use std::collections::HashMap;

use crate::{
    game::{Player, VarMap}
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
    var: VarType,
    op: Op,
    value: i16,
}

impl Check {
    fn get_var(&self, player: &Player, l_scope: &mut VarMap, g_scope: &mut VarMap) -> i16 {
        match self.var {
            VarType::Global(ref var_key) => {
                match g_scope.get(var_key.as_str()) {
                    None => {
                        g_scope.insert(var_key.clone(), 0).unwrap()
                    }
                    Some(val) => *val
                }
            },

            VarType::Local(ref var_key) => {
                match l_scope.get(var_key.as_str()) {
                    None => {
                        l_scope.insert(var_key.clone(), 0).unwrap()
                    }
                    Some(val) => *val
                }
            },
            
            VarType::Stat(ref var_key) => {
                player.stat_str(var_key.as_str()).unwrap()
            },
        }
    }

    pub fn check(&self, player: &Player, l_scope: &mut VarMap, g_scope: &mut VarMap) -> bool {
        let stat = self.get_var(player, l_scope, g_scope);
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
enum VarType {
    Stat(String),
    Local(String),
    Global(String),
}

#[derive(Debug)]
pub struct Condition(Vec<Check>);

impl Condition {
    fn parse_stat(stat: &str) -> Result<VarType, Box<dyn std::error::Error>> {
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

        if !VALID_STATS.contains(&stat) {
            Err(std::io::Error::other(format!("invalid stat given: {}", stat)))?;
        }
        Ok(VarType::Stat(stat.to_string()))
    }

    fn parse_variable(var: &str) -> Result<VarType, Box<dyn std::error::Error>> {
        if var.starts_with("local.") {
            return Ok(VarType::Local(var.to_string()));
        }
        if var.starts_with("global.") {
            return Ok(VarType::Global(var.to_string()));
        }
        if var.contains(".") {
            Err(std::io::Error::other(format!("invalid variable prefix given: {}", var)))?;
        }
        Self::parse_stat(var)
    }

    fn parse_condition(condition: &str) -> Result<Check, Box<dyn std::error::Error>> {
        const OPERATORS: [(&str, Op); 6] = [
            (">=", Op::Ge),
            ("<=", Op::Le),
            ("==", Op::Eq),
            ("!=", Op::Ne),
            (">", Op::Gt),
            ("<", Op::Lt),
        ];

        let (var, op, value) = OPERATORS.iter()
            .find_map(|(operator, op)| {
                let (var, value) = condition.split_once(operator)?;
                Some((var, op, value))
            }).ok_or(std::io::Error::other("failed on parsing"))?;

            let var = Self::parse_variable(var.trim())?;

            Ok(Check {
                var,
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

    pub fn check(&self, player: &Player, l_scope: &mut VarMap, g_scope: &mut VarMap) -> bool {
        !self.0.iter()
        .map(|check| check.check(player, l_scope, g_scope))
        .collect::<Vec<bool>>()
        .contains(&false)
    }
}
