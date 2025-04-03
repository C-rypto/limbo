use crate::common::{
    compile_time::ast_types::node_types::AtomNode, error, Symbol, Token, TokenStream,
};

use crate::syntax_err;

use super::expr;

pub fn parse(tokens: &mut TokenStream, current: Token) -> AtomNode {
    match current {
        Token::Identif(idt) => return AtomNode::Idt(idt),
        Token::Literal(val) => return AtomNode::Val(val),
        Token::Symbols(Symbol::LParen) => match tokens.pop_front() {
            Some(next) => return AtomNode::Expr(Box::new(expr::parse(tokens, next))),
            None => syntax_err!(error::illegal_eof()),
        },
        _ => syntax_err!(error::unexpected(current)),
    }
}
