use std::{iter::Peekable, slice::Iter};

use thiserror::Error;

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

    fn parse(&mut self, token: Token) -> Result<Command, Error> {
        Ok(match token {
            Token::Left => Command::MovePointerLeft,
            Token::Right => Command::MovePointerRight,
            Token::Plus => Command::IncrementCell,
            Token::Minus => Command::DecrementCell,
            Token::Input => Command::InputChar,
            Token::Output => Command::OutputChar,
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

                Command::NonZeroLoop(Parser::new(&tokens).collect::<Result<Vec<_>, _>>()?)
            }
            Token::Break => return Err(Error::UnmatchedBreak),
            Token::Dump => Command::DumpPast,
            Token::Boop => match self.tokens.next().ok_or(Error::UnmatchedBoop)? {
                Token::Left => Command::ShiftPointerLeft,
                Token::Right => Command::ShiftPointerRight,
                Token::Plus => Command::ShiftCellLeft,
                Token::Minus => Command::ShiftCellRight,
                Token::Input => Command::InputInt,
                Token::Output => Command::OutputInt,
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

                    Command::ZeroLoop(Parser::new(&tokens).collect::<Result<Vec<_>, _>>()?)
                }
                Token::Break => return Err(Error::UnmatchedBreak),
                Token::Dump => Command::DumpFuture,
                Token::Boop => Command::Boop,
            },
        })
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Command, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.tokens.next()?;
        Some(self.parse(*token))
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("unmatched break")]
    UnmatchedBreak,
    #[error("unmatched boop")]
    UnmatchedBoop,
}
