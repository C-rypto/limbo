use colored::Colorize;

#[macro_export]
macro_rules! syntax_err {
    ($msg: expr) => {
        crate::common::error::syntax_err::report($msg, file!(), line!())
    };
}

pub fn report<MSG>(msg: MSG, file: &'static str, line: u32) -> !
where
    MSG: ToString,
{
    panic!(
        "{} at {}:{} {}",
        "Syntax Error".red().bold(),
        file,
        line,
        msg.to_string().white()
    )
}

pub enum SyntaxError {
    UnExpected { get: String },
    UnknownTok { what: String, line: i32 },
    IllegalEOF,
}

impl core::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnExpected { get } => write!(f, "`{}` 这个玩意不能在这出现！", get),
            Self::UnknownTok { what, line } => {
                write!(f, "在第 {} 行有个未被定义的玩意： `{}`", line, what)
            }
            Self::IllegalEOF => write!(f, "这文件怎么就没了？"),
            // Self::TooManyParen => write!(f, "Have too many parens."),
        }
    }
}
