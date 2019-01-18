use cpu::Cpu;
use instructions::instruction::Instruction;
use std::fmt;

pub struct Sprite {
    register1: usize,
    register2: usize,
    nibble: u8,
}

impl Instruction for Sprite {
    const MASK: u16 = 0xDFFF;

    fn new(opcode: u16) -> Sprite {
        Sprite {
            register1: ((opcode & 0x0F00) >> 8) as usize,
            register2: ((opcode & 0x00F0) >> 4) as usize,
            nibble: (opcode & 0x000F) as u8,
        }
    }

    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}

impl fmt::Display for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DRW V{:X}, V{:X}, {:X}",
            self.register1, self.register2, self.nibble
        )
    }
}
