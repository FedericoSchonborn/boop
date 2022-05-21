use std::io::{BufRead, Write};

use crate::command::Command;

#[derive(Debug)]
pub struct Machine<'a, R, W, D>
where
    R: BufRead,
    W: Write,
    D: Write,
{
    memory: &'a mut [u8],
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
        memory: &'a mut [u8],
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

    #[allow(clippy::missing_panics_doc)]
    pub fn execute(&mut self, program: &'a [Command]) {
        let mut iter = program.iter().peekable();
        while let Some(command) = iter.next() {
            match command {
                Command::MovePointerLeft(n) => self.pointer -= n,
                Command::MovePointerRight(n) => self.pointer += n,
                Command::IncrementCell(n) => self.memory[self.pointer] += *n,
                Command::DecrementCell(n) => self.memory[self.pointer] -= *n,
                Command::InputChar => {
                    let mut buffer = [0; 1];
                    self.input.read_exact(&mut buffer).unwrap();
                    self.memory[self.pointer] = buffer[0];
                }
                Command::OutputChar => {
                    write!(self.output, "{}", char::from(self.memory[self.pointer])).unwrap();
                }
                Command::NonZeroLoop(block) => {
                    while self.memory[self.pointer] != 0 {
                        self.execute(block);
                    }
                }
                Command::DumpPast => writeln!(
                    self.debug,
                    "{pointer:#x} {memory:?} {last:?}",
                    pointer = self.pointer,
                    memory = self.memory,
                    last = self.last,
                )
                .unwrap(),
                Command::ShiftPointerLeft(n) => self.pointer <<= n,
                Command::ShiftPointerRight(n) => self.pointer >>= n,
                Command::ShiftCellLeft(n) => self.memory[self.pointer] <<= n,
                Command::ShiftCellRight(n) => self.memory[self.pointer] >>= n,
                Command::InputInt => {
                    let mut buffer = String::new();
                    self.input.read_line(&mut buffer).unwrap();
                    self.memory[self.pointer] = buffer.trim_end().parse().unwrap();
                }
                Command::OutputInt => write!(self.output, "{}", self.memory[self.pointer]).unwrap(),
                Command::ZeroLoop(block) => {
                    while self.memory[self.pointer] == 0 {
                        self.execute(block);
                    }
                }
                Command::DumpFuture => writeln!(
                    self.debug,
                    "{pointer:#x} {memory:?} {next:?}",
                    pointer = self.pointer,
                    memory = self.memory,
                    next = iter.peek(),
                )
                .unwrap(),
                Command::Boop => writeln!(self.debug, "🫵").unwrap(),
            }

            self.last = Some(command);
        }
    }
}
