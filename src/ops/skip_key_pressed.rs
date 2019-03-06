use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct SkipKeyPressed {
    register: usize,
}

impl Op for SkipKeyPressed {
    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}

impl Matcher for SkipKeyPressed {
    const MASK: u16 = 0xEF9E;

    fn new(opcode: u16) -> SkipKeyPressed {
        SkipKeyPressed {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }
}

impl fmt::Display for SkipKeyPressed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SKP V{:X}", self.register)
    }
}
