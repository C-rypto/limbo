use std::str::Chars;

use crate::common::values::Value;

pub fn is_identi(ch: char) -> bool {
    return ch.is_ascii_alphanumeric() || ch == '_';
}

// ----------

pub fn read_unknow(chars: &mut Chars, first: char, line: i32) -> (char, String, i32) {
    let mut value = String::from(first);

    while let Some(ch) = chars.next() {
        if ch.is_whitespace() {
            break;
        }
        value.push(ch);
    }

    return ('\0', value, line);
}

pub fn read_string(chars: &mut Chars, line: i32) -> (char, Value, i32) {
    fn convert(chars: &mut Chars) -> char {
        match chars.next() {
            Some(ch) => match ch {
                't' => '\t',
                'n' => '\n',
                'r' => '\r',
                '\\' => '\\',
                _ => ch,
            },
            None => return '\0',
        }
    }

    let mut value = String::new();
    let mut line = line;

    while let Some(ch) = chars.next() {
        if ch == '\'' || ch == '\"' {
            break;
        } else if ch == '\\' {
            value.push(convert(chars));
        } else if ch == '\n' {
            line += 1;
        } else {
            value.push(ch);
        }
    }

    return ('\0', Value::String(value), line);
}

pub fn read_number(chars: &mut Chars, first: char, line: i32) -> (char, Value, i32) {
    fn convert(ch: char) -> f64 {
        const START_POINT: u8 = 48;
        return ((ch as u8) - START_POINT) as f64;
    }

    let mut value = convert(first);
    let mut cache: char = '\0';
    let mut is_float = false;

    while let Some(ch) = chars.next() {
        if ch.is_ascii_digit() {
            let num = convert(ch);
            if is_float {
                value = value + (num / 10.0);
            } else {
                value = value * 10.0 + num;
            }
        } else if ch == '.' {
            is_float = true;
            continue;
        } else {
            cache = ch;
            break;
        }
    }

    return (cache, Value::Number(value), line);
}

pub fn read_identi(chars: &mut Chars, first: char, line: i32) -> (char, String, i32) {
    let mut value = String::from(first);
    let mut cache = '\0';
    let mut line = line;

    while let Some(ch) = chars.next() {
        if is_identi(ch) {
            value.push(ch);
        } else {
            cache = ch;
            break;
        }
    }

    if cache == '\n' {
        line += 1;
    }

    return (cache, value, line);
}
