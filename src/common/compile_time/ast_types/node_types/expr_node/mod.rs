mod atom_node;
mod math_expr;
mod term_node;

use crate::common::values::Value;
pub use {atom_node::*, math_expr::*, term_node::*};

#[derive(Clone, PartialEq)]
pub enum ExprNode {
    Math(MathExprNode),
}

impl ExprNode {
    pub fn value(&self) -> Value {
        todo!()
    }
}

impl core::fmt::Display for ExprNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Math(math) => write!(f, "{}", math),
        }
    }
}
