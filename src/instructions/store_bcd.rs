use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

pub struct StoreBCD {
    register: usize,
}

impl Instruction for StoreBCD {
    const MASK: u16 = 0xFF33;

    fn new(opcode: u16) -> StoreBCD {
        StoreBCD {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}

impl fmt::Display for StoreBCD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD B, V{:X}", self.register)
    }
}
