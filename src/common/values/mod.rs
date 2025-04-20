#![cfg_attr(debug_assertions, allow(dead_code))]
use colored::Colorize;

mod calculate;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
}

impl Value {
    pub fn try_boolean<TS>(word: &TS) -> Option<Self>
    where
        TS: ToString,
    {
        let word = word.to_string();
        if word == "true" {
            return Some(Self::Bool(true));
        } else if word == "false" {
            return Some(Self::Bool(false));
        } else {
            return None;
        }
    }

    pub fn and(a: Value, b: Value) -> Value {
        let (a, b) = (a.boolean(), b.boolean());
        return Value::Bool(a && b);
    }

    pub fn or(a: Value, b: Value) -> Value {
        let (a, b) = (a.boolean(), b.boolean());
        return Value::Bool(a || b);
    }

    pub fn is_boolean(&self) -> bool {
        if let Self::Bool(..) = self {
            return true;
        }
        return false;
    }

    pub fn is_number(&self) -> bool {
        if let Self::Number(..) = self {
            return true;
        }
        return false;
    }

    pub fn is_string(&self) -> bool {
        if let Self::String(..) = self {
            return true;
        }
        return false;
    }

    pub fn number(self) -> f64 {
        match self {
            Self::Number(num) => num,
            _ => unreachable!(),
        }
    }

    pub fn string(self) -> String {
        match self {
            Value::Number(num) => num.to_string(),
            Value::String(str) => str,
            Value::Bool(bool) => bool.to_string(),
        }
    }

    pub fn boolean(self) -> bool {
        match self {
            Value::Bool(bool) => bool,
            Value::Number(num) => num != 0.0,
            Value::String(..) => unreachable!(),
        }
    }

    pub fn output(&self) -> String {
        return self.to_string().white().to_string();
    }
}

impl core::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{}", num),
            Self::String(str) => write!(f, "{}", str),
            Self::Bool(bool) => write!(f, "{}", bool),
        }
    }
}
