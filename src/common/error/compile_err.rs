use crate::common::Token;

pub enum CompileErr {
    IllegalEOF,
    UnknownTok(Token),
    Unexpected(String),
}

impl core::fmt::Display for CompileErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IllegalEOF => write!(f, "文件怎么就结束了？"),
            Self::UnknownTok(token) => write!(
                f,
                "位于 `{}` 的 `{}` 是个啥玩意？",
                format!("{}:{}:{}", token.pos.0, token.pos.1, token.pos.2),
                token
            ),
            Self::Unexpected(name) => write!(f, "`{}` 这可不是我想要的", name),
        }
    }
}
