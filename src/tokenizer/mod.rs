use std::fs;

use read_values::{read_identi, read_number, read_string, read_unknow};

use crate::common::{error::ErrorType, Keyword, Symbol, Token, TokenStream, TokenType};

mod read_values;

pub fn tokenize(path: &String) -> Result<TokenStream, ErrorType> {
    let src = fs::read_to_string(path)?;

    let mut stream = TokenStream::new();

    let mut chars = src.chars();
    let mut cache = '\0';

    let mut line = 1_u32;
    let mut offset = 0_u32;
    let mut start_offset;

    loop {
        let ch = if cache != '\0' {
            cache
        } else if let Some(ch) = chars.next() {
            offset += 1;
            ch
        } else {
            break;
        };
        cache = '\0';

        if ch.is_ascii_alphabetic() {
            let value;
            start_offset = offset;
            (cache, value, offset) = read_identi(&mut chars, ch, offset);

            if let Some(value) = Keyword::is_keyword(&value) {
                stream.push_back(Token::new(
                    TokenType::Keyword(value),
                    (path.to_string(), line, start_offset),
                ));
            } else {
                stream.push_back(Token::new(
                    TokenType::Identif(value),
                    (path.to_string(), line, start_offset),
                ));
            }
            continue;
        } else if ch.is_ascii_digit() {
            let value;
            start_offset = offset;
            (cache, value, offset) = read_number(&mut chars, ch, offset);

            stream.push_back(Token::new(
                TokenType::Literal(value),
                (path.to_string(), line, start_offset),
            ));
            continue;
        }

        match ch {
            '\'' | '\"' => {
                let value;
                start_offset = offset;
                (cache, value, line, offset) = read_string(&mut chars, line, offset);

                stream.push_back(Token::new(
                    TokenType::Literal(value),
                    (path.to_string(), line, start_offset),
                ));
            }
            ' ' | '\r' | '\t' => continue,
            '\n' => {
                stream.push_back(Token::new(TokenType::EOL, (path.to_string(), line, offset)));

                line += 1;
                offset = 0;
            }
            _ => {
                if let Some(symbol) = Symbol::is_symbol(ch) {
                    stream.push_back(Token::new(
                        TokenType::Symbols(symbol),
                        (path.to_string(), line, offset),
                    ));
                } else {
                    let value;
                    start_offset = offset;
                    (cache, value, offset) = read_unknow(&mut chars, ch, offset);

                    stream.push_back(Token::new(
                        TokenType::Unknown(value),
                        (path.to_string(), line, start_offset),
                    ));
                }
            }
        }
    }

    return Ok(stream);
}
