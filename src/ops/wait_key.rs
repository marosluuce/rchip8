use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct WaitKey {
    register: usize,
}

impl Op for WaitKey {
    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}

impl Matcher for WaitKey {
    const MASK: u16 = 0xFF0A;

    fn new(opcode: u16) -> WaitKey {
        WaitKey {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }
}

impl fmt::Display for WaitKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD V{:X}, K", self.register)
    }
}
