use std::collections::VecDeque;

use crate::common::Token;

use super::node_types::{AtomNode, ExprNode, SeqNode, StmtNode};

pub struct Root {
    pub sequence: SeqNode,
}

#[derive(Clone, PartialEq)]
pub enum ASTNode {
    Stmt(StmtNode),
    Expr(ExprNode),
}

pub type ASTStream = VecDeque<ASTNode>;

impl From<Token> for ASTNode {
    fn from(value: Token) -> Self {
        match value {
            Token::Identif(idt) => Self::Expr(ExprNode::Atom(Box::new(AtomNode::Idt(idt)))),
            Token::Literal(lit) => Self::Expr(ExprNode::Atom(Box::new(AtomNode::Val(lit)))),
            _ => unreachable!(),
        }
    }
}

impl core::fmt::Display for ASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stmt(_) => write!(f, "Statement Node"),
            Self::Expr(_) => write!(f, "Expression Node"),
        }
    }
}
