use ast_parser::stmt;

use crate::common::{
    compile_time::ast_types::{
        ast_node::{ASTNode, ASTStream},
        Root,
    },
    TokenStream,
};

mod ast_parser;

pub fn analyze(tokens: &mut TokenStream) -> Root {
    let mut stream = ASTStream::new();

    while let Some(next) = tokens.pop_front() {
        stream.push_back(ASTNode::Stmt(stmt::parse(tokens, next)));
    }

    return Root { nodes: stream };
}
