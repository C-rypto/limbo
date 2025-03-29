use crate::common::{
    ast_types::node_types::stmt_node::StmtNode, error::syntax_err, Keyword, Token, TokenStream,
};

use super::{expect, expr};

pub fn parse(tokens: &mut TokenStream, current: Token) -> StmtNode {

    if let Token::Keyword(kwd) = current {
        match kwd {
            Keyword::Var => parse_var_stmt(tokens),
            Keyword::Out => parse_out_stmt(tokens),
        }
    } else {
        syntax_err::report(syntax_err::unexpected(current), file!(), line!());
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
            syntax_err::report(syntax_err::illegal_eof(), file!(), line!());
        }
    } else {
        syntax_err::report(syntax_err::unexpected(token), file!(), line!());
    }
}

fn parse_out_stmt(tokens: &mut TokenStream) -> StmtNode {
    if let Some(next) = tokens.pop_front() {
        let output_val = expr::parse(tokens, next);
        return StmtNode::Out(Box::new(output_val));
    } else {
        syntax_err::report(syntax_err::illegal_eof(), file!(), line!());
    }
}
