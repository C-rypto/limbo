use colored::Colorize;

#[derive(Clone)]
pub enum Symbol {
    Add,
    Sub,
    Mul,
    Div,

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

pub static SYMBOL_PAIR: [(char, Symbol); 6] = [
    ('+', Symbol::Add),
    ('-', Symbol::Sub),
    ('*', Symbol::Mul),
    ('/', Symbol::Div),
    ('(', Symbol::LParen),
    (')', Symbol::RParen),
];

impl core::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "{}", "+".white()),
            Self::Sub => write!(f, "{}", "-".white()),
            Self::Mul => write!(f, "{}", "*".white()),
            Self::Div => write!(f, "{}", "/".white()),

            Self::LParen => write!(f, "{}", "(".white()),
            Self::RParen => write!(f, "{}", ")".white()),
        }
    }
}
