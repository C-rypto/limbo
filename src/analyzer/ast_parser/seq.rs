use crate::common::{
    compile_time::ast_types::Sequence, error::ErrorType, Symbol, TokenStream, TokenType
};

use super::stmt;

pub fn parse(tokens: &mut TokenStream) -> Result<Sequence, ErrorType> {
    let mut seq = Sequence::new();

    while let Some(next) = tokens.next() {
        if let TokenType::Symbols(Symbol::Semicolon)
		// |TokenType::Keyword(Keyword::Else)
		 = next.token_type {
            tokens.undo();
            break;
        }
        seq.push_back(stmt::parse(tokens, next)?);
    }

    return Ok(seq);
}
