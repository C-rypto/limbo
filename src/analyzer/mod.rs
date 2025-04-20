use ast_parser::seq;

use crate::common::{compile_time::ast_types::Root, error::ErrorType, TokenStream};

mod ast_parser;

pub fn analyze(tokens: &mut TokenStream) -> Result<Root, ErrorType> {
    return Ok(Root {
        nodes: seq::parse(tokens)?,
    });
}
