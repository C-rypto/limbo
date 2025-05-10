use crate::{
    analyzer::ast_parser::term,
    common::{
        compile_time::ast_types::node_types::{MathNode, MathRest},
        error::{CompileErr, ErrorType},
        Symbol, Token, TokenStream, TokenType,
    },
};

pub fn parse(tokens: &mut TokenStream, current: Token) -> Result<MathNode, ErrorType> {
    let left_hand = term::parse(tokens, current)?;

    if let Some(next) = tokens.next() {
        let right_hand = parse_rest(tokens, next)?;
        return Ok(MathNode::new(left_hand, right_hand));
    } else {
        return Ok(MathNode::new(left_hand, None));
    }
}

fn parse_rest(tokens: &mut TokenStream, current: Token) -> Result<Option<MathRest>, ErrorType> {
    let oper: Symbol;
    match &current.token_type {
        TokenType::Symbols(sym) => match sym {
            Symbol::Add | Symbol::Sub => oper = sym.clone(),
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

    let right_hand: MathNode;
    if let Some(next) = tokens.next() {
        right_hand = parse(tokens, next)?;
    } else {
        return Err(CompileErr::IllegalEOF(Box::new(current.get_end_pos())).into());
    }

    return Ok(Some((oper, Box::new(right_hand))));
}
