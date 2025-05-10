use crate::common::{
    compile_time::ast_types::node_types::stmt_node::StmtNode,
    error::{CompileErr, ErrorType},
    Keyword, Symbol, Token, TokenStream, TokenType,
};

use super::{assign, block, expr};

pub fn parse(tokens: &mut TokenStream, current: Token) -> Result<StmtNode, ErrorType> {
    match &current.token_type {
        TokenType::Keyword(kwd) => match kwd {
            Keyword::Var => match tokens.next() {
                Some(next) => {
                    return Ok(StmtNode::Var {
                        inner: Box::new(assign::parse(tokens, next)?),
                    })
                }
                None => {
                    return Err(
                        CompileErr::IllegalEOF(Box::new(tokens.prev().get_end_pos())).into(),
                    )
                }
            },
            Keyword::Out => return parse_out_stmt(tokens),
            Keyword::If => return parse_if_stmt(tokens),
            Keyword::Else => unreachable!(), //return Err(CompileErr::Unexpected(Box::new(current)).into()),
        },
        TokenType::Symbols(sym) => match sym {
            Symbol::LBrace => return parse_block_stmt(tokens),
            _ => return Err(CompileErr::Unexpected(Box::new(current)).into()),
        },
        TokenType::Identif(..) => {
            return Ok(StmtNode::Assign {
                inner: Box::new(assign::parse(tokens, current)?),
            })
        }
        TokenType::Unknown(..) => return Err(CompileErr::UnknownTok(Box::new(current)).into()),
        _ => unreachable!(), //return Err(CompileErr::Unexpected(Box::new(current)).into()),
    }
}

fn parse_out_stmt(tokens: &mut TokenStream) -> Result<StmtNode, ErrorType> {
    match tokens.next() {
        Some(next) => {
            let value = Box::new(expr::parse(tokens, next)?);

            tokens.skip_white_space();
            return Ok(StmtNode::Out { inner: value });
        }
        None => return Err(CompileErr::IllegalEOF(Box::new(tokens.prev().get_end_pos())).into()),
    }
}

fn parse_block_stmt(tokens: &mut TokenStream) -> Result<StmtNode, ErrorType> {
    let value = Box::new(block::parse(tokens)?);

    tokens.skip_white_space();
    return Ok(StmtNode::Block { inner: value });
}

fn parse_if_stmt(tokens: &mut TokenStream) -> Result<StmtNode, ErrorType> {
    match tokens.next() {
        Some(next) => {
            let cond = Box::new(expr::parse(tokens, next)?);
            match tokens.next() {
                Some(after_cond) => {
                    let if_stmt = Box::new(parse(tokens, after_cond)?);
                    let else_stmt = parse_if_rest(tokens)?;

                    tokens.skip_white_space();
                    return Ok(StmtNode::IfElse {
                        cond,
                        if_stmt,
                        else_stmt,
                    });
                }
                None => {
                    return Err(
                        CompileErr::IllegalEOF(Box::new(tokens.prev().get_end_pos())).into(),
                    )
                }
            }
        }
        None => return Err(CompileErr::IllegalEOF(Box::new(tokens.prev().get_end_pos())).into()),
    }
}

fn parse_if_rest(tokens: &mut TokenStream) -> Result<Option<Box<StmtNode>>, ErrorType> {
    tokens.skip_white_space();
    match tokens.next() {
        Some(next) => {
            if let TokenType::Keyword(Keyword::Else) = next.token_type {
                tokens.skip_white_space();
                match tokens.next() {
                    Some(after_else) => {
                        let else_stmt = Some(Box::new(parse(tokens, after_else)?));
                        return Ok(else_stmt);
                    }
                    None => {
                        return Err(
                            CompileErr::IllegalEOF(Box::new(tokens.prev().get_end_pos())).into(),
                        )
                    }
                }
            } else {
                tokens.undo();
                return Ok(None);
            }
        }
        None => return Ok(None),
    }
}
