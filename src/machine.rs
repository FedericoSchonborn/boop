use std::{
    char,
    io::{self, BufRead, Write},
    num::ParseIntError,
};

use thiserror::Error;

use crate::command::Command;

#[derive(Debug)]
pub struct Machine<'a, R, W, D>
where
    R: BufRead,
    W: Write,
    D: Write,
{
    memory: &'a mut [u32],
    pointer: usize,
    input: &'a mut R,
    output: &'a mut W,
    debug: &'a mut D,
    last: Option<&'a Command>,
}

impl<'a, R, W, D> Machine<'a, R, W, D>
where
    R: BufRead,
    W: Write,
    D: Write,
{
    pub fn new(
        memory: &'a mut [u32],
        input: &'a mut R,
        output: &'a mut W,
        debug: &'a mut D,
    ) -> Self {
        Self {
            memory,
            pointer: 0,
            input,
            output,
            debug,
            last: None,
        }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn execute(&mut self, program: &'a [Command]) -> Result<(), Error> {
        let mut iter = program.iter().peekable();
        while let Some(command) = iter.next() {
            match command {
                Command::MovePointerLeft => self.pointer -= 1,
                Command::MovePointerRight => self.pointer += 1,
                Command::IncrementCell => self.memory[self.pointer] += 1,
                Command::DecrementCell => self.memory[self.pointer] -= 1,
                Command::InputChar => {
                    let mut buffer = String::new();
                    self.input.read_line(&mut buffer)?;
                    self.memory[self.pointer] =
                        buffer.trim_end().chars().next().ok_or(Error::EmptyInput)? as u32;
                }
                Command::OutputChar => {
                    write!(
                        self.output,
                        "{}",
                        char::from_u32(self.memory[self.pointer])
                            .unwrap_or(char::REPLACEMENT_CHARACTER)
                    )?;
                }
                Command::NonZeroLoop(block) => {
                    while self.memory[self.pointer] != 0 {
                        self.execute(block)?;
                    }
                }
                Command::DumpPast => writeln!(
                    self.debug,
                    "{pointer:#x} {memory:?} {last:?}",
                    pointer = self.pointer,
                    memory = self.memory,
                    last = self.last,
                )?,
                Command::ShiftPointerLeft => self.pointer <<= 1,
                Command::ShiftPointerRight => self.pointer >>= 1,
                Command::ShiftCellLeft => self.memory[self.pointer] <<= 1,
                Command::ShiftCellRight => self.memory[self.pointer] >>= 1,
                Command::InputInt => {
                    let mut buffer = String::new();
                    self.input.read_line(&mut buffer)?;
                    self.memory[self.pointer] = buffer.trim_end().parse()?;
                }
                Command::OutputInt => write!(self.output, "{}", self.memory[self.pointer])?,
                Command::ZeroLoop(block) => {
                    while self.memory[self.pointer] == 0 {
                        self.execute(block)?;
                    }
                }
                Command::DumpFuture => writeln!(
                    self.debug,
                    "{pointer:#x} {memory:?} {next:?}",
                    pointer = self.pointer,
                    memory = self.memory,
                    next = iter.peek(),
                )?,
                Command::Boop => writeln!(self.debug, "🫵")?,
            }

            self.last = Some(command);
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("empty input")]
    EmptyInput,
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}
