#![warn(clippy::pedantic)]

use std::{env, fs, io};

use boop::{Machine, Parser, Scanner};

fn main() {
    let path = env::args().nth(1).unwrap();
    let source = fs::read_to_string(path).unwrap();
    let scanner = Scanner::new(&source);
    let tokens = scanner.collect::<Vec<_>>();
    let parser = Parser::new(&tokens);
    let program = parser.collect::<Vec<_>>();
    let mut memory = [0; 7];
    let mut pointer = 0;
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let stdout = io::stdout();
    let mut output = stdout.lock();
    let machine = Machine::new(&program, &mut memory, &mut pointer, &mut input, &mut output);
    machine.for_each(drop);
}
