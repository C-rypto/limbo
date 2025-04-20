use crate::common::Symbol;

use super::MathNode;

#[derive(Clone, PartialEq)]
pub struct CompNode {
    pub left_hand: MathNode,
    pub right_hand: Option<CompRest>,
}

pub type CompRest = (Symbol, Box<CompNode>);

impl CompNode {
    pub fn new(left_hand: MathNode, right_hand: Option<CompRest>) -> CompNode {
        CompNode {
            left_hand,
            right_hand,
        }
    }
}

impl core::fmt::Display for CompNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.right_hand {
            Some((op, right)) => write!(f, "\n- Term: {},{},{}", self.left_hand, op, right),
            None => write!(f, "{}", self.left_hand),
        }
    }
}
