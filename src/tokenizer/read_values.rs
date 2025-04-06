use std::str::Chars;

use crate::common::values::Value;

pub fn is_identi(ch: char) -> bool {
    return ch.is_ascii_alphanumeric() || ch == '_';
}

// ----------

pub fn read_unknow(chars: &mut Chars, first: char, offset: u32) -> (char, String, u32) {
    let mut value = String::from(first);
    let mut offset = offset;

    while let Some(ch) = chars.next() {
        offset += 1;
        if ch.is_whitespace() {
            break;
        }
        value.push(ch);
    }

    return ('\0', value, offset);
}

pub fn read_string(chars: &mut Chars, line: u32, offset: u32) -> (char, Value, u32, u32) {
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
    let mut offset = offset;

    while let Some(ch) = chars.next() {
        offset += 1;
        if ch == '\'' || ch == '\"' {
            break;
        } else if ch == '\\' {
            value.push(convert(chars));
        } else if ch == '\n' {
            line += 1;
            offset = 0;
            value.push(ch);
        } else {
            value.push(ch);
        }
    }

    return ('\0', Value::String(value), line, offset);
}

pub fn read_number(chars: &mut Chars, first: char, offset: u32) -> (char, Value, u32) {
    fn convert(ch: char) -> f64 {
        const START_POINT: u8 = 48;
        return ((ch as u8) - START_POINT) as f64;
    }

    let mut value = convert(first);
    let mut cache: char = '\0';
    let mut is_float = false;

    let mut offset = offset;

    while let Some(ch) = chars.next() {
        offset += 1;
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

    return (cache, Value::Number(value), offset);
}

pub fn read_identi(chars: &mut Chars, first: char, offset: u32) -> (char, String, u32) {
    let mut value = String::from(first);
    let mut cache = '\0';
    let mut offset = offset;

    while let Some(ch) = chars.next() {
        offset += 1;
        if is_identi(ch) {
            value.push(ch);
        } else {
            cache = ch;
            break;
        }
    }

    return (cache, value, offset);
}
