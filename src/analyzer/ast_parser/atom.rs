use crate::{
    common::{
        compile_time::ast_types::node_types::AtomNode, error::CompileErr, Symbol, Token,
        TokenStream, TokenType,
    },
    err_report,
};

use super::expr;

pub fn parse(tokens: &mut TokenStream, current: Token) -> AtomNode {
    match current.token_type {
        TokenType::Identif(idt) => return AtomNode::Idt(idt),
        TokenType::Literal(val) => return AtomNode::Val(val),
        TokenType::Symbols(Symbol::LParen) => match tokens.pop_front() {
            Some(next) => return AtomNode::Expr(Box::new(expr::parse(tokens, next))),
            None => err_report!(CompileErr::IllegalEOF.into()),
        },
		TokenType::Unknown(..) => err_report!(CompileErr::UnknownTok(current).into()),
        _ => err_report!(CompileErr::Unexpected(current.to_string()).into()),
    }
}
