use core::iter::Iterator;

use crate::{
    game::{Player, VarMap}
};

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
                let var = VarType::from_str(name)?;
                self.pos += 1;

                Ok(Expression::Var(var))
            }

            Some(Token::LParen) => {
                self.pos += 1;

                let expr = self.parse_expression(1)?;

                match self.peek() {
                    Some(Token::RParen) => self.pos += 1,
                    _ => return Err(std::io::Error::other("expected closing parenthesis").into()),
                }

                Ok(expr)
            }

            Some(Token::Op(Op::Sub)) => {
                self.pos += 1;

                let expr = self.parse_expression(8)?;

                Ok(Expression::Neg(Box::new(expr)))
            }

            _ => Err(std::io::Error::other("expected expression").into()),
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
        } else if let Some((s, op)) = OPS.iter().find(|(s, _)| src[i..].starts_with(s)) {
            tokens.push(Token::Op(*op));
            i += s.len();
        } else {
            return Err(Box::new(std::io::Error::other(format!("unexpected character '{}' at {}", c, i))));
        }
    }
    Ok(tokens)
}

#[derive(Debug, PartialEq)]
pub enum VarType {
    Stat(String),
    Local(String),
    Global(String),
    Num(i16),
}

impl VarType {
    pub fn from_str(var: &str) -> Result<VarType, Box<dyn std::error::Error>> {
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

    pub fn get(&self, player: &Player, locals: &VarMap, globals: &VarMap) -> i16 {
        match self {
            VarType::Global(k) => globals.get(k).copied().unwrap_or(0),
            VarType::Local(k) => locals.get(k).copied().unwrap_or(0),
            VarType::Stat(k) => player.stat_str(k.as_str()).unwrap(), // should be already validated
            VarType::Num(n) => *n
        }
    }

    pub fn get_mut<'a>(&self, locals: &'a mut VarMap, globals: &'a mut VarMap) -> Option<&'a mut i16> {
        match self {
            VarType::Global(k) => Some(globals.entry(k.clone()).or_insert(0)),
            VarType::Local(k) => Some(locals.entry(k.clone()).or_insert(0)),
            VarType::Stat(_) => None,
            VarType::Num(_) => None,
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Var(VarType),
    Neg(Box<Expression>),
    Binary(Box<Expression>, Op, Box<Expression>)
}


impl Expression {
    fn assign(e1: &Expression, e2: i16, player: &Player, locals: &mut VarMap, globals: &mut VarMap) -> Result<i16, Box<dyn std::error::Error>> {
        if let Expression::Var(var) = e1 {
            match var {
                VarType::Global(_) | VarType::Local(_) => {
                    *var.get_mut(locals, globals).unwrap() = e2;
                    return Ok(var.get(player, locals, globals));
                }
                _ => ()
            }
        }
        
        Err(std::io::Error::other("").into())
    }

    pub fn calc_mut(&self, player: &Player, locals: &mut VarMap, globals: &mut VarMap) -> Result<i16, Box<dyn std::error::Error>> {
        Ok(match self {
            Expression::Var(var) => var.get(player, locals, globals),
            Expression::Neg(e) => e.calc_mut(player, locals, globals)? * -1,
            Expression::Binary(e1, op , e2) => {
                let c1 = e1.calc_mut(player, locals, globals)?;
                let c2 = e2.calc_mut(player, locals, globals)?;
                match op {
                    Op::Or => ((c1 != 0) || (c2 != 0)) as i16,
                    Op::And => ((c1 != 0) && (c2 != 0)) as i16,
                    Op::Eq => (c1 == c2) as i16,
                    Op::Ne => (c1 != c2) as i16,
                    Op::Ge => (c1 >= c2) as i16,
                    Op::Le => (c1 <= c2) as i16,
                    Op::AddAssign => Self::assign(e1, c1 + c2, player, locals, globals)?,
                    Op::SubAssign => Self::assign(e1, c1 - c2, player, locals, globals)?,
                    Op::MulAssign => Self::assign(e1, c1 * c2, player, locals, globals)?,
                    Op::DivAssign => Self::assign(e1, c1 / c2, player, locals, globals)?,
                    Op::Gt => (c1 > c2) as i16,
                    Op::Lt => (c1 < c2) as i16,
                    Op::Add => c1 + c2,
                    Op::Sub => c1 - c2,
                    Op::Mul => c1 * c2,
                    Op::Div => c1 / c2,
                    Op::Assign => Self::assign(e1, c2, player, locals, globals)?,
                }
            }
        })
    }

    pub fn calc_immut(&self, player: &Player, locals: &VarMap, globals: &VarMap) -> Result<i16, Box<dyn std::error::Error>>{
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