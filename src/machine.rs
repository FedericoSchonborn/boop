use std::io::{BufRead, Write};

use crate::command::Command;

#[derive(Debug)]
pub struct Machine<'a, R, W>
where
    R: BufRead,
    W: Write,
{
    memory: &'a mut [u8],
    pointer: usize,
    input: &'a mut R,
    output: &'a mut W,
    last: Option<&'a Command>,
}

impl<'a, R, W> Machine<'a, R, W>
where
    R: BufRead,
    W: Write,
{
    pub fn new(memory: &'a mut [u8], input: &'a mut R, output: &'a mut W) -> Self {
        Self {
            memory,
            pointer: 0,
            input,
            output,
            last: None,
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn execute(&mut self, program: &'a [Command]) {
        let mut iter = program.iter().peekable();
        while let Some(command) = iter.next() {
            match command {
                Command::MoveLeft(n) => self.pointer -= n,
                Command::MoveRight(n) => self.pointer += n,
                Command::Increment(n) => self.memory[self.pointer] += *n,
                Command::Decrement(n) => self.memory[self.pointer] -= *n,
                Command::Input => {
                    let mut buffer = [0; 1];
                    self.input.read_exact(&mut buffer).unwrap();
                    self.memory[self.pointer] = buffer[0];
                }
                Command::Output => {
                    write!(self.output, "{}", char::from(self.memory[self.pointer])).unwrap();
                }
                Command::Loop(block) => {
                    while self.memory[self.pointer] != 0 {
                        self.execute(block);
                    }
                }
                Command::Dump => {
                    eprintln!(
                        "{pointer:#x} {memory:?} {last:?}",
                        pointer = self.pointer,
                        memory = self.memory,
                        last = self.last,
                    );
                }
                Command::Boop(inner) => match &**inner.as_ref().expect("unmatched boop") {
                    Command::MoveLeft(n) => self.pointer <<= n,
                    Command::MoveRight(n) => self.pointer >>= n,
                    Command::Increment(n) => self.memory[self.pointer] <<= n,
                    Command::Decrement(n) => self.memory[self.pointer] >>= n,
                    Command::Input => {
                        let mut buffer = String::new();
                        self.input.read_line(&mut buffer).unwrap();
                        self.memory[self.pointer] = buffer.trim_end().parse().unwrap();
                    }
                    Command::Output => {
                        write!(self.output, "{}", self.memory[self.pointer]).unwrap();
                    }
                    Command::Loop(block) => {
                        while self.memory[self.pointer] == 0 {
                            self.execute(block);
                        }
                    }
                    Command::Dump => {
                        eprintln!(
                            "{pointer:#x} {memory:?} {next:?}",
                            pointer = self.pointer,
                            memory = self.memory,
                            next = iter.peek(),
                        );
                    }
                    Command::Boop(_) => writeln!(self.output, "🫵").unwrap(),
                },
            }

            self.last = Some(command);
        }
    }
}
