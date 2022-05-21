#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Command {
    // Regular commands.
    MovePointerLeft,
    MovePointerRight,
    IncrementCell,
    DecrementCell,
    InputChar,
    OutputChar,
    NonZeroLoop(Vec<Command>),
    DumpPast,
    // Booped commands.
    ShiftPointerLeft,
    ShiftPointerRight,
    ShiftCellLeft,
    ShiftCellRight,
    InputInt,
    OutputInt,
    ZeroLoop(Vec<Command>),
    DumpFuture,
    Boop,
}
