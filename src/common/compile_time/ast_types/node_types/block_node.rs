use super::stmt_node::Sequence;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub struct BlockNode {
    pub seq: Box<Sequence>,
}
