use std::fs;
use serde::Deserialize;
use std::collections::HashMap;

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

    // for (node_name, node) in dialogue.nodes.iter_mut() {
        // for answer in node.answers.iter_mut() {
            // if let Some(src) = &answer.condition_src {
                // let cond = Condition::parse(src)
                    // .map_err(|e| format!("Node '{node_name}': {e}"))?;
                // answer.condition = Some(cond);
            // }
        // }
    // }

    Ok(dialogue)
}

#[derive(Debug)]
enum Op { Gt, Ge, Lt, Le, Eq, Ne }

#[derive(Debug)]
struct Check {
    stat: String,
    op: Op,
    value: i32,
}

#[derive(Debug)]
pub struct Condition(Vec<Check>);

impl Condition {
    
}