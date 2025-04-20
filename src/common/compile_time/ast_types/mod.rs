pub use node_types::stmt_node::Sequence;

pub mod node_types;

pub struct Root {
    pub nodes: Sequence,
}

impl Root {
    pub fn new(nodes: Sequence) -> Self {
        Self { nodes }
    }
}
