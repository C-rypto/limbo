use std::str::Chars;

use crate::{
    common::{
        error::{CompileErr, ErrorType},
        utils::Location,
        values::Value,
    },
    err_report,
};

pub fn is_identi(ch: char) -> bool {
    return ch.is_ascii_alphanumeric() || ch == '_';
}

pub struct Source<'a> {
    path: String,
    chars: Chars<'a>,
    prev: char,

    pub cache: char,
    pub line: u32,
    pub offset: u32,
}

impl Source<'_> {
    pub fn new<T>(path: T, chars: Chars) -> Source
    where
        T: ToString,
    {
        Source {
            path: path.to_string(),
            chars,
            prev: '\0',
            cache: '\0',
            line: 1,
            offset: 0,
        }
    }

    pub fn path(&self) -> String {
        return self.path.to_string();
    }

    pub fn reset_cache(&mut self) {
        self.cache = '\0';
    }

    pub fn undo(&mut self) {
        self.cache = self.prev;
    }

    pub fn read_identi(&mut self) -> String {
        let mut value = String::from(self.prev);
        self.reset_cache();

        while let Some(ch) = self.next() {
            if is_identi(ch) {
                value.push(ch);
            } else {
                self.undo();
                break;
            }
        }

        return value;
    }

    pub fn read_number(&mut self) -> Value {
        fn convert(ch: char) -> f64 {
            const START_POINT: u8 = 48;
            return ((ch as u8) - START_POINT) as f64;
        }

        let mut value = convert(self.prev);
        self.reset_cache();
        let mut is_flt = false;

        while let Some(ch) = self.next() {
            if ch.is_ascii_digit() {
                let num = convert(ch);
                if is_flt {
                    value = value + (num / 10.0);
                } else {
                    value = value * 10.0 + num;
                }
            } else if ch == '.' {
                if !is_flt {
                    is_flt = true;
                    continue;
                } else {
                    err_report!(CompileErr::FloatError(Box::<Location>::new(
                        (self.path(), self.line, self.offset).into()
                    ))
                    .into())
                }
            } else {
                self.undo();
                break;
            }
        }

        return Value::Number(value);
    }

    pub fn read_string(&mut self) -> Result<Value, ErrorType> {
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

        while let Some(ch) = self.next() {
            if ch == '\'' || ch == '\"' {
                break;
            } else if ch == '\\' {
                value.push(convert(&mut self.chars));
            } else if ch == '\n' {
                self.line += 1;
                self.offset = 0;
                value.push(ch);
            } else {
                value.push(ch);
            }
        }

        if self.prev != '\'' && self.prev != '\"' {
            return Err(CompileErr::MissComma(Box::new(Location(
                self.path(),
                self.line,
                self.offset,
            )))
            .into());
        }

        return Ok(Value::String(value));
    }

    pub fn read_unknow(&mut self) -> String {
        let mut value = String::from(self.prev);

        while let Some(ch) = self.next() {
            if ch.is_whitespace() {
                break;
            }
            value.push(ch);
        }

        return value;
    }
}

impl Iterator for Source<'_> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cache != '\0' {
            self.prev = self.cache;
            self.reset_cache();
            return Some(self.prev);
        }

        self.offset += 1;

        match self.chars.next() {
            Some(ch) => {
                self.prev = ch;
                return Some(self.prev);
            }
            None => return None,
        }
    }
}
