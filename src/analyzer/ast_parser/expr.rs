use crate::common::{
    compile_time::ast_types::node_types::ExprNode, error::ErrorType, Token, TokenStream,
};

use super::logic;

pub fn parse(tokens: &mut TokenStream, current: Token) -> Result<ExprNode, ErrorType> {
    return Ok(ExprNode {
        inner: logic::parse(tokens, current)?,
    });
}
