mod keyword;
mod symbols;

use {
    super::{error::syntax_err, values::Value, Stream},
    colored::Colorize,
    std::collections::VecDeque,
};
pub use {keyword::Keyword, symbols::Symbol};

#[derive(Clone, PartialEq)]
pub enum Token {
    Unknown(String, i32),

    Symbols(Symbol),
    Literal(Value),
    Identif(String),
    Keyword(Keyword),
}

impl Token {
    pub fn get_mark(&self) -> String {
        match self {
            Self::Unknown(..) => "unk".to_string(),
            Self::Symbols(sym) => sym.to_string(),
            Self::Literal(..) => "lit".to_string(),
            Self::Identif(..) => "idt".to_string(),
            Self::Keyword(..) => "kwd".to_string(),
        }
    }
}

pub type TokenStream = VecDeque<Token>;
impl Stream for TokenStream {
    fn match_next(&mut self, mark: &'static str) {
        match self.pop_front() {
            Some(token) => {
                let self_mark = token.get_mark();
                if mark != self_mark {
                    syntax_err::report(syntax_err::not_expected(mark, Some(&self_mark)))
                }
            }
            None => syntax_err::report(syntax_err::not_expected(mark, None)),
        }
    }
}

impl core::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown(lexeme, line) => write!(
                f,
                "\n\tPanic at line {}: `{}`\n",
                line,
                lexeme.red().underline()
            ),
            Self::Symbols(symbols) => write!(f, "{}", symbols),
            Self::Literal(literal) => write!(f, "{}", literal),
            Self::Identif(identif) => write!(f, "{}", identif.cyan().bold()),
            Self::Keyword(keyword) => write!(f, "{}", keyword),
        }
    }
}
