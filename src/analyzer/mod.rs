use ast_parser::stmt;

use crate::common::{
    compile_time::ast_types::{
        ast_node::{ASTNode, ASTStream},
        Root,
    },
    error::ErrorType,
    TokenStream,
};

mod ast_parser;

pub fn analyze(tokens: &mut TokenStream) -> Result<Root, ErrorType> {
    let mut stream = ASTStream::new();

    while let Some(next) = tokens.pop_front() {
        stream.push_back(ASTNode::Stmt(stmt::parse(tokens, next)?));
    }

    return Ok(Root { nodes: stream });
}
