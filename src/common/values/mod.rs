use std::ops::{Add, Div, Mul, Sub};

use colored::Colorize;

use crate::err_report;

use super::error::RuntimeErr;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
}

impl Value {
    pub fn number(self) -> f64 {
        match self {
            Self::Number(num) => num,
            _ => err_report!(RuntimeErr::TypeError(self).into()),
        }
    }

    pub fn string(self) -> String {
        match self {
            Value::Number(num) => num.to_string(),
            Value::String(str) => str,
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
        }
    }
}

impl Add for Value {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Value::Number(num) => Value::Number(num + rhs.number()),
            Value::String(str) => Value::String(format!("{}{}", str, rhs.string())),
        }
    }
}

impl Sub for Value {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Value::Number(num) => Value::Number(num - rhs.number()),
            Value::String(..) => err_report!(RuntimeErr::TypeError(self).into()),
        }
    }
}

impl Mul for Value {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Value::Number(num) => Value::Number(num * rhs.number()),
            Value::String(str) => Value::String(str.repeat(rhs.number() as usize)),
        }
    }
}

impl Div for Value {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Value::Number(num) => Value::Number(num / rhs.number()),
            Value::String(..) => err_report!(RuntimeErr::TypeError(self).into()),
        }
    }
}
