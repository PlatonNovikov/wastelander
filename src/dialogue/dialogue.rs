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

    #[serde(default, rename = "set")]
    effects_src: Vec<String>,
    #[serde(skip)]
    pub effects: Vec<Effect>,
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
    var_1: VarType,
    op: Op,
    var_2: VarType,
}

impl Check {
    fn get_var(var: &VarType, player: &Player, locals: &mut VarMap, globals: &mut VarMap) -> i16 {
        match var {
            VarType::Global(k) => globals.get(k).copied().unwrap_or(0),
            VarType::Local(k) => locals.get(k).copied().unwrap_or(0),
            VarType::Stat(k) => player.stat_str(k.as_str()).unwrap(), // already validated
            VarType::Num(n) => *n
        }
    }

    pub fn check(&self, player: &Player, locals: &mut VarMap, globals: &mut VarMap) -> bool {
        let var_1 = Self::get_var(&self.var_1, player, locals, globals);
        let var_2 = Self::get_var(&self.var_2, player, locals, globals);

        match self.op {
            Op::Gt => var_1 > var_2,
            Op::Ge => var_1 >= var_2,
            Op::Lt => var_1 < var_2,
            Op::Le => var_1 <= var_2,
            Op::Eq => var_1 == var_2,
            Op::Ne => var_1 != var_2,
        }
    }
}

#[derive(Debug)]
enum VarType {
    Stat(String),
    Local(String),
    Global(String),
    Num(i16),
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
        if let Ok(n) = var.parse::<i16>() {
            return Ok(VarType::Num(n))
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

        let (var_1, op, var_2) = OPERATORS.iter()
            .find_map(|(operator, op)| {
                let (var_1, var_2) = condition.split_once(operator)?;
                Some((var_1, op, var_2))
            }).ok_or(std::io::Error::other("failed on parsing"))?;

            let var_1 = Self::parse_variable(var_1.trim())?;
            let var_2 = Self::parse_variable(var_2.trim())?;

            Ok(Check {
                var_1,
                op: op.clone(),
                var_2
            })
    }

    pub fn parse(conditions_src: &str) -> Result<Condition, Box<dyn std::error::Error>> {
        let parsed: Vec<Check> = conditions_src.split("&&")
        .map(Self::parse_condition)
        .collect::<Result<_, _>>()?;
        Ok(Condition(parsed))
    }

    pub fn check(&self, player: &Player, locals: &mut VarMap, globals: &mut VarMap) -> bool {
        !self.0.iter()
        .map(|check| check.check(player, locals, globals))
        .collect::<Vec<bool>>()
        .contains(&false)
    }
}

// Assignment operators
#[derive(Debug)]
enum Ao { Assign, Add, Sub, Mul, Div }

#[derive(Debug)]
pub struct Effect {
    var_1: VarType,
    operation: Ao,
    var_2: VarType,
}