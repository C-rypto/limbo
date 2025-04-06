use crate::common::{
    compile_time::ast_types::node_types::stmt_node::StmtNode,
    error::{CompileErr, ErrorType},
    Keyword, Token, TokenStream, TokenType,
};

use super::{expect, expr};

pub fn parse(tokens: &mut TokenStream, current: Token) -> Result<StmtNode, ErrorType> {
    match &current.token_type {
        TokenType::Keyword(kwd) => match kwd {
            Keyword::Var => return parse_var_stmt(tokens),
            Keyword::Out => parse_out_stmt(tokens),
        },
        TokenType::EOL => {
            while let Some(next) = tokens.pop_front() {
                if next.token_type == TokenType::EOL {
                    continue;
                }
                return parse(tokens, next);
            }
            return Err(CompileErr::IllegalEOF.into());
        }
        TokenType::Unknown(..) => return Err(CompileErr::UnknownTok(Box::new(current)).into()),
        _ => return Err(CompileErr::Unexpected(Box::new(current)).into()),
    }
}

fn parse_var_stmt(tokens: &mut TokenStream) -> Result<StmtNode, ErrorType> {
    let token = expect(tokens, "idt")?;
    if let TokenType::Identif(name) = &token.token_type {
        expect(tokens, "=")?;
        if let Some(next) = tokens.pop_front() {
            let variable_val = expr::parse(tokens, &next)?;
            return Ok(StmtNode::Var(name.to_string(), Box::new(variable_val)));
        } else {
            return Err(CompileErr::IllegalEOF.into());
        }
    } else {
        return Err(CompileErr::Unexpected(Box::new(token)).into());
    }
}

fn parse_out_stmt(tokens: &mut TokenStream) -> Result<StmtNode, ErrorType> {
    if let Some(next) = tokens.pop_front() {
        let output_val = expr::parse(tokens, &next)?;
        return Ok(StmtNode::Out(Box::new(output_val)));
    } else {
        return Err(CompileErr::IllegalEOF.into());
    }
}
