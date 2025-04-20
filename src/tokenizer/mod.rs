use std::fs;

use crate::common::{
    error::ErrorType, utils::Source, values::Value, Keyword, Symbol, Token, TokenStream, TokenType,
};

pub fn tokenize(path: &String) -> Result<TokenStream, ErrorType> {
    let src = fs::read_to_string(path)?;
    let mut src = Source::new(path, src.chars());
    // let mut chars = src.chars();

    let mut stream = TokenStream::new();

    let mut start_offset;

    loop {
        let ch = if let Some(ch) = src.next() {
            ch
        } else {
            break;
        };
        // src.reset_cache();

        if ch.is_ascii_alphabetic() {
            start_offset = src.offset;
            let word = src.read_identi();

            if let Some(value) = Value::try_boolean(&word) {
                stream.push(Token::new(
                    TokenType::Literal(value),
                    (src.path(), src.line, start_offset).into(),
                ));
            } else if let Some(keyword) = Keyword::is_keyword(&word) {
                stream.push(Token::new(
                    TokenType::Keyword(keyword),
                    (src.path(), src.line, start_offset).into(),
                ));
            } else {
                stream.push(Token::new(
                    TokenType::Identif(word),
                    (src.path(), src.line, start_offset).into(),
                ));
            }
            continue;
        } else if ch.is_ascii_digit() {
            start_offset = src.offset;
            let value = src.read_number();

            stream.push(Token::new(
                TokenType::Literal(value),
                (src.path(), src.line, start_offset).into(),
            ));
            continue;
        }

        match ch {
            '\'' | '\"' => {
                start_offset = src.offset;
                let value = src.read_string()?;

                stream.push(Token::new(
                    TokenType::Literal(value),
                    (src.path(), src.line, start_offset).into(),
                ));
            }
            ' ' | '\r' | '\t' => {
                stream.push(Token::new(
                    TokenType::WhiteSpace(1),
                    (src.path(), src.line, src.offset).into(),
                ));
            }
            '\n' => {
                stream.push(Token::new(
                    TokenType::EOL,
                    (src.path(), src.line, src.offset).into(),
                ));

                src.line += 1;
                src.offset = 0;
            }
            _ => {
                start_offset = src.offset;
                if let Some(next) = src.next() {
                    let word = format!("{}{}", ch, next);
                    match Symbol::try_symbol(word) {
                        Some(symbol) => {
                            stream.push(Token::new(
                                TokenType::Symbols(symbol),
                                (src.path(), src.line, start_offset).into(),
                            ));
                            continue;
                        }
                        None => src.undo(),
                    }
                }
                if let Some(symbol) = Symbol::try_symbol(ch) {
                    stream.push(Token::new(
                        TokenType::Symbols(symbol),
                        (src.path(), src.line, start_offset).into(),
                    ));
                } else {
                    start_offset = src.offset;
                    let value = src.read_unknow();

                    stream.push(Token::new(
                        TokenType::Unknown(value),
                        (src.path(), src.line, start_offset).into(),
                    ));
                }
            }
        }
    }

    return Ok(stream);
}
