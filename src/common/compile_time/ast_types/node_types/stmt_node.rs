use super::expr_node::ExprNode;

#[derive(Clone, PartialEq)]
pub enum StmtNode {
    Var(String, Box<ExprNode>),
    Out(Box<ExprNode>),
}
