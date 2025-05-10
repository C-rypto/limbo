use std::collections::VecDeque;

use crate::common::error::{CompileErr, ErrorType};

use super::{Token, TokenTag, TokenType};

#[derive(Clone)]
pub struct TokenStream {
    tokens: VecDeque<Token>,

    pub prev: Option<Token>,
}

impl TokenStream {
    pub fn new() -> Self {
        return TokenStream {
            tokens: VecDeque::new(),
            prev: None,
        };
    }

    fn push_front(&mut self, value: Token) {
        self.tokens.push_front(value);
    }

    pub fn push(&mut self, value: Token) {
        self.tokens.push_back(value);
    }

    pub fn prev(&self) -> Token {
        match &self.prev {
            Some(prev) => return prev.clone(),
            None => unreachable!(),
        }
    }

    pub fn expect(&mut self, target: TokenTag) -> Result<Token, ErrorType> {
        if let Some(next) = self.next() {
            let next_tag = next.get_tag();

            if next_tag != target && next_tag != TokenTag::WhiteSpace {
                return Err(CompileErr::Unexpected(Box::new(next)).into());
            } else if next_tag == TokenTag::WhiteSpace {
                let res = self.expect(target);
                return res;
            } else if next_tag == TokenTag::Unknown {
                return Err(CompileErr::UnknownTok(Box::new(next)).into());
            }
            return Ok(next);
        } else {
            return Err(CompileErr::IllegalEOF(Box::new(self.prev().get_end_pos())).into());
        }
    }

    pub fn undo(&mut self) {
        self.push_front(self.prev());
    }

    pub fn skip_white_space(&mut self) {
        while let Some(next) = self.next() {
            if let TokenType::WhiteSpace(..) | TokenType::EOL = next.token_type {
                continue;
            } else {
                self.undo();
                break;
            }
        }
    }
}

impl Iterator for TokenStream {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.tokens.pop_front();

        if let Some(token) = &next {
            self.prev = next.clone();
            if token.get_tag() == TokenTag::WhiteSpace {
                return self.next();
            }
        }

        return next;
    }
}
