use std::fmt::{self, Write};
use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::{
    game::{Player, VarMap},
};

use super::{
    ValType,
    expression::Var,
};

fn err(msg: &str) -> Box<dyn Error> {
    std::io::Error::other(msg.to_string()).into()
}

impl fmt::Display for ValType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValType::Num(n) => write!(f, "{n}"),
            ValType::String(s) => f.write_str(s),
            ValType::None => Ok(()),
        }
    }
}

#[derive(Debug)]
enum Segment {
    Lit(String),
    Var(Var),
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct Template {
    segments: Vec<Segment>,
}

impl TryFrom<String> for Template {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Template::parse(&s).map_err(|e| e.to_string())
    }
}

impl Template {
    pub fn parse(src: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut segments = Vec::new();
        let mut lit = String::new();
        let mut chars = src.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '{' if chars.peek() == Some(&'{') => { chars.next(); lit.push('{'); }
                '}' if chars.peek() == Some(&'}') => { chars.next(); lit.push('}'); }

                '{' => {
                    let mut name = String::new();
                    loop {
                        match chars.next() {
                            Some('}') => break,
                            Some('{') | None => return Err(err("unclosed '{'")),
                            Some(ch) => name.push(ch),
                        }
                    }
                    if !lit.is_empty() {
                        segments.push(Segment::Lit(std::mem::take(&mut lit)));
                    }
                    segments.push(Segment::Var(Var::from_str(name.trim())?));
                }

                '}' => return Err(err("unmatched '}'")),
                c => lit.push(c),
            }
        }

        if !lit.is_empty() {
            segments.push(Segment::Lit(lit));
        }
        Ok(Template { segments })
    }

    pub fn render(&self, player: &Player, locals: &VarMap, globals: &VarMap) -> String {
        let mut out = String::new();
        for seg in &self.segments {
            match seg {
                Segment::Lit(s) => out.push_str(s),
                Segment::Var(v) => {
                    let _ = write!(out, "{}", v.get(player, locals, globals));
                }
            }
        }
        out
    }
}