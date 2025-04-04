use crate::common::{
    compile_time::ast_types::node_types::stmt_node::StmtNode, error, Keyword, Token, TokenStream,
};
use crate::syntax_err;

use super::{expect, expr};

pub fn parse(tokens: &mut TokenStream, current: Token) -> StmtNode {
    match current {
        Token::Keyword(kwd) => match kwd {
            Keyword::Var => parse_var_stmt(tokens),
            Keyword::Out => parse_out_stmt(tokens),
        },
        Token::EOL => {
            while let Some(next) = tokens.pop_front() {
                if next == Token::EOL {
                    continue;
                }
                return parse(tokens, next);
            }
            syntax_err!(error::illegal_eof())
        }
        _ => syntax_err!(error::unexpected(current)),
    }
}

fn parse_var_stmt(tokens: &mut TokenStream) -> StmtNode {
    let token = expect(tokens, "idt");
    if let Token::Identif(name) = token {
        expect(tokens, "=");
        if let Some(next) = tokens.pop_front() {
            let variable_val = expr::parse(tokens, next);
            return StmtNode::Var(name, Box::new(variable_val));
        } else {
            syntax_err!(error::illegal_eof())
        }
    } else {
        syntax_err!(error::unexpected(token))
    }
}

fn parse_out_stmt(tokens: &mut TokenStream) -> StmtNode {
    if let Some(next) = tokens.pop_front() {
        let output_val = expr::parse(tokens, next);
        return StmtNode::Out(Box::new(output_val));
    } else {
        syntax_err!(error::illegal_eof())
    }
}
