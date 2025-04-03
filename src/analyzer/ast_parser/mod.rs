use crate::common::{error, Token, TokenStream};
use crate::syntax_err;

pub mod atom;
pub mod expr;
pub mod stmt;
pub mod term;

pub fn expect(tokens: &mut TokenStream, target: &'static str) -> Token {
    if let Some(next) = tokens.pop_front() {
        if next.get_mark() != target.to_string() {
            syntax_err!(error::unexpected(next))
        }
        return next;
    } else {
        syntax_err!(error::illegal_eof())
    }
}
