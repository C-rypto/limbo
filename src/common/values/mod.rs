use std::ops::{Add, Div, Mul, Sub};

use colored::Colorize;

use crate::runtime_err;

use super::run_time::type_eq;

#[derive(Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),

    Identi(String),
}

impl Value {
    pub fn number(self) -> f64 {
        match self {
            Value::Number(num) => num,
            _ => unreachable!(),
        }
    }

    pub fn string(self) -> String {
        match self {
            Value::String(str) => str,
            _ => unreachable!(),
        }
    }
}

impl core::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(number) => write!(
                f,
                "{}",
                number.to_string().custom_color((181, 206, 168)).bold()
            ),
            Self::String(string) => write!(f, "\"{}\"", string.green().bold()),
            Self::Identi(idt) => write!(f, "{}", idt.cyan().bold()),
        }
    }
}

impl Add for Value {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		match self {
			Value::Number(num) => {
				if type_eq(&self, &rhs) {
					return Value::Number(num + rhs.number());
				}
				else {
					runtime_err!("")
				}
			},
			Value::String(str) => Value::String(format!("{}{}",str,rhs.string())),
			_ => unreachable!()
		}
	}
}

impl Sub for Value {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		match self {
			Value::Number(num) => Value::Number(num - rhs.number()),
			_ => unreachable!()
		}
	}
}

impl Mul for Value {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output {
		match self {
			Value::Number(num) => Value::Number(num * rhs.number()),
			Value::String(str) => Value::String(str.repeat(rhs.number() as usize)),
			_ => unreachable!()
		}
	}
}

impl Div for Value {
	type Output = Self;
	fn div(self, rhs: Self) -> Self::Output {
		match self {
			Value::Number(num) => Value::Number(num / rhs.number()),
			_ => unreachable!()
		}
	}
}
