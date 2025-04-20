use crate::common::Symbol;

use super::comp_node::CompNode;

#[derive(Clone, PartialEq)]
pub struct LogicNode {
    pub left_hand: CompNode,
    pub right_hand: Option<LogicRest>,
}

pub type LogicRest = (Symbol, Box<LogicNode>);

impl LogicNode {
    pub fn new(left_hand: CompNode, right_hand: Option<LogicRest>) -> Self {
        Self {
            left_hand,
            right_hand,
        }
    }
}

impl core::fmt::Display for LogicNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.right_hand {
            Some((op, right)) => write!(f, "\n- Logic: {},{},{}", self.left_hand, op, right),
            None => write!(f, "{}", self.left_hand),
        }
    }
}
