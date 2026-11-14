use core::iter::Iterator;
use std::error::Error;
use std::cmp::Ordering;

use crate::{
    game::{Player, VarMap}
};

fn err(msg: &str) -> Box<dyn Error> {
    std::io::Error::other(msg.to_string()).into()
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Or, And,
    Eq, Ne, Gt, Ge, Lt, Le,
    Add, Sub, Mul, Div,
    Assign, AddAssign, SubAssign, MulAssign, DivAssign,
}

const OPS: [(&str, Op); 17] = [
            ("||", Op::Or), 
            ("&&", Op::And),
            ("==", Op::Eq),
            ("!=", Op::Ne), 
            (">=", Op::Ge), 
            ("<=", Op::Le),
            ("+=", Op::AddAssign), 
            ("-=", Op::SubAssign),
            ("*=", Op::MulAssign),
            ("/=", Op::DivAssign),
            (">", Op::Gt), 
            ("<", Op::Lt), 
            ("+", Op::Add), 
            ("-", Op::Sub), 
            ("*", Op::Mul), 
            ("/", Op::Div),
            ("=", Op::Assign), 
        ];

#[derive(Debug)]
enum Token {
    Var(String),
    Op(Op),
    LParen,
    RParen,
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn parse_expression(&mut self, min_prec: u8) -> Result<Expression, Box<dyn std::error::Error>> {
        let mut lhs = self.parse_primary()?;

        loop {
            let op = match self.peek() {
                Some(Token::Op(op)) => *op,
                _ => break,
            };

            let (left_bp, right_bp) = match op {
                // right associative operators
                Op::Assign | Op::AddAssign | Op::SubAssign
                | Op::MulAssign | Op::DivAssign => (1, 1),

                // left associative operators
                Op::Or => (2, 3),
                Op::And => (3, 4),
                Op::Eq | Op::Ne => (4, 5),
                Op::Gt | Op::Ge | Op::Lt | Op::Le => (5, 6),
                Op::Add | Op::Sub => (6, 7),
                Op::Mul | Op::Div => (7, 8),
            };

            if left_bp < min_prec {
                break;
            }

            self.pos += 1;

            let rhs = self.parse_expression(right_bp)?;

            lhs = Expression::Binary(
                Box::new(lhs),
                op,
                Box::new(rhs),
            );
        }

        Ok(lhs)
    }

    fn parse_primary(&mut self) -> Result<Expression, Box<dyn std::error::Error>> {
        match self.tokens.get(self.pos) {
            Some(Token::Var(name)) => {
                let var = Var::from_str(name)?;
                self.pos += 1;

                Ok(Expression::Var(var))
            }

            Some(Token::LParen) => {
                self.pos += 1;

                let expr = self.parse_expression(1)?;

                match self.peek() {
                    Some(Token::RParen) => self.pos += 1,
                    _ => return Err(err("expected closing parenthesis")),
                }

                Ok(expr)
            }

            Some(Token::Op(Op::Sub)) => {
                self.pos += 1;

                let expr = self.parse_expression(8)?;

                Ok(Expression::Neg(Box::new(expr)))
            }

            _ => Err(err("expected expression")),
        }
    }
}

fn tokenize(src: &str) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let bytes = src.as_bytes();

    while i < bytes.len() {
        let c = bytes[i] as char;
        if c.is_whitespace() {
            i += 1;
        } else if c == '(' {
            tokens.push(Token::LParen);
            i += 1;
        } else if c == ')' {
            tokens.push(Token::RParen);
            i += 1;
        } else if c.is_ascii_alphanumeric() || c == '_' || c == '.' {
            let start = i;
            while i < bytes.len() {
                let ch = bytes[i] as char;
                if ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' { i += 1; } else { break; }
            }
            tokens.push(Token::Var(src[start..i].to_string()));
        } else if c == '\"' {
            let start = i;
            while i < bytes.len() {
                let ch = bytes[i] as char;
                if ch == '\"' { break; }
            }
            tokens.push(Token::Var(src[start..i].to_string()));
        } else if let Some((s, op)) = OPS.iter().find(|(s, _)| src[i..].starts_with(s)) {
            tokens.push(Token::Op(*op));
            i += s.len();
        } else {
            return Err(err(format!("unexpected character '{}' at {}", c, i).as_str()));
        }
    }
    Ok(tokens)
}

