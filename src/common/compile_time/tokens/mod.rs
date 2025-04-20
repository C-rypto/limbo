mod keyword;
mod symbols;

mod token_stream;
mod token_tag;

use {
    crate::common::{
        utils::{Locatable, Location},
        values::Value,
    },
    colored::Colorize,
};
pub use {keyword::Keyword, symbols::Symbol, token_stream::TokenStream, token_tag::TokenTag};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub pos: Location,
}

impl Token {
    pub fn new(token_type: TokenType, pos: Location) -> Token {
        return Token { token_type, pos };
    }

    pub fn get_tag(&self) -> TokenTag {
        return self.token_type.get_tag();
    }

    pub fn get_file(&self) -> String {
        return self.pos.0.clone();
    }

    pub fn get_line(&self) -> u32 {
        return self.pos.1;
    }

    pub fn get_offset(&self) -> u32 {
        return self.pos.2;
    }

    pub fn get_end_pos(&self) -> Location {
        return Location::from((
            self.get_file(),
            self.get_line(),
            self.get_offset() + (self.to_string().len() as u32),
        ));
    }
}

impl Locatable for Token {
    fn locate(&self) -> String {
        return format!("{}", self.pos);
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum TokenType {
    EOL,
    WhiteSpace(usize),

    Unknown(String),

    Symbols(Symbol),
    Literal(Value),
    Identif(String),
    Keyword(Keyword),
}

impl TokenType {
    pub fn get_tag(&self) -> TokenTag {
        TokenTag::new(self)
    }
}

impl core::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return self.token_type.fmt(f);
    }
}

impl core::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EOL => write!(f, "\\n"),
            Self::WhiteSpace(num) => write!(f, "{}", " ".repeat(*num)),

            Self::Unknown(lexeme) => write!(f, "{}", lexeme.red().underline()),
            Self::Symbols(symbols) => write!(f, "{}", symbols),
            Self::Literal(literal) => write!(f, "{}", literal),
            Self::Identif(identif) => write!(f, "{}", identif.cyan().bold()),
            Self::Keyword(keyword) => write!(f, "{}", keyword),
        }
    }
}
