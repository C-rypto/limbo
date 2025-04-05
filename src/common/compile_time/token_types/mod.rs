mod keyword;
mod symbols;

use {crate::common::values::Value, colored::Colorize, std::collections::VecDeque};
pub use {keyword::Keyword, symbols::Symbol};

pub type TokenPos = (String, u32, u32);

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub pos: TokenPos,
}

impl Token {
    pub fn new(token_type: TokenType, pos: TokenPos) -> Token {
        return Token { token_type, pos };
    }

    pub fn get_mark(&self) -> String {
        return self.token_type.get_mark();
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum TokenType {
    EOL,

    Unknown(String),

    Symbols(Symbol),
    Literal(Value),
    Identif(String),
    Keyword(Keyword),
}

impl TokenType {
    pub fn get_mark(&self) -> String {
        match self {
            Self::EOL => "eol".to_string(),
            Self::Unknown(..) => "unk".to_string(),
            Self::Symbols(sym) => sym.to_string(),
            Self::Literal(..) => "lit".to_string(),
            Self::Identif(..) => "idt".to_string(),
            Self::Keyword(..) => "kwd".to_string(),
        }
    }
}

pub type TokenStream = VecDeque<Token>;

impl core::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_type)
    }
}

impl core::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EOL => write!(f, "\\n"),

            Self::Unknown(lexeme) => write!(f, "{}", lexeme.red().underline()),
            Self::Symbols(symbols) => write!(f, "{}", symbols),
            Self::Literal(literal) => write!(f, "{}", literal),
            Self::Identif(identif) => write!(f, "{}", identif.cyan().bold()),
            Self::Keyword(keyword) => write!(f, "{}", keyword),
        }
    }
}
