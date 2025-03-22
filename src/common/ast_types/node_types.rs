use crate::common::{values::Value, Symbol, Token};

use super::ast_node::ASTStream;

#[derive(Clone, PartialEq)]
pub struct SeqNode {
    pub elements: ASTStream,
}

#[derive(Clone, PartialEq)]
pub enum StmtNode {
    Var(String),
    Out(Box<ExprNode>),
}

#[derive(Clone, PartialEq)]
pub enum ExprNode {
    Atom(Box<AtomNode>),
    Term(Box<TermNode>),
    Expr(ASTStream),
}

impl From<Token> for  ExprNode {
	fn from(value: Token) -> Self {
		match value {
			Token::Identif(idt) => ExprNode::Atom(Box::new(AtomNode::Idt(idt))),
			Token::Literal(val) => ExprNode::Atom(Box::new(AtomNode::Val(val))),
			T
		}
	}
}

#[derive(Clone, PartialEq)]
pub enum AtomNode {
    Val(Value),
    Idt(String),
    Expr(Box<ExprNode>),
}

#[derive(Clone, PartialEq)]
pub struct TermNode {
    pub left: Box<ExprNode>,
    pub oper: Symbol,
    pub right: Box<ExprNode>,
}
