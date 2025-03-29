use crate::common::{error::syntax_err, Token, TokenStream};

pub mod atom;
pub mod expr;
pub mod stmt;
pub mod term;

pub fn expect(tokens: &mut TokenStream, target: &'static str) -> Token {
    if let Some(next) = tokens.pop_front() {
        if next.get_mark() != target.to_string() {
            syntax_err::report(syntax_err::unexpected(next), file!(), line!());
        }
        return next;
    } else {
        syntax_err::report(syntax_err::illegal_eof(), file!(), line!());
    }
}
