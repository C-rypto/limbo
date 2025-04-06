use crate::common::{
    utils::{Locatable, Location},
    values::Value,
};

use super::ExprNode;

#[derive(Clone, PartialEq)]
pub struct AtomNode {
    pub node_type: AtomNodeType,
    pub pos: Location,
}

impl AtomNode {
    pub fn new(node_type: AtomNodeType, pos: &Location) -> AtomNode {
        return AtomNode {
            node_type,
            pos: pos.clone(),
        };
    }
}

impl Locatable for AtomNode {
    fn locate(&self) -> String {
        return format!("{}:{}:{}", self.pos.0, self.pos.1, self.pos.2);
    }
}

#[derive(Clone, PartialEq)]
pub enum AtomNodeType {
    Val(Value),
    Idt(String),
    Expr(Box<ExprNode>),
}

impl core::fmt::Display for AtomNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return self.node_type.fmt(f);
    }
}

impl core::fmt::Display for AtomNodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Val(val) => write!(f, "{}", val),
            Self::Idt(idt) => write!(f, "{}", idt),
            Self::Expr(exp) => write!(f, "{}", exp),
        }
    }
}
