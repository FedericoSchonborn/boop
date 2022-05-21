#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token {
    Left,
    Right,
    Plus,
    Minus,
    Input,
    Output,
    Loop,
    Break,
    Dump,
    Boop,
}
