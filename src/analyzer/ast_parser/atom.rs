use crate::common::{
    ast_types::node_types::AtomNode, error::syntax_err, Symbol, Token, TokenStream,
};

use super::expr;

pub fn parse(tokens: &mut TokenStream, current: Token) -> AtomNode {
    match current {
        Token::Identif(idt) => return AtomNode::Idt(idt),
        Token::Literal(val) => return AtomNode::Val(val),
        Token::Symbols(Symbol::LParen) => match tokens.pop_front() {
            Some(next) => return AtomNode::Expr(Box::new(expr::parse(tokens, next))),
            None => syntax_err::report(syntax_err::illegal_eof(), file!(), line!()),
        },
        _ => syntax_err::report(syntax_err::unexpected(current), file!(), line!()),
    }
}
