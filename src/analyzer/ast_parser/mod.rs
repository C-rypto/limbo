use crate::common::{
    error::{CompileErr, ErrorType},
    Token, TokenStream,
};

pub mod atom;
pub mod expr;
pub mod stmt;
pub mod term;

pub fn expect(tokens: &mut TokenStream, target: &'static str) -> Result<Token, ErrorType> {
    if let Some(next) = tokens.pop_front() {
        if next.get_mark() != target.to_string() {
            return Err(CompileErr::Unexpected(Box::new(next)).into());
        } else if next.get_mark() == "unk" {
            return Err(CompileErr::UnknownTok(Box::new(next)).into());
        }
        return Ok(next);
    } else {
        return Err(CompileErr::IllegalEOF.into());
    }
}
