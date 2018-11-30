use cpu::Cpu;
use instructions::instruction::Instruction;
use instructions::op::Op;

struct LoadIntoRegisters {}

impl Op for LoadIntoRegisters {
    const MASK: u16 = 0xFF65;
}

impl Instruction for LoadIntoRegisters {
    fn new(opcode: u16) -> LoadIntoRegisters {
        LoadIntoRegisters {}
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        Cpu {
            pc: cpu.pc + 1,
            ..cpu
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
