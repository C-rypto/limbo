use std::collections::VecDeque;

use crate::common::{
    compile_time::ast_types::{Root, ToRoot},
    utils::Locatable,
};

use super::{block_node::BlockNode, expr_node::ExprNode, AssignNode};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum StmtNode {
    Var {
        inner: Box<AssignNode>,
    },
    Out {
        inner: Box<ExprNode>,
    },
    Block {
        inner: Box<BlockNode>,
    },
    IfElse {
        cond: Box<ExprNode>,
        if_stmt: Box<StmtNode>,
        else_stmt: Option<Box<StmtNode>>,
    },
    Assign {
        inner: Box<AssignNode>,
    },
}

impl ToRoot for StmtNode {
    fn to_root(self) -> crate::common::compile_time::ast_types::Root {
        Root::new(Sequence::from(self))
    }
}

impl Locatable for StmtNode {
    fn locate(&self) -> String {
        match self {
            Self::Var { inner: value } | Self::Assign { inner: value } => value.value.locate(),
            Self::Out { inner: value } => value.locate(),
            Self::Block { .. } | Self::IfElse { .. } => unreachable!(),
        }
    }
}

pub type Sequence = VecDeque<StmtNode>;

impl From<StmtNode> for Sequence {
    fn from(value: StmtNode) -> Self {
        let mut temp = Self::new();
        temp.push_back(value);
        return temp;
    }
}

impl ToRoot for Sequence {
    fn to_root(self) -> Root {
        Root::new(self)
    }
}

impl core::fmt::Display for StmtNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var { inner: value } | Self::Assign { inner: value } => {
                write!(f, "{}", value.value)
            }
            Self::Out { inner: value } => write!(f, "{}", value),
            Self::Block { inner: value, .. } => {
                for i in value.seq.iter() {
                    write!(f, "{} ", i)?;
                }
                write!(f, "")
            }
            Self::IfElse { .. } => todo!(),
        }
    }
}
