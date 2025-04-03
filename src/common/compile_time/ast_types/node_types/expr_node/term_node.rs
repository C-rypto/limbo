use crate::common::compile_time::token_types::Symbol;

use super::AtomNode;

#[derive(Clone, PartialEq)]
pub struct TermNode {
    pub left_hand: AtomNode,
    pub right_hand: Option<TermRest>,
}

pub type TermRest = (Symbol, Box<TermNode>);

impl TermNode {
    pub fn new(left_hand: AtomNode, right_hand: Option<TermRest>) -> TermNode {
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
