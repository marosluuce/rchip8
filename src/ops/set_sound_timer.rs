use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct SetSoundTimer {
    register: usize,
}

impl Op for SetSoundTimer {
    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}

impl Matcher for SetSoundTimer {
    const MASK: u16 = 0x5FF0;

    fn new(opcode: u16) -> SetSoundTimer {
        SetSoundTimer {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }
}

impl fmt::Display for SetSoundTimer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LD ST, V{:X}", self.register)
    }
}
