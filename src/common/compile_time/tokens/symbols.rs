use crate::common::{
    error::{ErrorType, RuntimeErr},
    utils::{add_sub_type, compare_type, logical_type, mul_div_type, LocatableValue, Location},
    values::Value,
};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum Symbol {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /

    And, // &&
    Or,  // ||
    Not, // !

    Equal,      // ==
    NotEqual,   // !=
    Less,       // <
    Great,      // >
    LessEqual,  // <=
    GreatEqual, // >=

    Assign, // =
    LParen, // (
    RParen, // )
	LBrace,	// {
	RBrace,	// }

    Colon,     // :
    Semicolon, // ;
}

pub static SYMBOL_PAIR: [(&'static str, Symbol); 20] = [
    ("+", Symbol::Add),
    ("-", Symbol::Sub),
    ("*", Symbol::Mul),
    ("/", Symbol::Div),
    ("&&", Symbol::And),
    ("||", Symbol::Or),
    ("!", Symbol::Not),
    ("==", Symbol::Equal),
    ("!=", Symbol::NotEqual),
    ("<", Symbol::Less),
    ("<=", Symbol::LessEqual),
    (">", Symbol::Great),
    (">=", Symbol::GreatEqual),
    ("=", Symbol::Assign),
    ("(", Symbol::LParen),
    (")", Symbol::RParen),
	("{", Symbol::LBrace),
	("}", Symbol::RBrace),
    (":", Symbol::Colon),
    (";", Symbol::Semicolon),
];

impl Symbol {
    pub fn try_symbol<TS>(word: TS) -> Option<Self>
    where
        TS: ToString,
    {
        for pair in SYMBOL_PAIR.iter() {
            if word.to_string() == pair.0 {
                return Some(pair.1.clone());
            }
        }
        return None;
    }

    pub fn unary_operate(&self, value: Value, pos: &Location) -> Result<Value, ErrorType> {
        match (self, value.is_boolean(), value.is_number()) {
            (Self::Sub, .., true) => return Ok(-value),
            (Self::Not, true, ..) => return Ok(!value),
            _ => {
                let value = Box::new(LocatableValue(value, pos.clone()));
                return Err(RuntimeErr::TypeError(value, None).into());
            }
        }
    }

    pub fn binary_operate(
        &self,
        left: Value,
        right: Value,
        left_pos: &Location,
        right_pos: &Location,
    ) -> Result<Value, ErrorType> {
        match (
            self,
            add_sub_type(&left, &right),
            mul_div_type(&left, &right),
            compare_type(&left, &right),
            logical_type(&left, &right),
        ) {
            (Self::Add, true, ..) => return Ok(left + right),
            (Self::Sub, true, ..) => return Ok(left - right),
            (Self::Mul, _, true, ..) => return Ok(left * right),
            (Self::Div, _, true, ..) => return Ok(left / right),
            (Self::Equal, .., true, _) => return Ok(Value::Bool(left == right)),
            (Self::NotEqual, .., true, _) => return Ok(Value::Bool(left != right)),
            (Self::Less, .., true) => return Ok(Value::Bool(left < right)),
            (Self::Great, .., true) => return Ok(Value::Bool(left > right)),
            (Self::LessEqual, .., true) => return Ok(Value::Bool(left <= right)),
            (Self::GreatEqual, .., true) => return Ok(Value::Bool(left >= right)),
            (Self::And, .., true) => return Ok(Value::and(left, right)),
            (Self::Or, .., true) => return Ok(Value::or(left, right)),
            _ => {
                let left_val = Box::new(LocatableValue(left, left_pos.clone()));
                let right_val = Box::new(LocatableValue(right, right_pos.clone()));
                return Err(RuntimeErr::TypeError(left_val, Some(right_val)).into());
            }
        }
    }
}

impl core::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "{}", "+"),
            Self::Sub => write!(f, "{}", "-"),
            Self::Mul => write!(f, "{}", "*"),
            Self::Div => write!(f, "{}", "/"),

            Self::And => write!(f, "{}", "&&"),
            Self::Or => write!(f, "{}", "||"),
            Self::Not => write!(f, "{}", "!"),

            Self::Equal => write!(f, "{}", "=="),
            Self::NotEqual => write!(f, "{}", "!="),
            Self::Less => write!(f, "{}", "<"),
            Self::Great => write!(f, "{}", ">"),
            Self::LessEqual => write!(f, "{}", "<="),
            Self::GreatEqual => write!(f, "{}", ">="),

            Self::Assign => write!(f, "{}", "="),

            Self::LParen => write!(f, "{}", "("),
            Self::RParen => write!(f, "{}", ")"),
			Self::LBrace => write!(f, "{}", "{"),
			Self::RBrace => write!(f, "{}", "}"),

            Self::Colon => write!(f, "{}", ":"),
            Self::Semicolon => write!(f, "{}", ";"),
        }
    }
}
