use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

struct ClearScreen;

impl Instruction for ClearScreen {
    const MASK: u16 = 0x00E0;

    fn new(_opcode: u16) -> ClearScreen {
        ClearScreen {}
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}

impl fmt::Display for ClearScreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CLS")
    }
}
