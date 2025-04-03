use crate::common::values::Value;

use super::ExprNode;

#[derive(Clone, PartialEq)]
pub enum AtomNode {
    Val(Value),
    Idt(String),
    Expr(Box<ExprNode>),
}

impl core::fmt::Display for AtomNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Val(val) => write!(f, "\n- Atom: {}", val),
            Self::Idt(idt) => write!(f, "\n- Atom: {}", idt),
            Self::Expr(exp) => write!(f, "\n- Atom: {}", exp),
        }
    }
}
