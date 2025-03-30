use crate::{
    analyzer::ast_parser::term,
    common::{
        compile_time::ast_types::node_types::{ExprNode, MathExprNode, MathExprRest},
        error::syntax_err,
        Symbol, Token, TokenStream,
    },
};

use crate::syntax_err;

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
    match current {
        Token::Symbols(sym) => match sym {
            Symbol::Add | Symbol::Sub => oper = sym,
            Symbol::RParen => return None,
            _ => syntax_err!(syntax_err::unexpected(sym)),
        },
        Token::Keyword(..) => {
            tokens.push_front(current);
            return None;
        }
        Token::EOL => return None,
        _ => syntax_err!(syntax_err::unexpected(current)),
    }

    let right_hand: ExprNode;
    if let Some(next) = tokens.pop_front() {
        right_hand = parse(tokens, next);
    } else {
        syntax_err!(syntax_err::illegal_eof());
    }

    return Some((oper, Box::new(right_hand)));
}
