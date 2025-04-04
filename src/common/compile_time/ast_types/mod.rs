use ast_node::ASTStream;

pub mod ast_node;
pub mod node_synch;
pub mod node_types;

pub struct Root {
    pub nodes: ASTStream,
}
