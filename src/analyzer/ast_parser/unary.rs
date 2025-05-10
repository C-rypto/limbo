use crate::common::{
    compile_time::ast_types::node_types::expr_node::UnaryNode,
    error::{CompileErr, ErrorType},
    Symbol, Token, TokenStream, TokenType,
};

use super::atom;

pub fn parse(tokens: &mut TokenStream, current: Token) -> Result<UnaryNode, ErrorType> {
    match &current.token_type {
        TokenType::Identif(..)
        | TokenType::Literal(..)
        | TokenType::Symbols(Symbol::LParen)
        | TokenType::Symbols(Symbol::LBrace) => {
            return Ok(UnaryNode::new(None, atom::parse(tokens, current)?))
        }
        TokenType::Symbols(symbol) => match tokens.next() {
            Some(next) => match symbol {
                Symbol::Sub | Symbol::Not => {
                    let op = symbol.clone();
                    let atom = atom::parse(tokens, next)?;
                    return Ok(UnaryNode::new(Some(op), atom));
                }
                _ => return Err(CompileErr::Unexpected(Box::new(current.clone())).into()),
            },
            None => return Err(CompileErr::IllegalEOF(Box::new(current.get_end_pos())).into()),
        },
        _ => unreachable!(),
    }
}
