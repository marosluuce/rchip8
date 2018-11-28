use cpu::Cpu;
use instructions::instruction::Instruction;
use instructions::op::Op;

struct ClearScreen;

impl Op for ClearScreen {
    const MASK: u16 = 0x00E0;
}

impl Instruction for ClearScreen {
    fn new(_opcode: u16) -> ClearScreen {
        ClearScreen {}
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}
