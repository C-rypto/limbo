use crate::common::compile_time::tokens::Symbol;

use super::unary_node::UnaryNode;

#[derive(Clone, PartialEq)]
pub struct TermNode {
    pub left_hand: UnaryNode,
    pub right_hand: Option<TermRest>,
}

pub type TermRest = (Symbol, Box<TermNode>);

impl TermNode {
    pub fn new(left_hand: UnaryNode, right_hand: Option<TermRest>) -> TermNode {
        TermNode {
            left_hand,
            right_hand,
        }
    }
}

impl core::fmt::Display for TermNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.right_hand {
            Some((op, right)) => write!(f, "\n- Term: {},{},{}", self.left_hand, op, right),
            None => write!(f, "{}", self.left_hand),
        }
    }
}