#[derive(Debug, PartialEq)]
pub enum Var {
    Stat(String),
    Local(String),
    Global(String),
    Value(ValType),
}

impl Var {
    pub fn from_str(var: &str) -> Result<Var, Box<dyn std::error::Error>> {
        if var.starts_with("local.") {
            return Ok(Var::Local(var.to_string()));
        }
        if var.starts_with("global.") {
            return Ok(Var::Global(var.to_string()));
        }
        if var.contains(".") {
            Err(err(format!("invalid variable prefix given: {}", var).as_str()))?;
        }
        if let Ok(n) = var.parse::<i16>() {
            return Ok(Var::Value(ValType::Num(n)))
        }
        if var.starts_with("\"") && var.ends_with("\"") {
            return Ok(Var::Value(ValType::String(String::from(
                var.strip_prefix("\"").unwrap()
                .strip_suffix("\"").unwrap()
            ))));
        }
        Self::parse_stat(var)
    }

    fn parse_stat(stat: &str) -> Result<Var, Box<dyn std::error::Error>> {
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
        Ok(Var::Stat(stat.to_string()))
    }

    pub fn get(&self, player: &Player, locals: &VarMap, globals: &VarMap) -> ValType {
        match self {
            Var::Global(k) => globals.get(k).cloned().unwrap_or(ValType::None),
            Var::Local(k) => locals.get(k).cloned().unwrap_or(ValType::None),
            Var::Stat(k) => ValType::Num(player.stat_str(k.as_str()).unwrap()), // should be already validated
            Var::Value(n) => n.clone()
        }
    }

