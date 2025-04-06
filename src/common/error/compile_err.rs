use super::Locatable;

pub enum CompileErr {
    IllegalEOF,
    BasicError(Box<dyn std::error::Error>),
    UnknownTok(Box<dyn Locatable>),
    Unexpected(Box<dyn Locatable>),
}

impl core::fmt::Display for CompileErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IllegalEOF => write!(f, "文件怎么就结束了？"),
            Self::BasicError(err) => write!(f, "{}", err),
            Self::UnknownTok(sth) => write!(
                f,
                "\n  pos: {}\n  msg: `{}` is a unknown token.",
                sth.get_pos(),
                sth.to_string()
            ),
            Self::Unexpected(sth) => write!(
                f,
                "\n  pos: {}\n  msg: `{}` is a unexpected token.",
                sth.get_pos(),
                sth.to_string()
            ),
        }
    }
}

impl From<std::io::Error> for CompileErr {
    fn from(value: std::io::Error) -> Self {
        Self::BasicError(Box::new(value))
    }
}
