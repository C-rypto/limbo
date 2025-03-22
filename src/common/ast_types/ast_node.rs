use std::collections::VecDeque;

use colored::Colorize;

use crate::common::values::Value;

use super::node_types::{ExprNode, SeqNode, StmtNode};

pub struct Root {
    pub sequence: SeqNode,
}

#[derive(Clone, PartialEq)]
pub enum ASTNode {
    Stmt(StmtNode),
    Expr(ExprNode),

    Idt(String),
    Val(Value),
}

pub type ASTStream = VecDeque<ASTNode>;

impl core::fmt::Display for ASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stmt(_) => write!(f, "Statement Node"),
            Self::Expr(_) => write!(f, "Expression Node"),
            Self::Idt(name) => write!(
                f,
                "{}{}",
                "Identifier".cyan().bold(),
                format!("{}{}{}", "[".yellow(), name, "]".yellow())
            ),
            Self::Val(val) => write!(
                f,
                "{}{}",
                "Value".cyan().bold(),
                format!("{}{}{}", "[".yellow(), val, "]".yellow())
            ),
        }
    }
}
