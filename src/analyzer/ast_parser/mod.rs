use crate::{
    common::{error::CompileErr, Token, TokenStream},
    err_report,
};

pub mod atom;
pub mod expr;
pub mod stmt;
pub mod term;

pub fn expect(tokens: &mut TokenStream, target: &'static str) -> Token {
    if let Some(next) = tokens.pop_front() {
        if next.get_mark() != target.to_string() {
            err_report!(CompileErr::Unexpected(next.to_string()).into())
        }
		else if next.get_mark() == "unk" {
			err_report!(CompileErr::UnknownTok(next).into())
		}
        return next;
    } else {
        err_report!(CompileErr::IllegalEOF.into())
    }
}
