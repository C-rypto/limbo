use colored::Colorize;

pub fn report<MSG>(msg: MSG) -> !
where
    MSG: ToString,
{
    panic!(
        "{} {}",
        "Syntax-Error".red().bold(),
        msg.to_string().white()
    )
}

pub enum ErrorType {
    NotExpected { need: String, get: Option<String> },
    TooManyParen,
}

pub fn not_expected<TS>(need: TS, get: Option<TS>) -> ErrorType
where
    TS: ToString,
{
    match get {
        Some(get) => ErrorType::NotExpected {
            need: need.to_string(),
            get: Some(get.to_string()),
        },
        None => ErrorType::NotExpected {
            need: need.to_string(),
            get: None,
        },
    }
}

impl core::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotExpected { need, get } => match get {
                Some(next) => write!(f, "Expected `{}` but get `{}`.", need, next),
                None => write!(f, "Expected `{}` but get Nothing.", need),
            },
            Self::TooManyParen => write!(f, "Have too many parens."),
        }
    }
}
