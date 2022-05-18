#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Command {
    MoveLeft(usize),
    MoveRight(usize),
    Increment(u8),
    Decrement(u8),
    Input,
    Output,
    Loop(Vec<Command>),
    Dump,
    Boop(Option<Box<Command>>),
}
