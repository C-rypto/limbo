use crate::common::compile_time::tokens::Symbol;

use super::TermNode;

#[derive(Clone, PartialEq)]
pub struct MathNode {
    pub left_hand: TermNode,
    pub right_hand: Option<MathRest>,
}

pub type MathRest = (Symbol, Box<MathNode>);

impl MathNode {
    pub fn new(left_hand: TermNode, right_hand: Option<MathRest>) -> MathNode {
        MathNode {
            left_hand,
            right_hand,
        }
    }
}

impl core::fmt::Display for MathNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.right_hand {
            Some((op, right)) => write!(f, "{},{},{}", self.left_hand, op, right),
            None => write!(f, "{}", self.left_hand),
        }
    }
}
