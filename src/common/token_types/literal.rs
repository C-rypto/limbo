use colored::Colorize;

#[derive(Clone)]
pub enum Literal {
    Number(f64),
    String(String),
}

impl core::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(number) => write!(
                f,
                "{}",
                number.to_string().custom_color((181, 206, 168)).bold()
            ),
            Self::String(string) => write!(f, "\"{}\"", string.green().bold()),
        }
    }
}
