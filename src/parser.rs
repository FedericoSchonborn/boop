use std::{iter::Peekable, slice::Iter};

use crate::{command::Command, token::Token};

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    #[must_use]
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens: tokens.iter().peekable(),
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.tokens.next()? {
            Token::MoveLeft => {
                let mut count = 1;
                while let Some(Token::MoveLeft) = self.tokens.peek() {
                    count += 1;
                    self.tokens.next();
                }

                Command::MoveLeft(count)
            }
            Token::MoveRight => {
                let mut count = 1;
                while let Some(Token::MoveRight) = self.tokens.peek() {
                    count += 1;
                    self.tokens.next();
                }

                Command::MoveRight(count)
            }
            Token::Increment => {
                let mut count = 1;
                while let Some(Token::Increment) = self.tokens.peek() {
                    count += 1;
                    self.tokens.next();
                }

                Command::Increment(count)
            }
            Token::Decrement => {
                let mut count = 1;
                while let Some(Token::Decrement) = self.tokens.peek() {
                    count += 1;
                    self.tokens.next();
                }

                Command::Decrement(count)
            }
            Token::Input => Command::Input,
            Token::Output => Command::Output,
            Token::Loop => {
                let mut tokens = Vec::new();
                let mut count = 1;
                while let Some(token) = self.tokens.peek().copied() {
                    match token {
                        Token::Loop => {
                            count += 1;
                        }
                        Token::Break => {
                            count -= 1;
                        }
                        _ => {}
                    }

                    self.tokens.next();
                    if count == 0 {
                        break;
                    }

                    tokens.push(*token);
                }

                Command::Loop(Parser::new(&tokens).collect())
            }
            Token::Break => panic!("unmatched break"),
            Token::Dump => Command::Dump,
            Token::Boop => Command::Boop(self.next().map(Box::new)),
        })
    }
}
