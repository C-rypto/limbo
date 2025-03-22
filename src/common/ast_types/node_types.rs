use crate::common::values::Value;

use super::ast_node::ASTStream;

#[derive(Clone, PartialEq)]
pub struct SeqNode {
    pub elements: ASTStream,
}

#[derive(Clone, PartialEq)]
pub enum StmtNode {
    Var(/* 符号表条目 */),
    Out(ExprNode),
}

#[derive(Clone, PartialEq)]
pub struct ExprNode {
    pub elements: ASTStream,
    pub value: Option<Value>,
}
