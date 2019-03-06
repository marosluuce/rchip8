use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct SkipKeyNotPressed {
    register: usize,
}

impl Op for SkipKeyNotPressed {
    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}

impl Matcher for SkipKeyNotPressed {
    const MASK: u16 = 0xEFA1;

    fn new(opcode: u16) -> SkipKeyNotPressed {
        SkipKeyNotPressed {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }
}

impl fmt::Display for SkipKeyNotPressed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SKNP V{:X}", self.register)
    }
}
