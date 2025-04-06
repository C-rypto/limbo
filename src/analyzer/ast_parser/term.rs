use crate::{
    analyzer::ast_parser::atom,
    common::{
        compile_time::ast_types::node_types::{expr_node::AtomNode, TermNode, TermRest},
        error::{CompileErr, ErrorType},
        Symbol, Token, TokenStream, TokenType,
    },
};

pub fn parse(tokens: &mut TokenStream, current: &Token) -> Result<TermNode, ErrorType> {
    let left_hand: AtomNode;
    match &current.token_type {
        TokenType::Identif(..) | TokenType::Literal(..) | TokenType::Symbols(Symbol::LParen) => {
            left_hand = atom::parse(tokens, current)?;
        }
        TokenType::Unknown(..) => {
            return Err(CompileErr::UnknownTok(Box::new(current.clone())).into())
        }
        _ => return Err(CompileErr::Unexpected(Box::new(current.clone())).into()),
    }

    match &tokens.pop_front() {
        Some(next) => {
            let right_hand = parse_rest(tokens, next.clone())?;
            return Ok(TermNode::new(left_hand, right_hand));
        }
        None => return Ok(TermNode::new(left_hand, None)),
    }
}

fn parse_rest(tokens: &mut TokenStream, current: Token) -> Result<Option<TermRest>, ErrorType> {
    let oper: Symbol;
    match &current.token_type {
        TokenType::Symbols(sym) => {
            if *sym == Symbol::Mul || *sym == Symbol::Div {
                oper = sym.clone();
            } else {
                tokens.push_front(current);
                return Ok(None);
            }
        }
        TokenType::Keyword(..) => {
            tokens.push_front(current);
            return Ok(None);
        }
        TokenType::Unknown(..) => return Err(CompileErr::UnknownTok(Box::new(current)).into()),
        TokenType::EOL => return Ok(None),
        _ => return Err(CompileErr::Unexpected(Box::new(current)).into()),
    }

    let rest: TermNode;
    if let Some(next) = tokens.pop_front() {
        rest = parse(tokens, &next)?;
    } else {
        return Err(CompileErr::IllegalEOF.into());
    }

    return Ok(Some((oper, Box::new(rest))));
}
