use std::ops::{Add, Div, Mul, Neg, Not, Sub};

use super::Value;

impl Add for Value {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Value::Number(num) => Value::Number(num + rhs.number()),
            Value::String(str) => Value::String(format!("{}{}", str, rhs.string())),
            Value::Bool(..) => unreachable!(),
        }
    }
}

impl Sub for Value {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Value::Number(num) => Value::Number(num - rhs.number()),
            Value::String(..) | Value::Bool(..) => unreachable!(),
        }
    }
}

impl Mul for Value {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Value::Number(num) => Value::Number(num * rhs.number()),
            Value::String(str) => Value::String(str.repeat(rhs.number() as usize)),
            Value::Bool(..) => unreachable!(),
        }
    }
}

impl Div for Value {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Value::Number(num) => Value::Number(num / rhs.number()),
            Value::String(..) | Value::Bool(..) => unreachable!(),
        }
    }
}

impl Neg for Value {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Value::Number(num) => Value::Number(-num),
            Value::String(..) | Value::Bool(..) => unreachable!(),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
            (Value::Bool(a), Value::Bool(b)) => a.partial_cmp(b),
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl Not for Value {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Value::Bool(bool) => Value::Bool(!bool),
            Value::Number(..) => Value::Bool(!self.boolean()),
            Value::String(..) => unreachable!(),
        }
    }
}
