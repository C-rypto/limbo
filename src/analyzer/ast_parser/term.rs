use crate::{
    analyzer::ast_parser::atom,
    common::{
        compile_time::ast_types::node_types::{AtomNode, TermNode, TermRest},
        error::syntax_err,
        Symbol, Token, TokenStream,
    },
};
use crate::syntax_err;

pub fn parse(tokens: &mut TokenStream, current: Token) -> TermNode {
    let left_hand: AtomNode;
    match current {
        Token::Identif(..) | Token::Literal(..) | Token::Symbols(Symbol::LParen) => {
            left_hand = atom::parse(tokens, current);
        }
        _ => syntax_err!(syntax_err::unexpected(current)),
    }

    match tokens.pop_front() {
        Some(next) => {
            let right_hand = parse_rest(tokens, next);
            return TermNode::new(left_hand, right_hand);
        }
        None => return TermNode::new(left_hand, None),
    }
}

fn parse_rest(tokens: &mut TokenStream, current: Token) -> Option<TermRest> {
    let oper: Symbol;
    match &current {
        Token::Symbols(sym) => {
            if *sym == Symbol::Mul || *sym == Symbol::Div {
                oper = sym.clone();
            } else {
                tokens.push_front(current);
                return None;
            }
        }
        Token::Keyword(..) => {
            tokens.push_front(current);
            return None;
        }
        Token::EOL => return None,
        _ => syntax_err!(syntax_err::unexpected(current)),
    }

    let rest: TermNode;
    if let Some(next) = tokens.pop_front() {
        rest = parse(tokens, next);
    } else {
        syntax_err!(syntax_err::illegal_eof());
    }

    return Some((oper, Box::new(rest)));
}
