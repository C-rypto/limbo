use std::collections::VecDeque;

use crate::common::utils::Locatable;

use super::{block_node::BlockNode, expr_node::ExprNode};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum StmtNode {
    Var { name: String, value: Box<ExprNode> },
    Out { value: Box<ExprNode> },
    Block { value: Box<BlockNode> },
}

impl Locatable for StmtNode {
	fn locate(&self) -> String {
		match self {
			Self::Var { value, .. } | Self::Out { value } => value.locate(),
			Self::Block { .. } => unreachable!(),
		}
	}
}

pub type Sequence = VecDeque<StmtNode>;

impl core::fmt::Display for StmtNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var { value, .. } => write!(f, "{}", value),
            Self::Out { value } => write!(f, "{}", value),
            Self::Block { value, .. } => {
                for i in value.seq.iter() {
                    write!(f, "{} ", i)?;
                }
                write!(f, "")
            } // _ => todo!(),
        }
    }
}
