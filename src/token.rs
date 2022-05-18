#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    Input,
    Output,
    Loop,
    Break,
    Dump,
    Boop,
}
