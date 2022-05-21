#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Command {
    // Regular commands.
    MovePointerLeft(usize),
    MovePointerRight(usize),
    IncrementCell(u8),
    DecrementCell(u8),
    InputChar,
    OutputChar,
    NonZeroLoop(Vec<Command>),
    DumpPast,
    // Booped commands.
    ShiftPointerLeft(usize),
    ShiftPointerRight(usize),
    ShiftCellLeft(usize),
    ShiftCellRight(usize),
    InputInt,
    OutputInt,
    ZeroLoop(Vec<Command>),
    DumpFuture,
    Boop,
}
