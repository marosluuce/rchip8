use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct AddToInstructionPointer {
    register: usize,
}

impl Op for AddToInstructionPointer {
    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}

impl Matcher for AddToInstructionPointer {
    const MASK: u16 = 0x5FF0;

    fn new(opcode: u16) -> AddToInstructionPointer {
        AddToInstructionPointer {
            register: ((opcode & 0x0F00) >> 8) as usize,
        }
    }
}

impl fmt::Display for AddToInstructionPointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ADD I, V{:X}", self.register)
    }
}
