use crate::common::{
    compile_time::ast_types::node_types::stmt_node::StmtNode,
    error::{CompileErr, ErrorType},
    Keyword, Symbol, Token, TokenStream, TokenTag, TokenType,
};

use super::{block, expr};

pub fn parse(tokens: &mut TokenStream, current: Token) -> Result<StmtNode, ErrorType> {
    match &current.token_type {
        TokenType::Keyword(kwd) => match kwd {
            Keyword::Var => return parse_var_stmt(tokens),
            Keyword::Out => return parse_out_stmt(tokens),
            // Keyword::If => return parse_if_stmt(tokens),
            // Keyword::Else => return Err(CompileErr::Unexpected(Box::new(current)).into()),
        },
        TokenType::Symbols(sym) => match sym {
            Symbol::LBrace => return parse_block_stmt(tokens),
            _ => return Err(CompileErr::Unexpected(Box::new(current)).into()),
        },
        TokenType::EOL => {
            while let Some(next) = tokens.next() {
                if next.token_type == TokenType::EOL {
                    continue;
                }
                return parse(tokens, next);
            }
            return Err(CompileErr::IllegalEOF(Box::new(current.get_end_pos())).into());
        }
        TokenType::Unknown(..) => return Err(CompileErr::UnknownTok(Box::new(current)).into()),
        _ => return Err(CompileErr::Unexpected(Box::new(current)).into()),
    }
}

fn parse_var_stmt(tokens: &mut TokenStream) -> Result<StmtNode, ErrorType> {
    let next_token = tokens.expect(TokenTag::Identif)?;
    if let TokenType::Identif(name) = &next_token.token_type {
        let current = tokens.expect(Symbol::Assign.into())?;
        match tokens.next() {
            Some(next) => {
                let value = expr::parse(tokens, &next)?;
                return Ok(StmtNode::Var {
                    name: name.to_string(),
                    value: Box::new(value),
                });
            }
            None => return Err(CompileErr::IllegalEOF(Box::new(current.get_end_pos())).into()),
        }
    } else {
        return Err(CompileErr::Unexpected(Box::new(next_token)).into());
    }
}

fn parse_out_stmt(tokens: &mut TokenStream) -> Result<StmtNode, ErrorType> {
    match tokens.next() {
        Some(next) => {
            let output_val = expr::parse(tokens, &next)?;
            return Ok(StmtNode::Out {
                value: Box::new(output_val),
            });
        }
        None => return Err(CompileErr::IllegalEOF(Box::new(tokens.prev().get_end_pos())).into()),
    }
}

fn parse_block_stmt(tokens: &mut TokenStream) -> Result<StmtNode, ErrorType> {
    let value = block::parse(tokens)?;
    return Ok(StmtNode::Block {
        value: Box::new(value),
    });
}
