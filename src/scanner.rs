use std::str::Chars;

use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    chars: Chars<'a>,
}

impl<'a> Scanner<'a> {
    #[must_use]
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars(),
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.chars.next()? {
            '😾' => Token::MoveLeft,
            '😺' => Token::MoveRight,
            '😸' => Token::Increment,
            '😿' => Token::Decrement,
            '😼' => Token::Input,
            '😽' => Token::Output,
            '😻' => Token::Loop,
            '🙀' => Token::Break,
            '💩' => Token::Dump,
            '👉' => Token::Boop,
            _ => return self.next(),
        })
    }
}
