use crate::{
    analyzer::ast_parser::atom,
    common::{
        compile_time::ast_types::node_types::{AtomNode, TermNode, TermRest},
        error::CompileErr,
        Symbol, Token, TokenStream, TokenType,
    },
    err_report,
};

pub fn parse(tokens: &mut TokenStream, current: Token) -> TermNode {
    let left_hand: AtomNode;
    match current.token_type {
        TokenType::Identif(..) | TokenType::Literal(..) | TokenType::Symbols(Symbol::LParen) => {
            left_hand = atom::parse(tokens, current);
        }
		TokenType::Unknown(..) => err_report!(CompileErr::UnknownTok(current).into()),
        _ => err_report!(CompileErr::Unexpected(current.to_string()).into()),
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
    match &current.token_type {
        TokenType::Symbols(sym) => {
            if *sym == Symbol::Mul || *sym == Symbol::Div {
                oper = sym.clone();
            } else {
                tokens.push_front(current);
                return None;
            }
        }
        TokenType::Keyword(..) => {
            tokens.push_front(current);
            return None;
        }
		TokenType::Unknown(..) => err_report!(CompileErr::UnknownTok(current).into()),
        TokenType::EOL => return None,
        _ => err_report!(CompileErr::Unexpected(current.to_string()).into()),
    }

    let rest: TermNode;
    if let Some(next) = tokens.pop_front() {
        rest = parse(tokens, next);
    } else {
        err_report!(CompileErr::IllegalEOF.into())
    }

    return Some((oper, Box::new(rest)));
}
