use std::collections::VecDeque;

use super::node_types::{expr_node::ExprNode, stmt_node::StmtNode};

#[derive(Clone, PartialEq)]
pub enum ASTNode {
    Stmt(StmtNode),
    Expr(ExprNode),
}

pub type ASTStream = VecDeque<ASTNode>;

impl From<ExprNode> for ASTNode {
    fn from(value: ExprNode) -> Self {
        ASTNode::Expr(value)
    }
}

impl From<StmtNode> for ASTNode {
    fn from(value: StmtNode) -> Self {
        ASTNode::Stmt(value)
    }
}

impl core::fmt::Display for ASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stmt(node) => write!(f, "Stmt{}", node),
            Self::Expr(_) => write!(f, "Expression Node"),
        }
    }
}
