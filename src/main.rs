#![warn(clippy::pedantic)]

use std::{env, fs, io, time::Instant};

use anyhow::{Context, Result};
use boop::{Machine, Parser, Scanner};

fn main() -> Result<()> {
    let path = env::args().nth(1).context("missing path argument")?;
    let source = fs::read_to_string(path)?;
    let scanner = Scanner::new(&source);
    let tokens = scanner.collect::<Vec<_>>();
    let parser = Parser::new(&tokens);
    let program = parser.collect::<Result<Vec<_>, _>>()?;
    let mut memory = [0; 30_000];
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let stdout = io::stdout();
    let mut output = stdout.lock();
    let stderr = io::stderr();
    let mut debug = stderr.lock();
    let mut machine = Machine::new(&mut memory, &mut input, &mut output, &mut debug);
    let now = Instant::now();
    machine.execute(&program)?;
    println!("{}s", now.elapsed().as_secs_f64());

    Ok(())
}
