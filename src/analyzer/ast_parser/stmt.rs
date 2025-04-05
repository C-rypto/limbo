use crate::{
    common::{
        compile_time::ast_types::node_types::stmt_node::StmtNode, error::CompileErr, Keyword,
        Token, TokenStream, TokenType,
    },
    err_report,
};

use super::{expect, expr};

pub fn parse(tokens: &mut TokenStream, current: Token) -> StmtNode {
    match current.token_type {
        TokenType::Keyword(kwd) => match kwd {
            Keyword::Var => parse_var_stmt(tokens),
            Keyword::Out => parse_out_stmt(tokens),
        },
        TokenType::EOL => {
            while let Some(next) = tokens.pop_front() {
                if next.token_type == TokenType::EOL {
                    continue;
                }
                return parse(tokens, next);
            }
            err_report!(CompileErr::IllegalEOF.into())
        }
		TokenType::Unknown(..) => err_report!(CompileErr::UnknownTok(current).into()),
        _ => err_report!(CompileErr::Unexpected(current.to_string()).into()),
    }
}

fn parse_var_stmt(tokens: &mut TokenStream) -> StmtNode {
    let token = expect(tokens, "idt");
    if let TokenType::Identif(name) = token.token_type {
        expect(tokens, "=");
        if let Some(next) = tokens.pop_front() {
            let variable_val = expr::parse(tokens, next);
            return StmtNode::Var(name, Box::new(variable_val));
        } else {
            err_report!(CompileErr::IllegalEOF.into())
        }
    } else {
        err_report!(CompileErr::Unexpected(token.to_string()).into())
    }
}

fn parse_out_stmt(tokens: &mut TokenStream) -> StmtNode {
    if let Some(next) = tokens.pop_front() {
        let output_val = expr::parse(tokens, next);
        return StmtNode::Out(Box::new(output_val));
    } else {
        err_report!(CompileErr::IllegalEOF.into())
    }
}
