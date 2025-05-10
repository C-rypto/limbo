use crate::common::{
    compile_time::ast_types::node_types::expr_node::{CompNode, CompRest},
    error::{CompileErr, ErrorType},
    Symbol, Token, TokenStream, TokenType,
};

use super::math;

pub fn parse(tokens: &mut TokenStream, current: Token) -> Result<CompNode, ErrorType> {
    let left_hand = math::parse(tokens, current)?;

    if let Some(next) = tokens.next() {
        let right_hand = parse_rest(tokens, next)?;
        return Ok(CompNode::new(left_hand, right_hand));
    } else {
        return Ok(CompNode::new(left_hand, None));
    }
}

pub fn parse_rest(tokens: &mut TokenStream, current: Token) -> Result<Option<CompRest>, ErrorType> {
    let oper: Symbol;
    match &current.token_type {
        TokenType::Symbols(sym) => match sym {
            Symbol::Equal
            | Symbol::NotEqual
            | Symbol::Less
            | Symbol::LessEqual
            | Symbol::Great
            | Symbol::GreatEqual => oper = sym.clone(),
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
        TokenType::EOL => return Ok(None),
        TokenType::Unknown(..) => return Err(CompileErr::UnknownTok(Box::new(current)).into()),
        _ => return Err(CompileErr::Unexpected(Box::new(current)).into()),
    }

    let right_hand: CompNode;
    if let Some(next) = tokens.next() {
        right_hand = parse(tokens, next)?;
    } else {
        return Err(CompileErr::IllegalEOF(Box::new(current.get_end_pos())).into());
    }

    return Ok(Some((oper, Box::new(right_hand))));
}
