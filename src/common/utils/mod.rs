use super::values::Value;

mod location;
mod source;

pub use {location::*, source::*};

pub fn indent(level: usize) -> String {
    return "    ".repeat(level);
}

pub fn mul_div_type(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(..), Value::Number(..)) | (Value::String(..), Value::String(..)) => {
            return true
        }
        _ => return false,
    }
}

pub fn add_sub_type(a: &Value, b: &Value) -> bool {
    if mul_div_type(&a, &b) {
        return true;
    } else {
        match (a, b) {
            (Value::Number(..), Value::String(..)) | (Value::String(..), Value::Number(..)) => {
                return true
            }
            _ => return false,
        }
    }
}
