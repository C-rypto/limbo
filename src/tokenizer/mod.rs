use read_values::{read_identi, read_number, read_string, read_unknow};

use crate::common::{Keyword, Symbol, Token, TokenStream};

mod read_values;

pub fn tokenize(src: &String) -> TokenStream {
    let mut stream = TokenStream::new();

    let mut chars = src.chars();
    let mut line = 1;
    let mut cache = '\0';

    loop {
        let ch = if cache != '\0' {
            cache
        } else if let Some(ch) = chars.next() {
            ch
        } else {
            break;
        };
        cache = '\0';

        if ch.is_ascii_alphabetic() {
            let value;
            (cache, value, line) = read_identi(&mut chars, ch, line);

            if let Some(value) = Keyword::is_keyword(&value) {
                stream.push_back(Token::Keyword(value));
            } else {
                stream.push_back(Token::Identif(value));
            }
            continue;
        } else if ch.is_ascii_digit() {
            let value;
            (cache, value, line) = read_number(&mut chars, ch, line);

            stream.push_back(Token::Literal(value));
            continue;
        }

        match ch {
            '\'' | '\"' => {
                let value;
                (cache, value, line) = read_string(&mut chars, line);

                stream.push_back(Token::Literal(value));
            }
            ' ' | '\r' | '\t' => continue,
            '\n' => line += 1,
            _ => {
                if let Some(symbol) = Symbol::is_symbol(ch) {
                    stream.push_back(Token::Symbols(symbol));
                } else {
                    let value;
                    (cache, value, line) = read_unknow(&mut chars, ch, line);

                    stream.push_back(Token::Unknown(value, line));
                }
            }
        }
    }

    return stream;
}
