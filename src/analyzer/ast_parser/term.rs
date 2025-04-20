use crate::common::{
    compile_time::ast_types::node_types::{expr_node::UnaryNode, TermNode, TermRest},
    error::{CompileErr, ErrorType},
    Symbol, Token, TokenStream, TokenType,
};

use super::unary;

pub fn parse(tokens: &mut TokenStream, current: &Token) -> Result<TermNode, ErrorType> {
    let left_hand: UnaryNode;
    match &current.token_type {
        TokenType::Identif(..) | TokenType::Literal(..) | TokenType::Symbols(..) => {
            left_hand = unary::parse(tokens, current)?;
        }
        TokenType::Unknown(..) => {
            return Err(CompileErr::UnknownTok(Box::new(current.clone())).into())
        }
        _ => return Err(CompileErr::Unexpected(Box::new(current.clone())).into()),
    }

    match &tokens.next() {
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
        TokenType::Symbols(sym) => match sym {
            Symbol::Mul | Symbol::Div => oper = sym.clone(),
            Symbol::RParen => return Ok(None),
            _ => {
                tokens.undo();
                return Ok(None);
            }
        },
        TokenType::Keyword(..) => {
            tokens.undo();
            return Ok(None);
        }
        TokenType::Unknown(..) => return Err(CompileErr::UnknownTok(Box::new(current)).into()),
        TokenType::EOL => return Ok(None),
        _ => return Err(CompileErr::Unexpected(Box::new(current)).into()),
    }

    let rest: TermNode;
    if let Some(next) = tokens.next() {
        rest = parse(tokens, &next)?;
    } else {
        return Err(CompileErr::IllegalEOF(Box::new(current.get_end_pos())).into());
    }

    return Ok(Some((oper, Box::new(rest))));
}
