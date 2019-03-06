use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct SetDelayTimer {
    register: usize,
}

impl Op for SetDelayTimer {
    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}

impl Matcher for SetDelayTimer {
    const MASK: u16 = 0xFF15;

    fn new(opcode: u16) -> SetDelayTimer {
        SetDelayTimer {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }
}

impl fmt::Display for SetDelayTimer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD DT, V{:X}", self.register)
    }
}
