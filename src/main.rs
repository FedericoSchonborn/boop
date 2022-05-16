#![warn(clippy::pedantic)]

use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

use anyhow::{anyhow, Result};
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
    // Actual value of the cell.
    value: u8,
    // Count of times the cell was booped.
    boops: usize,
}

fn eval(program: Vec<Command>, memory: &mut [Cell]) -> Result<()> {
    let mut index = 0_usize;

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    for command in program {
        match command {
            MoveRight => {
                index = index.wrapping_add(1);
            }
            MoveLeft => {
                index = index.wrapping_sub(1);
            }
            Increment => {
                memory[index].value = memory[index].value.wrapping_add(1);
            }
            Decrement => {
                memory[index].value = memory[index].value.wrapping_sub(1);
            }
            Output => {
                println!("{}", char::from(memory[index].value));
            }
            Input => {
                let mut input = [0_u8; 1];
                stdin.read_exact(&mut input)?;

                memory[index].value = input[0];
            }
            OpenLoop | CloseLoop => todo!(),
            Boop => {
                memory[index].boops = memory[index]
                    .boops
                    .checked_add(1)
                    .ok_or_else(|| anyhow!("Cell {} was booped too much!", index))?;
            }
        }
    }

    Ok(())
}

#[derive(Debug, Parser)]
struct Options {
    /// Memory cell size.
    #[clap(short, long)]
    memory_size: Option<usize>,

    path: PathBuf,
}

fn main() -> Result<()> {
    let options = Options::parse();
    let source = fs::read_to_string(options.path)?;
    let program = read(&source);
    println!("{program:?}");
    let mut memory = vec![Cell::default(); options.memory_size.unwrap_or(30_000)];
    eval(program, &mut memory)?;
    println!("{memory:?}");

    Ok(())
}
