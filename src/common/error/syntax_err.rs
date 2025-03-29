use colored::Colorize;

use crate::common::Token;

pub fn report<MSG>(msg: MSG, file: &'static str, line: u32) -> !
where
    MSG: ToString,
{
    panic!(
        "{} at {}:{} {}",
        "Syntax-Error".red().bold(),
        file,
        line,
        msg.to_string().white()
    )
}

pub enum ErrorType {
    UnExpected { get: String },
    UnknownTok { what: String, line: i32 },
    IllegalEOF,
}

pub fn unexpected<TS>(get: TS) -> ErrorType
where
    TS: ToString,
{
    return ErrorType::UnExpected {
        get: get.to_string(),
    };
}

pub fn unknown_tok(token: Token) -> ErrorType {
    if let Token::Unknown(what, line) = token {
        return ErrorType::UnknownTok { what, line };
    }
    unreachable!()
}

pub fn illegal_eof() -> ErrorType {
    ErrorType::IllegalEOF
}

impl core::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnExpected { get } => write!(f, "`{}` is not the expected token.", get),
            Self::UnknownTok { what, line } => {
                write!(f, "Unknown token `{}` at line {}", what, line)
            }
            Self::IllegalEOF => write!(f, "Illegal EOF!"),
            // Self::TooManyParen => write!(f, "Have too many parens."),
        }
    }
}
