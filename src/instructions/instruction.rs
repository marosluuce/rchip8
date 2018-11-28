use cpu::Cpu;

pub trait Instruction {
    fn new(opcode: u16) -> Self;
    fn execute(&self, cpu: Cpu) -> Cpu;
}
