use crate::common::{
    compile_time::ast_types::node_types::{expr_node::AtomNode, AtomNodeType},
    error::{CompileErr, ErrorType},
    Symbol, Token, TokenStream, TokenType,
};

use super::expr;

pub fn parse(tokens: &mut TokenStream, current: &Token) -> Result<AtomNode, ErrorType> {
    match &current.token_type {
        TokenType::Identif(idt) => {
            return Ok(AtomNode::new(
                AtomNodeType::Idt(idt.to_string()),
                &current.pos,
            ))
        }
        TokenType::Literal(val) => {
            return Ok(AtomNode::new(AtomNodeType::Val(val.clone()), &current.pos))
        }
        TokenType::Symbols(Symbol::LParen) => match tokens.next() {
            Some(next) => {
                return Ok(AtomNode::new(
                    AtomNodeType::Expr(Box::new(expr::parse(tokens, &next)?)),
                    &current.pos,
                ))
            }
            None => return Err(CompileErr::IllegalEOF(Box::new(current.get_end_pos())).into()),
        },
        TokenType::Unknown(..) => {
            return Err(CompileErr::UnknownTok(Box::new(current.clone())).into())
        }
        _ => return Err(CompileErr::Unexpected(Box::new(current.clone())).into()),
    }
}
