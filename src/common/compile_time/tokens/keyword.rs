use colored::Colorize;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum Keyword {
    Var,
    Out,
    If,
    Else,
}

pub static KEYWORD_PAIR: [(&'static str, Keyword); 4] = [
    ("var", Keyword::Var),
    ("out", Keyword::Out),
    ("if", Keyword::If),
    ("else", Keyword::Else),
];

impl Keyword {
    pub fn is_keyword(word: &String) -> Option<Keyword> {
        for keyword in KEYWORD_PAIR.iter() {
            if word == keyword.0 {
                return Some(keyword.1.clone());
            }
        }
        return None;
    }
}

static KEYWORD_COLOR: (u8, u8, u8) = (216, 160, 223);

impl core::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var => write!(f, "{}", "var".custom_color(KEYWORD_COLOR)),
            Self::Out => write!(f, "{}", "out".custom_color(KEYWORD_COLOR)),
            Self::If => write!(f, "{}", "if".custom_color(KEYWORD_COLOR)),
            Self::Else => write!(f, "{}", "else".custom_color(KEYWORD_COLOR)),
        }
    }
}
