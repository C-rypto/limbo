use crate::common::utils::{indent, Locatable};

pub enum CompileErr {
    BasicError(Box<dyn std::error::Error>),
    IllegalEOF(Box<dyn Locatable>),
    FloatError(Box<dyn Locatable>),
    UnknownTok(Box<dyn Locatable>),
    Unexpected(Box<dyn Locatable>),

    MissComma(Box<dyn Locatable>),
    MissSemicolon(Box<dyn Locatable>),
}

impl core::fmt::Display for CompileErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IllegalEOF(sth) => write!(
                f,
                "`{}`\n{}here should not be the end of file.",
                sth.locate(),
                indent(1),
            ),
            Self::BasicError(err) => write!(f, "{}", err),
            Self::FloatError(sth) => write!(
                f,
                "`{}`\n{}here is an extra decimal point.",
                sth.locate(),
                indent(1)
            ),
            Self::UnknownTok(sth) => write!(
                f,
                "`{}`\n{}`{}` is a unknown token.",
                sth.locate(),
                indent(1),
                sth.to_string()
            ),
            Self::Unexpected(sth) => write!(
                f,
                "`{}`\n{}`{}` is a unexpected token.",
                sth.locate(),
                indent(1),
                sth.to_string()
            ),
            Self::MissComma(sth) => write!(
                f,
                "`{}`\n{}here need a '\"' symbol to terminate the string.",
                sth.locate(),
                indent(1),
            ),
            Self::MissSemicolon(sth) => write!(
                f,
                "`{}`\n{}here need a ';' symbol to terminate the if-else statement.",
                sth.locate(),
                indent(1),
            ),
        }
    }
}

impl From<std::io::Error> for CompileErr {
    fn from(value: std::io::Error) -> Self {
        Self::BasicError(Box::new(value))
    }
}
