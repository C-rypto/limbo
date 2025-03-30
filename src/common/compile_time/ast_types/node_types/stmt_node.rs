use super::expr_node::ExprNode;

#[derive(Clone, PartialEq)]
pub enum StmtNode {
    Var(String, Box<ExprNode>),
    Out(Box<ExprNode>),
}

impl core::fmt::Display for StmtNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var(name, val) => write!(f, "\n- Var: {},{}", name, val),
            Self::Out(val) => write!(f, "\n- Out: {}", val),
        }
    }
}
