use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct StoreBCD {
    register: usize,
}

impl Op for StoreBCD {
    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}

impl Matcher for StoreBCD {
    const MASK: u16 = 0xFF33;

    fn new(opcode: u16) -> StoreBCD {
        StoreBCD {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }
}

impl fmt::Display for StoreBCD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD B, V{:X}", self.register)
    }
}
