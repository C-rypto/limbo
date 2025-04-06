use crate::common::compile_time::token_types::Symbol;

use super::{ExprNode, TermNode};

#[derive(Clone, PartialEq)]
pub struct MathExprNode {
    pub left_hand: TermNode,
    pub right_hand: Option<MathExprRest>,
}

pub type MathExprRest = (Symbol, Box<ExprNode>);

impl MathExprNode {
    pub fn new(left_hand: TermNode, right_hand: Option<MathExprRest>) -> MathExprNode {
        MathExprNode {
            left_hand,
            right_hand,
        }
    }
}

impl core::fmt::Display for MathExprNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.right_hand {
            Some((op, right)) => write!(f, "{},{},{}", self.left_hand, op, right),
            None => write!(f, "{}", self.left_hand),
        }
    }
}
