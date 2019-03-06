use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct StoreRegisters {
    register: usize,
}

impl Op for StoreRegisters {
    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}

impl Matcher for StoreRegisters {
    const MASK: u16 = 0xFF55;

    fn new(opcode: u16) -> StoreRegisters {
        StoreRegisters {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }
}

impl fmt::Display for StoreRegisters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD [I], V{:X}", self.register)
    }
}
