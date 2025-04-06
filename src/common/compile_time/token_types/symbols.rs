use crate::common::{
    error::{ErrorType, RuntimeErr},
    utils::{add_sub_type, mul_div_type, LocatableValue, Location},
    values::Value,
};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum Symbol {
    Add,
    Sub,
    Mul,
    Div,

    Equal,

    LParen,
    RParen,
}

impl Symbol {
    pub fn is_symbol(ch: char) -> Option<Self> {
        for pair in SYMBOL_PAIR.iter() {
            if ch == pair.0 {
                return Some(pair.1.clone());
            }
        }
        return None;
    }

    pub fn binary_operate(
        &self,
        left: Value,
        right: Value,
		left_pos: &Location,
        right_pos: &Location,
    ) -> Result<Value, ErrorType> {
        match (self, add_sub_type(&left, &right), mul_div_type(&left, &right)) {
			(Self::Add, true, ..) => return Ok(left+right),
			(Self::Sub, true, ..) => return Ok(left-right),
			(Self::Mul, .., true) => return Ok(left*right),
			(Self::Div, .., true) => return Ok(left/right),
			_ => {
				let left_val = Box::new(LocatableValue(left, left_pos.clone()));
				let right_val = Box::new(LocatableValue(right, right_pos.clone()));
				return Err(RuntimeErr::TypeError(left_val, right_val).into());
			}
		}
    }
}

pub static SYMBOL_PAIR: [(char, Symbol); 7] = [
    ('+', Symbol::Add),
    ('-', Symbol::Sub),
    ('*', Symbol::Mul),
    ('/', Symbol::Div),
    ('=', Symbol::Equal),
    ('(', Symbol::LParen),
    (')', Symbol::RParen),
];

impl core::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "{}", "+"),
            Self::Sub => write!(f, "{}", "-"),
            Self::Mul => write!(f, "{}", "*"),
            Self::Div => write!(f, "{}", "/"),

            Self::Equal => write!(f, "{}", "="),

            Self::LParen => write!(f, "{}", "("),
            Self::RParen => write!(f, "{}", ")"),
        }
    }
}
