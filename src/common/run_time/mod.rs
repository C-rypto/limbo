use super::values::Value;

pub mod env;

pub fn type_eq(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(..), Value::Number(..))
        | (Value::String(..), Value::String(..))
        | (Value::Identi(..), Value::Identi(..)) => true,
        _ => false,
    }
}
