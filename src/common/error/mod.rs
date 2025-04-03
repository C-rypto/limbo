use runtime_err::RuntimeError;
use syntax_err::SyntaxError;

use crate::common::Token;

pub mod runtime_err;
pub mod syntax_err;

pub enum ErrorType {
    Syntax(SyntaxError),
    Runtime(RuntimeError),
}

impl core::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Syntax(err) => write!(f, "{}", err),
            Self::Runtime(err) => write!(f, "{}", err),
        }
    }
}

pub fn unexpected<TS>(get: TS) -> ErrorType
where
    TS: ToString,
{
    return ErrorType::Syntax(SyntaxError::UnExpected {
        get: get.to_string(),
    });
}

pub fn unknown_tok(token: Token) -> ErrorType {
    if let Token::Unknown(what, line) = token {
        return ErrorType::Syntax(SyntaxError::UnknownTok { what, line });
    }
    unreachable!()
}

pub fn illegal_eof() -> ErrorType {
    return ErrorType::Syntax(SyntaxError::IllegalEOF);
}

pub fn type_error() -> ErrorType {
    return ErrorType::Runtime(RuntimeError::TypeError);
}

pub fn undeclared<TS>(name: TS) -> ErrorType
where
    TS: ToString,
{
    return ErrorType::Runtime(RuntimeError::UndeclaredIdt(name.to_string()));
}
