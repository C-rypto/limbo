#[derive(Clone, PartialEq)]
pub enum Symbol {
    Add,
    Sub,
    Mul,
    Div,

    Equal,

    LParen,
    RParen,
}

impl Symbol {
    pub fn is_symbol(ch: char) -> Option<Self> {
        for pair in SYMBOL_PAIR.iter() {
            if ch == pair.0 {
                return Some(pair.1.clone());
            }
        }
        return None;
    }
}

pub static SYMBOL_PAIR: [(char, Symbol); 7] = [
    ('+', Symbol::Add),
    ('-', Symbol::Sub),
    ('*', Symbol::Mul),
    ('/', Symbol::Div),
    ('=', Symbol::Equal),
    ('(', Symbol::LParen),
    (')', Symbol::RParen),
];

impl core::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "{}", "+"),
            Self::Sub => write!(f, "{}", "-"),
            Self::Mul => write!(f, "{}", "*"),
            Self::Div => write!(f, "{}", "/"),

            Self::Equal => write!(f, "{}", "="),

            Self::LParen => write!(f, "{}", "("),
            Self::RParen => write!(f, "{}", ")"),
        }
    }
}
