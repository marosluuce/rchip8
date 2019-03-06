use cpu::Cpu;
use ops::op::{Matcher, Op};
use std::fmt;

pub struct ClearScreen;

impl Op for ClearScreen {
    fn execute(&self, cpu: Cpu) -> Cpu {
        cpu
    }
}

impl Matcher for ClearScreen {
    const MASK: u16 = 0x00E0;

    fn new(_opcode: u16) -> ClearScreen {
        ClearScreen {}
    }
}

impl fmt::Display for ClearScreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CLS")
    }
}
