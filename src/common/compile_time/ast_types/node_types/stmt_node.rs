use std::collections::VecDeque;

use super::expr_node::ExprNode;

#[derive(Clone, PartialEq)]
pub enum StmtNode {
    Var {
        name: String,
        value: Box<ExprNode>,
    },
    Out {
        value: Box<ExprNode>,
    },
    // IfElse {
    //     cond: Box<ExprNode>,
    //     if_seq: Sequence,
    //     else_seq: Option<Sequence>,
    // },
}

pub type Sequence = VecDeque<StmtNode>;

impl core::fmt::Display for StmtNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var { value, .. } => write!(f, "{}", value),
            Self::Out { value } => write!(f, "{}", value),
            // _ => todo!(),
        }
    }
}
