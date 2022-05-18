use std::{
    io::{BufRead, Write},
    iter::Peekable,
    slice::Iter,
};

use crate::command::Command;

#[derive(Debug)]
pub struct Machine<'a, R, W>
where
    R: BufRead,
    W: Write,
{
    program: Peekable<Iter<'a, Command>>,
    memory: &'a mut [u8],
    pointer: &'a mut usize,
    input: &'a mut R,
    output: &'a mut W,
    last: Option<&'a Command>,
}

impl<'a, R, W> Machine<'a, R, W>
where
    R: BufRead,
    W: Write,
{
    pub fn new(
        program: &'a [Command],
        memory: &'a mut [u8],
        pointer: &'a mut usize,
        input: &'a mut R,
        output: &'a mut W,
    ) -> Self {
        Self {
            program: program.iter().peekable(),
            memory,
            pointer,
            input,
            output,
            last: None,
        }
    }
}

impl<'a, R, W> Iterator for Machine<'a, R, W>
where
    R: BufRead,
    W: Write,
{
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let command = self.program.next()?;
        match command {
            Command::MoveLeft(n) => *self.pointer -= n,
            Command::MoveRight(n) => *self.pointer += n,
            Command::Increment(n) => self.memory[*self.pointer] += *n,
            Command::Decrement(n) => self.memory[*self.pointer] -= *n,
            Command::Input => {
                let mut buffer = [0; 1];
                self.input.read_exact(&mut buffer).unwrap();
                self.memory[*self.pointer] = buffer[0];
            }
            Command::Output => {
                write!(self.output, "{}", char::from(self.memory[*self.pointer])).unwrap();
            }
            Command::Loop(block) => {
                while self.memory[*self.pointer] != 0 {
                    Machine::new(block, self.memory, self.pointer, self.input, self.output)
                        .for_each(drop);
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
                Command::MoveLeft(n) => *self.pointer <<= n,
                Command::MoveRight(n) => *self.pointer >>= n,
                Command::Increment(n) => self.memory[*self.pointer] <<= n,
                Command::Decrement(n) => self.memory[*self.pointer] >>= n,
                Command::Input => {
                    let mut buffer = String::new();
                    self.input.read_line(&mut buffer).unwrap();
                    self.memory[*self.pointer] = buffer.trim_end().parse().unwrap();
                }
                Command::Output => {
                    write!(self.output, "{}", self.memory[*self.pointer]).unwrap();
                }
                Command::Loop(block) => {
                    while self.memory[*self.pointer] != 0 {
                        Machine::new(
                            &block.iter().rev().cloned().collect::<Vec<_>>(),
                            self.memory,
                            self.pointer,
                            self.input,
                            self.output,
                        )
                        .for_each(drop);
                    }
                }
                Command::Dump => {
                    eprintln!(
                        "{pointer:#x} {memory:?} {next:?}",
                        pointer = self.pointer,
                        memory = self.memory,
                        next = self.program.peek(),
                    );
                }
                Command::Boop(_) => writeln!(self.output, "🫵").unwrap(),
            },
        }

        self.last = Some(command);
        Some(())
    }
}
