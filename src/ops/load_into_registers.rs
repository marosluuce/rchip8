use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct LoadIntoRegisters {
    register: usize,
}

impl Op for LoadIntoRegisters {
    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}

impl Matcher for LoadIntoRegisters {
    const MASK: u16 = 0xFF65;

    fn new(opcode: u16) -> LoadIntoRegisters {
        LoadIntoRegisters {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }
}

impl fmt::Display for LoadIntoRegisters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD V{:X}, [I]", self.register)
    }
}
