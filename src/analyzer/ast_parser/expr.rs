use crate::{
    analyzer::ast_parser::term,
    common::{
        compile_time::ast_types::node_types::{ExprNode, MathExprNode, MathExprRest},
        error::{CompileErr, ErrorType},
        Symbol, Token, TokenStream, TokenType,
    },
};

pub fn parse(tokens: &mut TokenStream, current: &Token) -> Result<ExprNode, ErrorType> {
    let left_hand = term::parse(tokens, current)?;

    if let Some(next) = tokens.pop_front() {
        let right_hand = parse_rest(tokens, next)?;
        return Ok(ExprNode::Math(MathExprNode::new(left_hand, right_hand)));
    } else {
        return Ok(ExprNode::Math(MathExprNode::new(left_hand, None)));
    }
}

fn parse_rest(tokens: &mut TokenStream, current: Token) -> Result<Option<MathExprRest>, ErrorType> {
    let oper: Symbol;
    match &current.token_type {
        TokenType::Symbols(sym) => match sym {
            Symbol::Add | Symbol::Sub => oper = sym.clone(),
            Symbol::RParen => return Ok(None),
            _ => return Err(CompileErr::Unexpected(Box::new(current)).into()),
        },
        TokenType::Keyword(..) => {
            tokens.push_front(current);
            return Ok(None);
        }
        TokenType::EOL => return Ok(None),
        TokenType::Unknown(..) => return Err(CompileErr::UnknownTok(Box::new(current)).into()),
        _ => return Err(CompileErr::Unexpected(Box::new(current)).into()),
    }

    let right_hand: ExprNode;
    if let Some(next) = tokens.pop_front() {
        right_hand = parse(tokens, &next)?;
    } else {
        return Err(CompileErr::IllegalEOF.into());
    }

    return Ok(Some((oper, Box::new(right_hand))));
}
