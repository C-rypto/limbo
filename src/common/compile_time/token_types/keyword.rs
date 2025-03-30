use colored::Colorize;

#[derive(Clone, PartialEq)]
pub enum Keyword {
    Var,
    Out,
}

pub static KEYWORD_PAIR: [(&'static str, Keyword); 2] =
    [("var", Keyword::Var), ("out", Keyword::Out)];

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

impl core::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var => write!(f, "{}", "var".custom_color((216, 160, 223))),
            Self::Out => write!(f, "{}", "out".custom_color((216, 160, 223))),
        }
    }
}
