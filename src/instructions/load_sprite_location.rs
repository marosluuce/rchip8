use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

pub struct LoadSpriteLocation {
    register: usize,
}

impl Instruction for LoadSpriteLocation {
    const MASK: u16 = 0xFF29;

    fn new(opcode: u16) -> LoadSpriteLocation {
        LoadSpriteLocation {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}

impl fmt::Display for LoadSpriteLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD F, V{:X}", self.register)
    }
}
