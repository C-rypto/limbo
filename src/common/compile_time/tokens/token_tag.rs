use super::{Symbol, TokenType};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum TokenTag {
    EOL,
    WhiteSpace,

    Unknown,

    Symbols(Symbol),
    Literal,
    Identif,
    Keyword,
}

impl From<Symbol> for TokenTag {
    fn from(value: Symbol) -> Self {
        Self::Symbols(value)
    }
}

impl TokenTag {
    pub fn new(token: &TokenType) -> Self {
        match token {
            TokenType::EOL => Self::EOL,
            TokenType::WhiteSpace(..) => Self::WhiteSpace,
            TokenType::Unknown(..) => Self::Unknown,
            TokenType::Symbols(symbol) => Self::Symbols(symbol.clone()),
            TokenType::Literal(..) => Self::Literal,
            TokenType::Identif(..) => Self::Identif,
            TokenType::Keyword(..) => Self::Keyword,
        }
    }
}
