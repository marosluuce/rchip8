use cpu::Cpu;
use instructions::instruction::Instruction;
use instructions::op::Op;

struct JumpStack;

impl Op for JumpStack {
    const MASK: u16 = 0x00E0;
}

impl Instruction for JumpStack {
    fn new(_opcode: u16) -> JumpStack {
        JumpStack {}
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}
