#![warn(clippy::pedantic)]

use std::{
    fmt::{self, Display, Formatter},
    fs,
    io::{self, Read},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;

#[derive(Debug)]
enum Command {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    OpenLoop,
    CloseLoop,
    Boop,
}

use Command::{
    Boop, CloseLoop, Decrement, Increment, Input, MoveLeft, MoveRight, OpenLoop, Output,
};

fn read(source: &str) -> Vec<Command> {
    let mut program = Vec::new();

    for slice in source.split_whitespace() {
        program.push({
            match slice {
                "👉👉" => MoveRight,
                "👉👈" => MoveLeft,
                "👉😺" => Increment,
                "👉😾" => Decrement,
                "👉😻" => Output,
                "👉🐱" => Input,
                "👉😸" => OpenLoop,
                "👉🙀" => CloseLoop,
                "👉🐈" => Boop,
                _ => continue,
            }
        });
    }

    program
}

#[derive(Debug, Clone, Copy, Default)]
struct Cell {
    /// Actual value of this cell.
    value: u8,
    /// Count of times this cell was booped.
    boops: usize,
}

impl Cell {
    /// Set the value of this cell.
    fn set(&mut self, value: u8) {
        self.value = value;
    }

    /// Increment the value of this cell by 1.
    fn increment(&mut self) {
        self.value = self.value.wrapping_add(1);
    }

    /// Decrement the value of this cell by 1.
    fn decrement(&mut self) {
        self.value = self.value.wrapping_sub(1);
    }

    /// Add 1 to the boop count of this cell.
    fn boop(&mut self) {
        self.boops = self.boops.saturating_add(1);
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        char::from(self.value).fmt(f)
    }
}

fn eval(program: &[Command], memory: &mut [Cell]) -> Result<()> {
    let mut pointer = 0_usize;

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    for command in program {
        println!("{command:?}");

        match command {
            MoveRight => {
                pointer = pointer.wrapping_add(1);
            }
            MoveLeft => {
                pointer = pointer.wrapping_sub(1);
            }
            Increment => {
                memory[pointer].increment();
            }
            Decrement => {
                memory[pointer].decrement();
            }
            Output => {
                print!("{}", memory[pointer]);
            }
            Input => {
                let mut input = [0_u8; 1];
                stdin.read_exact(&mut input)?;

                memory[pointer].set(input[0]);
            }
            OpenLoop | CloseLoop => todo!(),
            Boop => {
                memory[pointer].boop();
            }
        }
    }

    Ok(())
}

#[derive(Debug, Parser)]
struct Options {
    /// Memory cell size.
    #[clap(short, long, default_value_t = 30_000)]
    memory_size: usize,

    path: PathBuf,
}

fn main() -> Result<()> {
    let options = Options::parse();
    let source = fs::read_to_string(options.path)?;

    let program = read(&source);
    println!("{program:?}");

    let mut memory = vec![Cell::default(); options.memory_size];
    eval(&program, &mut memory)?;
    println!("{memory:?}");

    Ok(())
}
