mod atom_node;
mod comp_node;
mod logic_node;
mod math_node;
mod term_node;
mod unary_node;

use crate::common::utils::Locatable;
pub use {atom_node::*, comp_node::*, logic_node::*, math_node::*, term_node::*, unary_node::*};

#[derive(Clone, PartialEq)]
pub struct ExprNode {
    pub inner: LogicNode,
}

impl Locatable for ExprNode {
    fn locate(&self) -> String {
        return format!(
            "{}",
            self.inner
                .left_hand
                .left_hand
                .left_hand
                .left_hand
                .atom
                .locate()
        );
    }
}

impl core::fmt::Display for ExprNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return self.inner.fmt(f);
    }
}
