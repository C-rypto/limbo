use crate::common::Symbol;

use super::AtomNode;

#[derive(Clone, PartialEq)]
pub struct UnaryNode {
    pub op: Option<Symbol>,
    pub atom: AtomNode,
}

impl UnaryNode {
    pub fn new(op: Option<Symbol>, atom: AtomNode) -> UnaryNode {
        UnaryNode { op, atom }
    }
}

impl core::fmt::Display for UnaryNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.op {
            Some(oper) => write!(f, "{}{}", oper, self.atom),
            None => write!(f, "{}", self.atom),
        }
    }
}
