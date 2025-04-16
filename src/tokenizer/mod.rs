use std::fs;

use crate::common::{error::ErrorType, utils::Source, Keyword, Symbol, Token, TokenStream, TokenType};

pub fn tokenize(path: &String) -> Result<TokenStream, ErrorType> {
    let src = fs::read_to_string(path)?;
	let mut src = Source::new(src.chars());
    // let mut chars = src.chars();

    let mut stream = TokenStream::new();

    let mut start_offset;

    loop {
        let ch = if src.cache != '\0' {
            src.cache
        } else if let Some(ch) = src.next() {
            // offset += 1;
            ch
        } else {
            break;
        };
        src.reset_cache();

        if ch.is_ascii_alphabetic() {
            start_offset = src.offset;
            let value = src.read_identi();

            if let Some(value) = Keyword::is_keyword(&value) {
                stream.push_back(Token::new(
                    TokenType::Keyword(value),
                    (path.to_string(), src.line, start_offset),
                ));
            } else {
                stream.push_back(Token::new(
                    TokenType::Identif(value),
                    (path.to_string(), src.line, start_offset),
                ));
            }
            continue;
        } else if ch.is_ascii_digit() {
            start_offset = src.offset;
            let value = src.read_number();

            stream.push_back(Token::new(
                TokenType::Literal(value),
                (path.to_string(), src.line, start_offset),
            ));
            continue;
        }

        match ch {
            '\'' | '\"' => {
                start_offset = src.offset;
                let value = src.read_string();

                stream.push_back(Token::new(
                    TokenType::Literal(value),
                    (path.to_string(), src.line, start_offset),
                ));
            }
            ' ' | '\r' | '\t' => continue,
            '\n' => {
                stream.push_back(Token::new(TokenType::EOL, (path.to_string(), src.line, src.offset)));

                src.line += 1;
                src.offset = 0;
            }
            _ => {
                if let Some(symbol) = Symbol::is_symbol(ch) {
                    stream.push_back(Token::new(
                        TokenType::Symbols(symbol),
                        (path.to_string(), src.line, src.offset),
                    ));
                } else {
                    start_offset = src.offset;
                    let value = src.read_unknow();

                    stream.push_back(Token::new(
                        TokenType::Unknown(value),
                        (path.to_string(), src.line, start_offset),
                    ));
                }
            }
        }
    }

    return Ok(stream);
}
