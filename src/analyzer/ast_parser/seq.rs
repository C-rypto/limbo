use crate::common::{compile_time::ast_types::Sequence, error::ErrorType, TokenStream};

use super::stmt;

pub fn parse(tokens: &mut TokenStream) -> Result<Sequence, ErrorType> {
    let mut seq = Sequence::new();

    while let Some(next) = tokens.next() {
        seq.push_back(stmt::parse(tokens, next)?);
        tokens.skip_white_space();
    }

    return Ok(seq);
}
