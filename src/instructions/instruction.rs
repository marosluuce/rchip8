use cpu::Cpu;

pub trait Instruction {
    const MASK: u16;

    fn matches(opcode: u16) -> bool {
        opcode & Self::MASK == opcode
    }

    fn new(opcode: u16) -> Self;
    fn execute(&self, cpu: Cpu) -> Cpu;
}