    pub fn get_mut<'a>(&self, locals: &'a mut VarMap, globals: &'a mut VarMap) -> Option<&'a mut ValType> {
        match self {
            Var::Global(k) => Some(globals.entry(k.clone()).or_insert(ValType::None)),
            Var::Local(k) => Some(locals.entry(k.clone()).or_insert(ValType::None)),
            Var::Stat(_) => None,
            Var::Value(_) => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValType {
    Num(i16),
    String(String),
    None
}

impl ValType {
    pub fn truthy(&self) -> bool {
        match self {
            ValType::Num(n) => *n != 0,
            ValType::String(s) => !s.is_empty(),
            ValType::None => false,
        }
    }

    fn from_bool(b: bool) -> ValType {
        ValType::Num(b as i16)
    }

    fn as_num(&self) -> Result<i16, Box<dyn Error>> {
        match self {
            ValType::Num(n) => Ok(*n),
            ValType::None => Ok(0),
            ValType::String(_) => Err(err("expected number, got string")),
        }
    }

    fn compare(&self, other: &ValType) -> Option<Ordering> {
        match (self, other) {
            (ValType::Num(a), ValType::Num(b)) => a.partial_cmp(b),
            (ValType::String(a), ValType::String(b)) => a.partial_cmp(b),
            _ => None,
        }
    }

    pub fn binary(op: Op, a: ValType, b: ValType) -> Result<ValType, Box<dyn Error>> {
        Ok(match op {
            Op::Or  => Self::from_bool(a.truthy() || b.truthy()),
            Op::And => Self::from_bool(a.truthy() && b.truthy()),
            Op::Eq  => Self::from_bool(a == b),
            Op::Ne  => Self::from_bool(a != b),
            Op::Gt  => Self::from_bool(a.compare(&b) == Some(Ordering::Greater)),
            Op::Lt  => Self::from_bool(a.compare(&b) == Some(Ordering::Less)),
            Op::Ge  => Self::from_bool(matches!(a.compare(&b), Some(Ordering::Greater | Ordering::Equal))),
            Op::Le  => Self::from_bool(matches!(a.compare(&b), Some(Ordering::Less | Ordering::Equal))),

            Op::Add => match (a, b) {
                (ValType::Num(x), ValType::Num(y)) => ValType::Num(
                    x.checked_add(y).ok_or_else(|| err("overflow"))?,
                ),
                (ValType::String(x), ValType::String(y)) => ValType::String(format!("{x}{y}")),
                (ValType::String(x), ValType::Num(y)) => ValType::String(format!("{x}{y}")),
                (ValType::Num(x), ValType::String(y)) => ValType::String(format!("{x}{y}")),
                (x, ValType::None) => x,
                (ValType::None, y) => y,
            },

            Op::Sub | Op::Mul | Op::Div => {
                let (x, y) = (a.as_num()?, b.as_num()?);
                let r = match op {
                    Op::Sub => x.checked_sub(y),
                    Op::Mul => x.checked_mul(y),
                    Op::Div => x.checked_div(y),
                    _ => unreachable!(),
                };
                ValType::Num(r.ok_or_else(|| err("fix the numbers dork"))?)
            }

            _ => return Err(err("wrong operator")),
        })
    }
}

#[derive(Debug)]
pub enum Expression {
    Var(Var),
    Neg(Box<Expression>),
    Binary(Box<Expression>, Op, Box<Expression>)
}


impl Expression {
    fn assign(e1: &Expression, e2: ValType, player: &Player, locals: &mut VarMap, globals: &mut VarMap) -> Result<ValType, Box<dyn std::error::Error>> {
        if let Expression::Var(var) = e1 {
            match var {
                Var::Global(_) | Var::Local(_) => {
                    *var.get_mut(locals, globals).unwrap() = e2;
                    return Ok(var.get(player, locals, globals));
                }
                _ => ()
            }
        }
        
        Err(std::io::Error::other("").into())
    }

    pub fn calc_mut(&self, player: &Player, locals: &mut VarMap, globals: &mut VarMap) -> Result<ValType, Box<dyn std::error::Error>> {
        Ok(match self {
            Expression::Var(var) => var.get(player, locals, globals),
            Expression::Neg(e) => {
                let v = e.calc_mut(player, locals, globals)?;
                ValType::binary(Op::Sub, ValType::Num(0), v)?
            },
            Expression::Binary(e1, op , e2) => {
                let c1 = e1.calc_mut(player, locals, globals)?;
                let c2 = e2.calc_mut(player, locals, globals)?;
                match op {
                    Op::AddAssign => Self::assign(e1, ValType::binary(Op::Add, c1, c2)?, player, locals, globals)?,
                    Op::SubAssign => Self::assign(e1, ValType::binary(Op::Sub, c1, c2)?, player, locals, globals)?,
                    Op::MulAssign => Self::assign(e1, ValType::binary(Op::Mul, c1, c2)?, player, locals, globals)?,
                    Op::DivAssign => Self::assign(e1, ValType::binary(Op::Div, c1, c2)?, player, locals, globals)?,
                    Op::Assign => Self::assign(e1, c2, player, locals, globals)?,
                    _ => ValType::binary(*op, c1, c2)?,
                }
            }
        })
    }

    pub fn calc_immut(&self, player: &Player, locals: &VarMap, globals: &VarMap) -> Result<ValType, Box<dyn std::error::Error>>{
        let mut locals = locals.clone();
        let mut globals = globals.clone();
        self.calc_mut(player, &mut locals, &mut globals)
    }

    pub fn parse(src: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let tokens = tokenize(src)?;

        let mut parser = Parser {
            tokens,
            pos: 0,
        };

        let expr = parser.parse_expression(1)?;

        if parser.pos != parser.tokens.len() {
            return Err(std::io::Error::other("unexpected token").into());
        }

        Ok(expr)
    }
}