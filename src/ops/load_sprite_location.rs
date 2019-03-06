use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct LoadSpriteLocation {
    register: usize,
}

impl Op for LoadSpriteLocation {
    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}

impl Matcher for LoadSpriteLocation {
    const MASK: u16 = 0xFF29;

    fn new(opcode: u16) -> LoadSpriteLocation {
        LoadSpriteLocation {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }
}

impl fmt::Display for LoadSpriteLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD F, V{:X}", self.register)
    }
}
