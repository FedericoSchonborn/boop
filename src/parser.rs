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

    #[allow(clippy::too_many_lines)] // Shhhh.
    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.tokens.next()? {
            Token::Left => {
                let mut count = 1;
                while let Some(Token::Left) = self.tokens.peek() {
                    count += 1;
                    self.tokens.next();
                }

                Command::MovePointerLeft(count)
            }
            Token::Right => {
                let mut count = 1;
                while let Some(Token::Right) = self.tokens.peek() {
                    count += 1;
                    self.tokens.next();
                }

                Command::MovePointerRight(count)
            }
            Token::Plus => {
                let mut count = 1;
                while let Some(Token::Plus) = self.tokens.peek() {
                    count += 1;
                    self.tokens.next();
                }

                Command::IncrementCell(count)
            }
            Token::Minus => {
                let mut count = 1;
                while let Some(Token::Minus) = self.tokens.peek() {
                    count += 1;
                    self.tokens.next();
                }

                Command::DecrementCell(count)
            }
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

                Command::NonZeroLoop(Parser::new(&tokens).collect())
            }
            Token::Break => panic!("unmatched break"),
            Token::Dump => Command::DumpPast,
            Token::Boop => match self.tokens.next().expect("unmatched loop") {
                Token::Left => {
                    let mut count = 1;
                    while let Some(Token::Left) = self.tokens.peek() {
                        count += 1;
                        self.tokens.next();
                    }

                    Command::ShiftPointerLeft(count)
                }
                Token::Right => {
                    let mut count = 1;
                    while let Some(Token::Right) = self.tokens.peek() {
                        count += 1;
                        self.tokens.next();
                    }

                    Command::ShiftPointerRight(count)
                }
                Token::Plus => {
                    let mut count = 1;
                    while let Some(Token::Plus) = self.tokens.peek() {
                        count += 1;
                        self.tokens.next();
                    }

                    Command::ShiftCellLeft(count)
                }
                Token::Minus => {
                    let mut count = 1;
                    while let Some(Token::Minus) = self.tokens.peek() {
                        count += 1;
                        self.tokens.next();
                    }

                    Command::ShiftCellRight(count)
                }
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

                    Command::ZeroLoop(Parser::new(&tokens).collect())
                }
                Token::Break => panic!("unmatched break"),
                Token::Dump => Command::DumpFuture,
                Token::Boop => Command::Boop,
            },
        })
    }
}
