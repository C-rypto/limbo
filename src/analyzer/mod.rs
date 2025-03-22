use ast_parser::seq;

use crate::{common::ast_types::ast_node::Root, common::TokenStream};

mod ast_parser;

pub fn analyze(tokens: &mut TokenStream) -> Root {
    let seq = seq::parse(tokens);
    let root = Root { sequence: seq };
    root
}
