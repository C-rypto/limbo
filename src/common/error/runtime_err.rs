use crate::common::values::Value;

use super::Locatable;

pub enum RuntimeErr {
    Undeclared(Box<dyn Locatable>),
    TypeError(Value),
}

impl core::fmt::Display for RuntimeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Undeclared(node) => write!(
                f,
                "\n  pos: {}\n  msg: `{}` is undeclared.",
                node.get_pos(),
                node.to_string(),
            ),
            Self::TypeError(val) => write!(f, "`{}` 的类型不太对吧……", val),
        }
    }
}
