use crate::common::{
    compile_time::ast_types::node_types::AssignNode,
    error::{CompileErr, ErrorType},
    Symbol, Token, TokenStream, TokenType,
};

use super::expr;

pub fn parse(tokens: &mut TokenStream, current: Token) -> Result<AssignNode, ErrorType> {
    if let TokenType::Identif(name) = &current.token_type {
        let current = tokens.expect(Symbol::Assign.into())?;
        match tokens.next() {
            Some(next) => {
                let value = Box::new(expr::parse(tokens, next)?);

                tokens.skip_white_space();
                return Ok(AssignNode {
                    name: name.to_string(),
                    value,
                    pos: current.pos,
                });
            }
            None => return Err(CompileErr::IllegalEOF(Box::new(current.get_end_pos())).into()),
        }
    } else {
        return Err(CompileErr::Unexpected(Box::new(current)).into());
    }
}
