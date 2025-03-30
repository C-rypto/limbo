use crate::common::{
    compile_time::ast_types::node_types::stmt_node::StmtNode, error::syntax_err, Keyword, Token, TokenStream,
};
use crate::syntax_err;

use super::{expect, expr};

pub fn parse(tokens: &mut TokenStream, current: Token) -> StmtNode {
    if let Token::Keyword(kwd) = current {
        match kwd {
            Keyword::Var => parse_var_stmt(tokens),
            Keyword::Out => parse_out_stmt(tokens),
        }
    } else {
        syntax_err!(syntax_err::unexpected(current))
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
            syntax_err!(syntax_err::illegal_eof())
        }
    } else {
        syntax_err!(syntax_err::unexpected(token))
    }
}

fn parse_out_stmt(tokens: &mut TokenStream) -> StmtNode {
    if let Some(next) = tokens.pop_front() {
        let output_val = expr::parse(tokens, next);
        return StmtNode::Out(Box::new(output_val));
    } else {
        syntax_err!(syntax_err::illegal_eof())
    }
}
