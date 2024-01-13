pub enum Error {
    ParseInvalidInstruction(u16),
    OutOfBoundsMemoryAccess(u16),
    NotEnoughMemoryForProgram,
}
