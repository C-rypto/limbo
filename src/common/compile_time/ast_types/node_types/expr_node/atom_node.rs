use crate::common::{
    compile_time::ast_types::node_types::BlockNode,
    utils::{Locatable, Location},
    values::Value,
};

use super::ExprNode;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub struct AtomNode {
    pub node_type: AtomType,
    pub pos: Location,
}

impl AtomNode {
    pub fn new(node_type: AtomType, pos: &Location) -> AtomNode {
        return AtomNode {
            node_type,
            pos: pos.clone(),
        };
    }
}

impl Locatable for AtomNode {
    fn locate(&self) -> String {
        return format!("{}", self.pos);
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum AtomType {
    Val(Value),
    Idt(String),
    Expr(Box<ExprNode>),
    Block(Box<BlockNode>),
}

impl core::fmt::Display for AtomNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return self.node_type.fmt(f);
    }
}

impl core::fmt::Display for AtomType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Val(val) => write!(f, "{}", val),
            Self::Idt(idt) => write!(f, "{}", idt),
            Self::Expr(exp) => write!(f, "{}", exp),
            Self::Block(blc) => write!(f, "{:?}", blc),
        }
    }
}
