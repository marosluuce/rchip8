pub trait Op {
    const MASK: u16;

    fn matches(opcode: u16) -> bool {
        opcode & Self::MASK == opcode
    }
}
