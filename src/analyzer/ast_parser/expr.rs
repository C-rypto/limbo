use crate::{
    analyzer::ast_parser::term,
    common::{
        compile_time::ast_types::node_types::{ExprNode, MathExprNode, MathExprRest},
        error::CompileErr,
        Symbol, Token, TokenStream, TokenType,
    },
    err_report,
};

pub fn parse(tokens: &mut TokenStream, current: Token) -> ExprNode {
    let left_hand = term::parse(tokens, current);

    if let Some(next) = tokens.pop_front() {
        let right_hand = parse_rest(tokens, next);
        return ExprNode::Math(MathExprNode::new(left_hand, right_hand));
    } else {
        return ExprNode::Math(MathExprNode::new(left_hand, None));
    }
}

fn parse_rest(tokens: &mut TokenStream, current: Token) -> Option<MathExprRest> {
    let oper: Symbol;
    match current.token_type {
        TokenType::Symbols(sym) => match sym {
            Symbol::Add | Symbol::Sub => oper = sym,
            Symbol::RParen => return None,
            _ => err_report!(CompileErr::Unexpected(sym.to_string()).into()),
        },
        TokenType::Keyword(..) => {
            tokens.push_front(current);
            return None;
        }
        TokenType::EOL => return None,
		TokenType::Unknown(..) => err_report!(CompileErr::UnknownTok(current).into()),
        _ => err_report!(CompileErr::Unexpected(current.to_string()).into()),
    }

    let right_hand: ExprNode;
    if let Some(next) = tokens.pop_front() {
        right_hand = parse(tokens, next);
    } else {
        err_report!(CompileErr::IllegalEOF.into())
    }

    return Some((oper, Box::new(right_hand)));
}
