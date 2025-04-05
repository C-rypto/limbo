use crate::common::values::Value;

pub enum RuntimeErr {
    Undeclared(String),
    TypeError(Value),
}

impl core::fmt::Display for RuntimeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Undeclared(name) => write!(f, "`{}` 这玩意还没定义呢", name),
            Self::TypeError(val) => write!(f, "`{}` 的类型不太对吧……", val),
        }
    }
}
