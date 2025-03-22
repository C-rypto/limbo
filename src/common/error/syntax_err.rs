use colored::Colorize;

use crate::common::Token;

pub fn report<MSG>(msg: MSG) -> !
where
    MSG: ToString,
{
    panic!(
        "{} {}",
        "Syntax-Error".red().bold(),
        msg.to_string().white()
    )
}

pub enum ErrorType {
    NotExpected { need: String, get: Option<String> },
    UnknownToken { what: String, line: i32 },
}

pub fn not_expected<TS>(need: TS, get: Option<TS>) -> ErrorType
where
    TS: ToString,
{
    match get {
        Some(get) => ErrorType::NotExpected {
            need: need.to_string(),
            get: Some(get.to_string()),
        },
        None => ErrorType::NotExpected {
            need: need.to_string(),
            get: None,
        },
    }
}

pub fn unknown_token(token: Token) -> ErrorType {
    if let Token::Unknown(what, line) = token {
        return ErrorType::UnknownToken { what, line };
    }
    unreachable!()
}

impl core::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotExpected { need, get } => match get {
                Some(next) => write!(f, "Expected `{}` but get `{}`.", need, next),
                None => write!(f, "Expected `{}` but get Nothing.", need),
            },
            Self::UnknownToken { what, line } => {
                write!(f, "Unknown token `{}` at line {}", what, line)
            }
            // Self::TooManyParen => write!(f, "Have too many parens."),
        }
    }
}
