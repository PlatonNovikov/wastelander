use core::iter::Iterator;
use std::fs;
use std::collections::HashMap;

use serde::Deserialize;

use super::expression::{Expression};

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
    pub condition: Option<Expression>,

    #[serde(default, rename = "set")]
    effects_src: Vec<String>,
    #[serde(skip)]
    pub effects: Vec<Expression>,
}

pub fn load_dialogue(dialogue_file: &str) -> Result<Dialogue, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(dialogue_file)?;
    let mut dialogue: Dialogue = toml::from_str(&content)?;

    for (node_name, node) in dialogue.nodes.iter_mut() {
        for answer in node.answers.iter_mut() {
            if let Some(src) = &answer.condition_src {
                let cond = Expression::parse(src)
                    .map_err(|e| format!("Node '{node_name}': {e}"))?;
                answer.condition = Some(cond);
            }
            
            for effect in answer.effects_src.iter() {
                answer.effects.push(Expression::parse(effect)
                    .map_err(|e| format!("Node '{node_name}': {e}"))?);
            }
        }
    }

    Ok(dialogue)
}